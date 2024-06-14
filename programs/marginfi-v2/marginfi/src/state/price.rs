use std::cmp::min;

use anchor_lang::prelude::*;

use enum_dispatch::enum_dispatch;
use fixed::types::I80F48;
use pyth_sdk_solana::{load_price_feed_from_account_info, Price, PriceFeed};
use switchboard_v2::{
    AggregatorAccountData, AggregatorResolutionMode, SwitchboardDecimal, SWITCHBOARD_PROGRAM_ID,
};

use crate::{
    check,
    constants::{
        CONF_INTERVAL_MULTIPLE, EXP_10, EXP_10_I80F48, MAX_CONF_INTERVAL, PYTH_ID, STD_DEV_MULTIPLE,
    },
    debug, math_error,
    prelude::*,
};

use super::marginfi_group::BankConfig;

#[repr(u8)]
#[cfg_attr(any(feature = "test", feature = "client"), derive(PartialEq, Eq))]
#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum OracleSetup {
    None,
    PythEma,
    SwitchboardV2,
}

#[derive(Copy, Clone, Debug)]
pub enum PriceBias {
    Low,
    High,
}

#[derive(Copy, Clone, Debug)]
pub enum OraclePriceType {
    /// Time weighted price
    /// EMA for PythEma
    TimeWeighted,
    /// Real time price
    RealTime,
}

#[enum_dispatch]
pub trait PriceAdapter {
    fn get_price_of_type(
        &self,
        oracle_price_type: OraclePriceType,
        bias: Option<PriceBias>,
    ) -> MarginfiResult<I80F48>;
}

#[enum_dispatch(PriceAdapter)]
#[cfg_attr(feature = "client", derive(Clone))]
pub enum OraclePriceFeedAdapter {
    PythEma(PythEmaPriceFeed),
    SwitchboardV2(SwitchboardV2PriceFeed),
}

impl OraclePriceFeedAdapter {
    pub fn try_from_bank_config(
        bank_config: &BankConfig,
        ais: &[AccountInfo],
        current_timestamp: i64,
    ) -> MarginfiResult<Self> {
        Self::try_from_bank_config_with_max_age(
            bank_config,
            ais,
            current_timestamp,
            bank_config.get_oracle_max_age(),
        )
    }

    pub fn try_from_bank_config_with_max_age(
        bank_config: &BankConfig,
        ais: &[AccountInfo],
        current_timestamp: i64,
        max_age: u64,
    ) -> MarginfiResult<Self> {
        debug!("Max age: {}", max_age);
        match bank_config.oracle_setup {
            OracleSetup::None => Err(MarginfiError::OracleNotSetup.into()),
            OracleSetup::PythEma => {
                check!(ais.len() == 1, MarginfiError::InvalidOracleAccount);
                check!(
                    ais[0].key == &bank_config.oracle_keys[0],
                    MarginfiError::InvalidOracleAccount
                );

                let account_info = &ais[0];

                Ok(OraclePriceFeedAdapter::PythEma(
                    PythEmaPriceFeed::load_checked(account_info, current_timestamp, max_age)?,
                ))
            }
            OracleSetup::SwitchboardV2 => {
                check!(ais.len() == 1, MarginfiError::InvalidOracleAccount);
                check!(
                    ais[0].key == &bank_config.oracle_keys[0],
                    MarginfiError::InvalidOracleAccount
                );

                Ok(OraclePriceFeedAdapter::SwitchboardV2(
                    SwitchboardV2PriceFeed::load_checked(&ais[0], current_timestamp, max_age)?,
                ))
            }
        }
    }

    pub fn validate_bank_config(
        bank_config: &BankConfig,
        oracle_ais: &[AccountInfo],
    ) -> MarginfiResult {
        match bank_config.oracle_setup {
            OracleSetup::None => Err(MarginfiError::OracleNotSetup.into()),
            OracleSetup::PythEma => {
                check!(oracle_ais.len() == 1, MarginfiError::InvalidOracleAccount);
                check!(
                    oracle_ais[0].key == &bank_config.oracle_keys[0],
                    MarginfiError::InvalidOracleAccount
                );

                PythEmaPriceFeed::check_ais(&oracle_ais[0])?;

                Ok(())
            }
            OracleSetup::SwitchboardV2 => {
                check!(oracle_ais.len() == 1, MarginfiError::InvalidOracleAccount);
                check!(
                    oracle_ais[0].key == &bank_config.oracle_keys[0],
                    MarginfiError::InvalidOracleAccount
                );

                SwitchboardV2PriceFeed::check_ais(&oracle_ais[0])?;

                Ok(())
            }
        }
    }
}

#[cfg_attr(feature = "client", derive(Clone, Debug))]
pub struct PythEmaPriceFeed {
    ema_price: Box<Price>,
    price: Box<Price>,
}

impl PythEmaPriceFeed {
    pub fn load_checked(ai: &AccountInfo, current_time: i64, max_age: u64) -> MarginfiResult<Self> {
        let price_feed = load_pyth_price_feed(ai)?;
        let ema_price = price_feed
            .get_ema_price_no_older_than(current_time, max_age)
            .ok_or(MarginfiError::StaleOracle)?;

        let price = price_feed
            .get_price_no_older_than(current_time, max_age)
            .ok_or(MarginfiError::StaleOracle)?;

        Ok(Self {
            ema_price: Box::new(ema_price),
            price: Box::new(price),
        })
    }

    fn check_ais(ai: &AccountInfo) -> MarginfiResult {
        load_pyth_price_feed(ai)?;
        Ok(())
    }

    fn get_confidence_interval(&self, use_ema: bool) -> MarginfiResult<I80F48> {
        let price = if use_ema {
            &self.ema_price
        } else {
            &self.price
        };

        let conf_interval =
            pyth_price_components_to_i80f48(I80F48::from_num(price.conf), price.expo)?
                .checked_mul(CONF_INTERVAL_MULTIPLE)
                .ok_or_else(math_error!())?;

        // Cap confidence interval to 5% of price
        let price = pyth_price_components_to_i80f48(I80F48::from_num(price.price), price.expo)?;

        let max_conf_interval = price
            .checked_mul(MAX_CONF_INTERVAL)
            .ok_or_else(math_error!())?;

        assert!(
            max_conf_interval >= I80F48::ZERO,
            "Negative max confidence interval"
        );

        assert!(
            conf_interval >= I80F48::ZERO,
            "Negative confidence interval"
        );

        Ok(min(conf_interval, max_conf_interval))
    }

    #[inline(always)]
    fn get_ema_price(&self) -> MarginfiResult<I80F48> {
        pyth_price_components_to_i80f48(I80F48::from_num(self.ema_price.price), self.ema_price.expo)
    }

    #[inline(always)]
    fn get_unweighted_price(&self) -> MarginfiResult<I80F48> {
        pyth_price_components_to_i80f48(I80F48::from_num(self.price.price), self.price.expo)
    }
}

impl PriceAdapter for PythEmaPriceFeed {
    fn get_price_of_type(
        &self,
        price_type: OraclePriceType,
        bias: Option<PriceBias>,
    ) -> MarginfiResult<I80F48> {
        let price = match price_type {
            OraclePriceType::TimeWeighted => self.get_ema_price()?,
            OraclePriceType::RealTime => self.get_unweighted_price()?,
        };

        match bias {
            None => Ok(price),
            Some(price_bias) => {
                let confidence_interval = self
                    .get_confidence_interval(matches!(price_type, OraclePriceType::TimeWeighted))?;

                match price_bias {
                    PriceBias::Low => Ok(price
                        .checked_sub(confidence_interval)
                        .ok_or_else(math_error!())?),
                    PriceBias::High => Ok(price
                        .checked_add(confidence_interval)
                        .ok_or_else(math_error!())?),
                }
            }
        }
    }
}

#[cfg_attr(feature = "client", derive(Clone, Debug))]
pub struct SwitchboardV2PriceFeed {
    aggregator_account: Box<LiteAggregatorAccountData>,
}

impl SwitchboardV2PriceFeed {
    pub fn load_checked(
        ai: &AccountInfo,
        current_timestamp: i64,
        max_age: u64,
    ) -> MarginfiResult<Self> {
        let ai_data = ai.data.borrow();

        check!(
            ai.owner.eq(&SWITCHBOARD_PROGRAM_ID),
            MarginfiError::InvalidOracleAccount
        );

        let aggregator_account = AggregatorAccountData::new_from_bytes(&ai_data)
            .map_err(|_| MarginfiError::InvalidOracleAccount)?;

        aggregator_account
            .check_staleness(current_timestamp, max_age as i64)
            .map_err(|_| MarginfiError::StaleOracle)?;

        Ok(Self {
            aggregator_account: Box::new(aggregator_account.into()),
        })
    }

    fn check_ais(ai: &AccountInfo) -> MarginfiResult {
        let ai_data = ai.data.borrow();

        check!(
            ai.owner.eq(&SWITCHBOARD_PROGRAM_ID),
            MarginfiError::InvalidOracleAccount
        );

        AggregatorAccountData::new_from_bytes(&ai_data)
            .map_err(|_| MarginfiError::InvalidOracleAccount)?;

        Ok(())
    }

    fn get_price(&self) -> MarginfiResult<I80F48> {
        let sw_decimal = self
            .aggregator_account
            .get_result()
            .map_err(|_| MarginfiError::InvalidPrice)?;

        Ok(swithcboard_decimal_to_i80f48(sw_decimal)
            .ok_or(MarginfiError::InvalidSwitchboardDecimalConversion)?)
    }

    fn get_confidence_interval(&self) -> MarginfiResult<I80F48> {
        let std_div = self.aggregator_account.latest_confirmed_round_std_deviation;
        let std_div = swithcboard_decimal_to_i80f48(std_div)
            .ok_or(MarginfiError::InvalidSwitchboardDecimalConversion)?;

        let conf_interval = std_div
            .checked_mul(STD_DEV_MULTIPLE)
            .ok_or_else(math_error!())?;

        let price = self.get_price()?;

        let max_conf_interval = price
            .checked_mul(MAX_CONF_INTERVAL)
            .ok_or_else(math_error!())?;

        assert!(
            max_conf_interval >= I80F48::ZERO,
            "Negative max confidence interval"
        );

        assert!(
            conf_interval >= I80F48::ZERO,
            "Negative confidence interval"
        );

        Ok(min(conf_interval, max_conf_interval))
    }
}

impl PriceAdapter for SwitchboardV2PriceFeed {
    fn get_price_of_type(
        &self,
        _price_type: OraclePriceType,
        bias: Option<PriceBias>,
    ) -> MarginfiResult<I80F48> {
        let price = self.get_price()?;

        match bias {
            Some(price_bias) => {
                let confidence_interval = self.get_confidence_interval()?;

                match price_bias {
                    PriceBias::Low => Ok(price
                        .checked_sub(confidence_interval)
                        .ok_or_else(math_error!())?),
                    PriceBias::High => Ok(price
                        .checked_add(confidence_interval)
                        .ok_or_else(math_error!())?),
                }
            }
            None => Ok(price),
        }
    }
}

/// A slimmed down version of the AggregatorAccountData struct copied from the switchboard-v2/src/aggregator.rs
#[cfg_attr(feature = "client", derive(Clone, Debug))]
struct LiteAggregatorAccountData {
    /// Use sliding windoe or round based resolution
    /// NOTE: This changes result propogation in latest_round_result
    pub resolution_mode: AggregatorResolutionMode,
    /// Latest confirmed update request result that has been accepted as valid.
    pub latest_confirmed_round_result: SwitchboardDecimal,
    pub latest_confirmed_round_num_success: u32,
    pub latest_confirmed_round_std_deviation: SwitchboardDecimal,
    /// Minimum number of oracle responses required before a round is validated.
    pub min_oracle_results: u32,
}

impl From<&AggregatorAccountData> for LiteAggregatorAccountData {
    fn from(agg: &AggregatorAccountData) -> Self {
        Self {
            resolution_mode: agg.resolution_mode,
            latest_confirmed_round_result: agg.latest_confirmed_round.result,
            latest_confirmed_round_num_success: agg.latest_confirmed_round.num_success,
            latest_confirmed_round_std_deviation: agg.latest_confirmed_round.std_deviation,
            min_oracle_results: agg.min_oracle_results,
        }
    }
}

impl LiteAggregatorAccountData {
    /// If sufficient oracle responses, returns the latest on-chain result in SwitchboardDecimal format
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use switchboard_v2::AggregatorAccountData;
    /// use std::convert::TryInto;
    ///
    /// let feed_result = AggregatorAccountData::new(feed_account_info)?.get_result()?;
    /// let decimal: f64 = feed_result.try_into()?;
    /// ```
    pub fn get_result(&self) -> anchor_lang::Result<SwitchboardDecimal> {
        if self.resolution_mode == AggregatorResolutionMode::ModeSlidingResolution {
            return Ok(self.latest_confirmed_round_result);
        }
        let min_oracle_results = self.min_oracle_results;
        let latest_confirmed_round_num_success = self.latest_confirmed_round_num_success;
        if min_oracle_results > latest_confirmed_round_num_success {
            return Err(MarginfiError::InvalidOracleAccount.into());
        }
        Ok(self.latest_confirmed_round_result)
    }
}

#[inline(always)]
fn pyth_price_components_to_i80f48(price: I80F48, exponent: i32) -> MarginfiResult<I80F48> {
    let scaling_factor = EXP_10_I80F48[exponent.unsigned_abs() as usize];

    let price = if exponent == 0 {
        price
    } else if exponent < 0 {
        price
            .checked_div(scaling_factor)
            .ok_or_else(math_error!())?
    } else {
        price
            .checked_mul(scaling_factor)
            .ok_or_else(math_error!())?
    };

    Ok(price)
}

/// Load and validate a pyth price feed account.
fn load_pyth_price_feed(ai: &AccountInfo) -> MarginfiResult<PriceFeed> {
    check!(ai.owner.eq(&PYTH_ID), MarginfiError::InvalidOracleAccount);
    let price_feed =
        load_price_feed_from_account_info(ai).map_err(|_| MarginfiError::InvalidOracleAccount)?;
    Ok(price_feed)
}

#[inline(always)]
fn swithcboard_decimal_to_i80f48(decimal: SwitchboardDecimal) -> Option<I80F48> {
    let decimal = fit_scale_switchboard_decimal(decimal, MAX_SCALE)?;

    I80F48::from_num(decimal.mantissa).checked_div(EXP_10_I80F48[decimal.scale as usize])
}

const MAX_SCALE: u32 = 20;

/// Scale a SwitchboardDecimal down to a given scale.
/// Return original SwitchboardDecimal if it is already at or below the given scale.
///
/// This may result in minimal loss of precision past the scale delta.
#[inline]
fn fit_scale_switchboard_decimal(
    decimal: SwitchboardDecimal,
    scale: u32,
) -> Option<SwitchboardDecimal> {
    if decimal.scale <= scale {
        return Some(decimal);
    }

    let scale_diff = decimal.scale - scale;
    let mantissa = decimal.mantissa.checked_div(EXP_10[scale_diff as usize])?;

    Some(SwitchboardDecimal { mantissa, scale })
}
