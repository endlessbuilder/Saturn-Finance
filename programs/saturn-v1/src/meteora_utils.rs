use anchor_lang::prelude::*;
use anchor_lang::solana_program::borsh0_10;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use std::str::FromStr;
use std::collections::HashMap;
use marinade_sdk::state::State;
use std::convert::TryInto;
use dynamic_amm::state::DepegType;
use dynamic_amm::constants::depeg;
use spl_stake_pool::state::StakePool;

pub fn get_stake_pool_virtual_price(
    depeg_type: DepegType,
    spl_stake_pool: Pubkey,
    stake_data: HashMap<Pubkey, Vec<u8>>,
) -> Option<u64> {
    match depeg_type {
        DepegType::Lido => solido::get_virtual_price(&stake_data.get(&solido::stake::ID)?),
        DepegType::Marinade => marinade::get_virtual_price(&stake_data.get(&marinade::stake::ID)?),
        DepegType::SplStake => spl_stake::get_virtual_price(&stake_data.get(&spl_stake_pool)?),
        DepegType::None => None,
    }
}

/// Marinade module consists of functions to support marinade depeg pool operation
pub mod marinade {
    pub fn get_virtual_price(bytes: &[u8]) -> Option<u64> {
        let stake_state = State::deserialize(&mut &bytes[8..]).ok()?;

        let virtual_price = (stake_state.msol_price as u128)
            .checked_mul(depeg::PRECISION as u128)?
            .checked_div(State::PRICE_DENOMINATOR as u128)?;

        virtual_price.try_into().ok()
    }

    pub mod stake {
        use anchor_lang::prelude::declare_id;
        use super::*;
        declare_id!("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC");
    }
}

/// Solido module consists of functions to support solido depeg pool operation
pub mod solido {
    pub fn get_virtual_price(bytes: &[u8]) -> Option<u64> {
        let mut stsol_supply_byte = [0u8; 8];
        let mut stol_balance_bytes = [0u8; 8];

        stsol_supply_byte.copy_from_slice(&bytes[73..81]);
        stol_balance_bytes.copy_from_slice(&bytes[81..89]);

        let stsol_supply = u64::from_le_bytes(stsol_supply_byte);
        let sol_balance = u64::from_le_bytes(stol_balance_bytes);

        let stsol_price = (sol_balance as u128)
            .checked_mul(depeg::PRECISION as u128)?
            .checked_div(stsol_supply as u128)?;

        stsol_price.try_into().ok()
    }

    pub mod stake {
        use anchor_lang::prelude::declare_id;
        declare_id!("49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn");
    }
}

/// SPL stake pool module consists of functions to support SPL stake pool based depeg pool operation
pub mod spl_stake {
    pub fn get_virtual_price(bytes: &[u8]) -> Option<u64> {
        let stake: StakePool = borsh0_10::try_from_slice_unchecked(bytes).ok()?;

        let virtual_price = (stake.total_lamports as u128)
            .checked_mul(depeg::PRECISION as u128)?
            .checked_div(stake.pool_token_supply as u128)?;

        virtual_price.try_into().ok()
    }
}
