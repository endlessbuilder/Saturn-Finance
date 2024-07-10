use crate::account::meteora_account::Partner;
use crate::meteora_utils::{MeteoraProgram, MeteoraUtils};
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{instructions::Instructions as SysInstructions, SysvarId},
};
use anchor_spl::token::accessor::amount;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use kamino_lending::program::KaminoLending;
use kamino_lending::state::{LendingMarket, Obligation, Reserve};
use marginfi::program::Marginfi;
use marginfi::state::{
    marginfi_account::MarginfiAccount,
    marginfi_group::{Bank, MarginfiGroup},
};
use meteora::state::Vault;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

use crate::{utils::*, GetValueInKamino};
use crate::{account::*, constants::*};

/// Need to check whether we can convert to unchecked account
#[derive(Accounts)]
pub struct ReAllocate<'info> {
    /// user CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,
    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    //****** kamino accounts ******
    #[account(
        mut,
        constraint = kamino_obligation.load()?.lending_market == kamino_lending_market.key(),
        constraint = kamino_obligation.load()?.owner == treasury_authority.key(),
    )]
    pub kamino_obligation: AccountLoader<'info, Obligation>,
    pub kamino_lending_market: AccountLoader<'info, LendingMarket>,
    /// CHECK: just authority
    pub kamino_lending_market_authority: AccountInfo<'info>,
    #[account(
        mut,
        constraint = kamino_reserve.load()?.lending_market == kamino_lending_market.key()
    )]
    pub kamino_reserve: AccountLoader<'info, Reserve>,
    #[account(mut,
        address = kamino_reserve.load()?.liquidity.supply_vault
    )]
    pub kamino_reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        address = kamino_reserve.load()?.collateral.mint_pubkey
    )]
    pub kamino_reserve_collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        address = kamino_reserve.load()?.collateral.supply_vault
    )]
    pub kamino_reserve_collateral_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        token::mint = kamino_reserve.load()?.liquidity.mint_pubkey
    )]
    pub kamino_user_source_liquidity: Account<'info, TokenAccount>,
    #[account(mut,
        token::mint = kamino_reserve_collateral_mint.key()
    )]
    pub kamino_user_destination_collateral: Box<Account<'info, TokenAccount>>,

    
    #[account(
        mut, 
        constraint = kamino_obligation.load()?.lending_market == kamino_lending_market.key(),
        constraint = sol_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == SOL_MINT,
    )]
    pub sol_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        
        constraint = usdc_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == USDC_MINT,
    )]
    pub usdc_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        
        constraint = usdt_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == USDT_MINT,
    )]
    pub usdt_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        
        constraint = wbtc_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == WBTC_MINT,
    )]
    pub wbtc_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        
        constraint = weth_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == WETH_MINT,
    )]
    pub weth_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        
        constraint = bonk_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == BONK_MINT,
    )]
    pub bonk_reserve: AccountLoader<'info, Reserve>,


    #[account(address = SysInstructions::id())]
    /// CHECK:address checked
    pub instruction_sysvar_account: AccountInfo<'info>,
    pub kamino_program: Program<'info, KaminoLending>,

    //****** marginfi accounts *******
    pub marginfi_group: AccountLoader<'info, MarginfiGroup>,
    #[account(
        mut,
        constraint = marginfi_account.load()?.group == marginfi_group.key(),
        constraint = marginfi_account.load()?.authority == treasury_authority.key(),
    )]
    pub marginfi_account: AccountLoader<'info, MarginfiAccount>,
    #[account(
        mut,
        constraint = marginfi_bank.load()?.group == marginfi_group.key(),
    )]
    pub marginfi_bank: AccountLoader<'info, Bank>,
    /// CHECK: marginfi account
    #[account(mut)]
    pub marginfi_bank_liquidity_vault: Account<'info, TokenAccount>,
    /// CHECK: Seed constraint check
    #[account(mut)]
    pub marginfi_bank_liquidity_vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub marginfi_user_liquidity: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
    pub marginfi_program: Program<'info, Marginfi>,

    #[account(
        mut,
        constraint = sol_bank.load()?.group == marginfi_group.key(),
        constraint = sol_bank.load()?.mint.to_string().as_str() == SOL_MINT,
    )]
    pub sol_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = usdc_bank.load()?.group == marginfi_group.key(),
        constraint = usdc_bank.load()?.mint.to_string().as_str() == USDC_MINT,
    )]
    pub usdc_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = usdt_bank.load()?.group == marginfi_group.key(),
        constraint = usdt_bank.load()?.mint.to_string().as_str() == USDT_MINT,
    )]
    pub usdt_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = wbtc_bank.load()?.group == marginfi_group.key(),
        constraint = wbtc_bank.load()?.mint.to_string().as_str() == WBTC_MINT,
    )]
    pub wbtc_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = weth_bank.load()?.group == marginfi_group.key(),
        constraint = weth_bank.load()?.mint.to_string().as_str() == WETH_MINT,
    )]
    pub weth_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = bonk_bank.load()?.group == marginfi_group.key(),
        constraint = bonk_bank.load()?.mint.to_string().as_str() == BONK_MINT,
    )]
    pub bonk_bank: AccountLoader<'info, Bank>,

    //******** meteora accounts *********
    /// partner info CHECK:
    #[account(mut, constraint = meteora_partner.vault == meteora_vault.key())]
    pub meteora_partner: Box<Account<'info, Partner>>,
    /// CHECK:
    #[account(mut)]
    pub meteora_vault: Box<Account<'info, Vault>>,
    /// CHECK:
    #[account(mut)]
    pub meteora_token_vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub meteora_vault_lp_mint: Box<Account<'info, Mint>>,
    /// treasury token CHECK:
    #[account(mut)]
    pub meteora_treasury_token: UncheckedAccount<'info>,
    /// treasury lp CHECK:
    #[account(mut, constraint = meteora_treasury_lp.owner == treasury_authority.key())]
    //mint to account of treasury PDA
    pub meteora_treasury_lp: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    pub meteora_vault_program: Program<'info, MeteoraProgram>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[allow(unused_variables)]
pub fn handle(ctx: Context<ReAllocate>) -> Result<()> {
    let get_kamino_value_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: ctx.program_id.key(),
        accounts: vec![
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.treasury_authority.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.kamino_lending_market.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.sol_reserve.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.usdc_reserve.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.usdt_reserve.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.wbtc_reserve.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.weth_reserve.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.bonk_reserve.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.kamino_obligation.key(), true),
        ],
        data: Vec::new(),
    };

    anchor_lang::solana_program::program::invoke(
        &get_kamino_value_ix,
        &[
            // ctx.accounts.program_id.clone(),
        ctx.accounts.treasury_authority.to_account_info().clone(),
        ctx.accounts.kamino_lending_market.to_account_info().clone(),
        ctx.accounts.sol_reserve.to_account_info().clone(),
        ctx.accounts.usdc_reserve.to_account_info().clone(),
        ctx.accounts.usdt_reserve.to_account_info().clone(),
        ctx.accounts.wbtc_reserve.to_account_info().clone(),
        ctx.accounts.weth_reserve.to_account_info().clone(),
        ctx.accounts.bonk_reserve.to_account_info().clone(),
        ctx.accounts.kamino_obligation.to_account_info().clone(),
        ],
    )?;

    let get_marginfi_value_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: ctx.program_id.key(),
        accounts: vec![
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.treasury_authority.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.marginfi_group.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.marginfi_account.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.sol_bank.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.usdc_bank.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.usdt_bank.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.wbtc_bank.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.weth_bank.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.bonk_bank.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.kamino_obligation.key(), true),
        ],
        data: Vec::new(),
    };

    anchor_lang::solana_program::program::invoke(
        &get_marginfi_value_ix,
        &[
            // ctx.accounts.program_id.clone(),
        ctx.accounts.treasury_authority.to_account_info().clone(),
        ctx.accounts.kamino_lending_market.to_account_info().clone(),
        ctx.accounts.sol_bank.to_account_info().clone(),
        ctx.accounts.usdc_bank.to_account_info().clone(),
        ctx.accounts.usdt_bank.to_account_info().clone(),
        ctx.accounts.wbtc_bank.to_account_info().clone(),
        ctx.accounts.weth_bank.to_account_info().clone(),
        ctx.accounts.bonk_bank.to_account_info().clone(),
        ctx.accounts.kamino_obligation.to_account_info().clone(),
        ],
    )?;

    let get_meteora_value_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: ctx.program_id.key(),
        accounts: vec![
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.treasury_authority.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.meteora_treasury_lp.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.meteora_vault_lp_mint.key(), true),
            anchor_lang::solana_program::instruction::AccountMeta::new(ctx.accounts.meteora_vault.key(), true),
        ],
        data: Vec::new(),
    };

    anchor_lang::solana_program::program::invoke(
        &get_meteora_value_ix,
        &[
            // ctx.accounts.program_id.clone(),
        ctx.accounts.treasury_authority.to_account_info().clone(),
        ctx.accounts.meteora_treasury_lp.to_account_info().clone(),
        ctx.accounts.meteora_vault_lp_mint.to_account_info().clone(),
        ctx.accounts.meteora_vault.to_account_info().clone(),
        ],
    )?;



    Ok(())
}
