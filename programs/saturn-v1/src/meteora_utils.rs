use std::str::FromStr;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_lang::prelude::*;

use meteora::state::Vault;
use meteora::cpi::accounts::{DepositWithdrawLiquidity, WithdrawDirectlyFromStrategy};
use meteora::cpi::*;

use crate::meteora_account::Partner;
use crate::error::*;
use crate::constants::PRICE_PRECISION;

/// MeteoraProgram struct
#[derive(Clone)]
pub struct MeteoraProgram;

impl anchor_lang::Id for MeteoraProgram {
    fn id() -> Pubkey {
        meteora::id()
    }
}

/// Admin address, only admin can initialize a partner
pub fn get_admin_address() -> Pubkey {
    Pubkey::from_str("DHLXnJdACTY83yKwnUkeoDjqi4QBbsYGa1v8tJL76ViX")
        .expect("Must be correct Solana address")
}

/// VaultUtils struct
pub struct MeteoraUtils;

impl MeteoraUtils {
    /// deposit to vault
    #[allow(clippy::too_many_arguments)]
    pub fn deposit<'info>(
        vault: &AccountInfo<'info>,
        lp_mint: &AccountInfo<'info>,
        user_token: &AccountInfo<'info>,
        user_lp: &AccountInfo<'info>,
        user: &AccountInfo<'info>,
        token_vault: &AccountInfo<'info>,
        token_program: &AccountInfo<'info>,
        vault_program: &AccountInfo<'info>,
        token_amount: u64,
        minimum_lp_amount: u64,
    ) -> Result<()> {
        let accounts = DepositWithdrawLiquidity {
            vault: vault.to_account_info(),
            lp_mint: lp_mint.to_account_info(),
            user_token: user_token.to_account_info(),
            user_lp: user_lp.to_account_info(),
            user: user.to_account_info(),
            token_vault: token_vault.to_account_info(),
            token_program: token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(vault_program.to_account_info(), accounts);
        deposit(cpi_ctx, token_amount, minimum_lp_amount)
    }
    /// withdraw from vault
    #[allow(clippy::too_many_arguments)]
    pub fn withdraw<'info>(
        vault: &AccountInfo<'info>,
        lp_mint: &AccountInfo<'info>,
        user_token: &AccountInfo<'info>,
        user_lp: &AccountInfo<'info>,
        user: &AccountInfo<'info>,
        token_vault: &AccountInfo<'info>,
        token_program: &AccountInfo<'info>,
        vault_program: &AccountInfo<'info>,
        unmint_amount: u64,
        minimum_out_amount: u64,
        signers: &[&[&[u8]]],
    ) -> Result<()> {
        let accounts = DepositWithdrawLiquidity {
            vault: vault.to_account_info(),
            lp_mint: lp_mint.to_account_info(),
            user_token: user_token.to_account_info(),
            user_lp: user_lp.to_account_info(),
            user: user.to_account_info(),
            token_vault: token_vault.to_account_info(),
            token_program: token_program.to_account_info(),
        };
        let cpi_ctx =
            CpiContext::new_with_signer(vault_program.to_account_info(), accounts, signers);

        withdraw(cpi_ctx, unmint_amount, minimum_out_amount)
    }
    /// withdraw directly from strategy
    #[allow(clippy::too_many_arguments)]
    pub fn withdraw_directly_from_strategy<'info>(
        vault: &AccountInfo<'info>,
        strategy: &AccountInfo<'info>,
        reserve: &AccountInfo<'info>,
        strategy_program: &AccountInfo<'info>,
        collateral_vault: &AccountInfo<'info>,
        token_vault: &AccountInfo<'info>,
        lp_mint: &AccountInfo<'info>,
        fee_vault: &AccountInfo<'info>,
        user_token: &AccountInfo<'info>,
        user_lp: &AccountInfo<'info>,
        user: &AccountInfo<'info>,
        token_program: &AccountInfo<'info>,
        vault_program: &AccountInfo<'info>,
        remaining_accounts: &[AccountInfo<'info>],
        unmint_amount: u64,
        minimum_out_amount: u64,
        signers: &[&[&[u8]]],
    ) -> Result<()> {
        let accounts = WithdrawDirectlyFromStrategy {
            vault: vault.clone(),
            strategy: strategy.clone(),
            reserve: reserve.clone(),
            strategy_program: strategy_program.clone(),
            collateral_vault: collateral_vault.clone(),
            token_vault: token_vault.clone(),
            lp_mint: lp_mint.clone(),
            fee_vault: fee_vault.clone(),
            user_token: user_token.clone(),
            user_lp: user_lp.clone(),
            user: user.clone(),
            token_program: token_program.clone(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(vault_program.to_account_info(), accounts, signers)
                .with_remaining_accounts(remaining_accounts.to_vec());

        withdraw_directly_from_strategy(cpi_ctx, unmint_amount, minimum_out_amount)
    }
}

/// update liquidity
pub fn update_liquidity_wrapper<'info>(
    update_liquidity_fn: impl FnOnce() -> Result<()>,
    vault: &mut Account<'info, Vault>,
    vault_lp_mint: &mut Account<'info, Mint>,
    user_lp: &mut Account<'info, TokenAccount>,
    partner: &mut Account<'info, Partner>,
    // user: &mut Account<'info, User>,
) -> Result<()> {
    // accrue fee
    let current_time = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or(VaultError::MathOverflow)?;
    let virtual_price = vault
        .get_virtual_price(current_time, vault_lp_mint.supply)
        .ok_or(VaultError::MathOverflow)?;

    let fee_ratio = partner.fee_ratio;

    let fee = partner
        .get_fee(virtual_price, fee_ratio)
        .ok_or(VaultError::MathOverflow)?;

    msg!("fee: {}", fee);
    emit!(PartnerFee { fee });
    // acrrure fee for partner
    partner.accrue_fee(fee).ok_or(VaultError::MathOverflow)?;

    update_liquidity_fn()?;

    // save new user state
    user_lp.reload()?;
    partner.set_new_state(virtual_price, user_lp.amount);

    Ok(())
}

/// VirtualPrice trait
pub trait VirtualPrice {
    /// get virtual price
    fn get_virtual_price(&self, current_time: u64, lp_supply: u64) -> Option<u64>;
}

impl VirtualPrice for Vault {
    fn get_virtual_price(&self, current_time: u64, lp_supply: u64) -> Option<u64> {
        let unlocked_amount = self.get_unlocked_amount(current_time)?;
        let virtual_price = u128::from(unlocked_amount)
            .checked_mul(PRICE_PRECISION)?
            .checked_div(u128::from(lp_supply))?;
        u64::try_from(virtual_price).ok()
    }
}

#[event]
/// PartnerFee struct
pub struct PartnerFee {
    fee: u64,
}