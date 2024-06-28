use anchor_lang::{account, prelude::*, solana_program::clock::Slot};
use derivative::Derivative;

use super::LastUpdate;
use crate::{
    utils::ELEVATION_GROUP_NONE, utils::OBLIGATION_SIZE,
    BigFractionBytes,
    LendingError,
    xmsg
};

static_assertions::const_assert_eq!(OBLIGATION_SIZE, std::mem::size_of::<Obligation>());
static_assertions::const_assert_eq!(0, std::mem::size_of::<Obligation>() % 8);
#[derive(PartialEq, Derivative)]
#[derivative(Debug)]
#[account(zero_copy)]
#[repr(C)]
pub struct Obligation {
    pub tag: u64,
    pub last_update: LastUpdate,
    pub lending_market: Pubkey,
    pub owner: Pubkey,
    pub deposits: [ObligationCollateral; 8],
    pub lowest_reserve_deposit_liquidation_ltv: u64,
    pub deposited_value_sf: u128,

    pub borrows: [ObligationLiquidity; 5],
    pub borrow_factor_adjusted_debt_value_sf: u128,
    pub borrowed_assets_market_value_sf: u128,
    pub allowed_borrow_value_sf: u128,
    pub unhealthy_borrow_value_sf: u128,

    pub deposits_asset_tiers: [u8; 8],
    pub borrows_asset_tiers: [u8; 5],

    pub elevation_group: u8,

    pub num_of_obsolete_reserves: u8,

    pub has_debt: u8,

    pub referrer: Pubkey,

    pub borrowing_disabled: u8,

    #[derivative(Debug = "ignore")]
    pub reserved: [u8; 7],

    pub highest_borrow_factor_pct: u64,

    #[derivative(Debug = "ignore")]
    pub padding_3: [u64; 126],
}

impl Default for Obligation {
    fn default() -> Self {
        Self {
            tag: 0,
            last_update: LastUpdate::default(),
            lending_market: Pubkey::default(),
            owner: Pubkey::default(),
            deposits: [ObligationCollateral::default(); 8],
            borrows: [ObligationLiquidity::default(); 5],
            deposited_value_sf: 0,
            borrowed_assets_market_value_sf: 0,
            allowed_borrow_value_sf: 0,
            unhealthy_borrow_value_sf: 0,
            lowest_reserve_deposit_liquidation_ltv: 0,
            borrow_factor_adjusted_debt_value_sf: 0,
            deposits_asset_tiers: [u8::MAX; 8],
            borrows_asset_tiers: [u8::MAX; 5],
            elevation_group: ELEVATION_GROUP_NONE,
            num_of_obsolete_reserves: 0,
            has_debt: 0,
            borrowing_disabled: 0,
            highest_borrow_factor_pct: 0,
            reserved: [0; 7],
            padding_3: [0; 126],
            referrer: Pubkey::default(),
        }
    }
}

impl Obligation {
    pub fn borrows_empty(&self) -> bool {
        self.borrows
            .iter()
            .all(|l| l.borrow_reserve == Pubkey::default())
    }
    
    pub fn deposits_empty(&self) -> bool {
        self.deposits
            .iter()
            .all(|c| c.deposit_reserve == Pubkey::default())
    }
    
    pub fn position_of_collateral_in_deposits(&self, deposit_reserve: Pubkey) -> Result<usize> {
        if self.deposits_empty() {
            xmsg!("Obligation has no deposits");
            return err!(LendingError::ObligationDepositsEmpty);
        }
        self.deposits
            .iter()
            .position(|collateral| collateral.deposit_reserve == deposit_reserve)
            .ok_or(error!(LendingError::InvalidObligationCollateral))
    }
}
pub struct InitObligationParams {
    pub current_slot: Slot,
    pub lending_market: Pubkey,
    pub owner: Pubkey,
    pub deposits: [ObligationCollateral; 8],
    pub borrows: [ObligationLiquidity; 5],
    pub tag: u64,
    pub referrer: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitObligationArgs {
    pub tag: u8,
    pub id: u8,
}

#[derive(Debug, Default, PartialEq, Eq)]
#[zero_copy]
#[repr(C)]
pub struct ObligationCollateral {
    pub deposit_reserve: Pubkey,
    pub deposited_amount: u64,
    pub market_value_sf: u128,
    pub borrowed_amount_against_this_collateral_in_elevation_group: u64,
    pub padding: [u64; 9],
}

#[derive(Debug, Default, PartialEq, Eq)]
#[zero_copy]
#[repr(C)]
pub struct ObligationLiquidity {
    pub borrow_reserve: Pubkey,
    pub cumulative_borrow_rate_bsf: BigFractionBytes,
    pub padding: u64,
    pub borrowed_amount_sf: u128,
    pub market_value_sf: u128,
    pub borrow_factor_adjusted_market_value_sf: u128,

    pub borrowed_amount_outside_elevation_groups: u64,

    pub padding2: [u64; 7],
}
