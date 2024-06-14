use crate::{
    state::{obligation::Obligation, LendingMarket, Reserve},
    utils::{
        consts::{OBLIGATION_SIZE, USER_METADATA_SIZE},
        seeds,
    },
    InitObligationArgs, LendingError, ReferrerTokenState, UserMetadata,
};
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{instructions::Instructions as SysInstructions, SysvarId},
    Accounts,
};
use anchor_spl::token::{Mint, Token, TokenAccount};
use farms::{program::Farms, state::UserState as FarmsUserState};
use solana_program::sysvar::instructions;

#[derive(Accounts)]
pub struct InitUserMetadata<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(init,
        seeds = [seeds::BASE_SEED_USER_METADATA, owner.key().as_ref()],
        bump,
        payer = fee_payer,
        space = USER_METADATA_SIZE + 8,
    )]
    pub user_metadata: AccountLoader<'info, UserMetadata>,

    pub referrer_user_metadata: Option<AccountLoader<'info, UserMetadata>>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(args: InitObligationArgs)]
pub struct InitObligation<'info> {
    pub obligation_owner: Signer<'info>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(init,
        seeds = [&[args.tag], &[args.id], obligation_owner.key().as_ref(), lending_market.key().as_ref(), seed1_account.key().as_ref(), seed2_account.key().as_ref()],
        bump,
        payer = fee_payer,
        space = OBLIGATION_SIZE + 8,
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    /// CHECK: just seeds
    pub seed1_account: AccountInfo<'info>,
    /// CHECK: just seeds
    pub seed2_account: AccountInfo<'info>,

    #[account(
        seeds = [seeds::BASE_SEED_USER_METADATA, obligation_owner.key().as_ref()],
        bump = owner_user_metadata.load()?.bump.try_into().unwrap(),
    )]
    pub owner_user_metadata: AccountLoader<'info, UserMetadata>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositReserveLiquidityAndObligationCollateral<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut,
        has_one = lending_market,
        has_one = owner
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    pub lending_market: AccountLoader<'info, LendingMarket>,
    #[account(
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub reserve: AccountLoader<'info, Reserve>,
    #[account(mut,
        address = reserve.load()?.liquidity.supply_vault
    )]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        address = reserve.load()?.collateral.mint_pubkey
    )]
    pub reserve_collateral_mint: Box<Account<'info, Mint>>,

    #[account(mut,
        address = reserve.load()?.collateral.supply_vault
    )]
    pub reserve_destination_deposit_collateral: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        token::mint = reserve.load()?.liquidity.mint_pubkey
    )]
    pub user_source_liquidity: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        token::mint = reserve_collateral_mint.key()
    )]
    pub user_destination_collateral: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    #[account(address = SysInstructions::id())]
    /// CHECK:address checked
    pub instruction_sysvar_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateral<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut,
        has_one = lending_market,
        has_one = owner
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    pub lending_market: AccountLoader<'info, LendingMarket>,
    #[account(
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub withdraw_reserve: AccountLoader<'info, Reserve>,

    #[account(mut, address = withdraw_reserve.load()?.collateral.supply_vault)]
    pub reserve_source_collateral: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = withdraw_reserve.load()?.collateral.mint_pubkey)]
    pub reserve_collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut, address = withdraw_reserve.load()?.liquidity.supply_vault)]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        token::mint = withdraw_reserve.load()?.liquidity.mint_pubkey
    )]
    pub user_destination_liquidity: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        token::mint = reserve_collateral_mint.key()
    )]
    pub user_destination_collateral: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    #[account(address = SysInstructions::id())]
    /// CHECK:address checked
    pub instruction_sysvar_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitFarmsForReserve<'info> {
    #[account(mut)]
    pub lending_market_owner: Signer<'info>,
    #[account(has_one = lending_market_owner)]
    pub lending_market: AccountLoader<'info, LendingMarket>,
    #[account(
        mut,
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub reserve: AccountLoader<'info, Reserve>,

    pub farms_program: Program<'info, Farms>,
    pub farms_global_config: AccountInfo<'info>,

    #[account(mut)]
    pub farm_state: AccountInfo<'info>,

    pub farms_vault_authority: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitObligationFarmsForReserve<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub owner: AccountInfo<'info>,

    #[account(
        mut,
        has_one = lending_market,
        has_one = owner
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    #[account(
        mut,
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    pub lending_market_authority: AccountInfo<'info>,

    #[account(
        mut,
        has_one = lending_market
    )]
    pub reserve: AccountLoader<'info, Reserve>,

    #[account(mut)]
    pub reserve_farm_state: AccountInfo<'info>,

    #[account(mut)]
    pub obligation_farm: AccountInfo<'info>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    pub farms_program: Program<'info, Farms>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RefreshObligationFarmsForReserve<'info> {
    #[account(mut)]
    pub crank: Signer<'info>,
    #[account(
        constraint = obligation_farm_user_state.load()?.delegatee == obligation.key() @ LendingError::InvalidAccountInput
    )]
    /// CHECK: farm stuff
    pub obligation: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,

    #[account(has_one = lending_market)]
    pub reserve: AccountLoader<'info, Reserve>,

    #[account(mut)]
    /// CHECK: farm stuff
    pub reserve_farm_state: AccountInfo<'info>,

    #[account(mut,
        constraint = obligation_farm_user_state.load()?.delegatee == obligation.key() @ LendingError::InvalidAccountInput,
    )]
    pub obligation_farm_user_state: AccountLoader<'info, FarmsUserState>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    pub farms_program: Program<'info, Farms>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FlashRepayReserveLiquidity<'info> {
    pub user_transfer_authority: Signer<'info>,

    #[account(
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    #[account(mut,
        has_one = lending_market
    )]
    pub reserve: AccountLoader<'info, Reserve>,

    #[account(mut,
        address = reserve.load()?.liquidity.supply_vault
    )]
    pub reserve_destination_liquidity: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user_source_liquidity: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        address = reserve.load()?.liquidity.fee_vault
    )]
    pub reserve_liquidity_fee_receiver: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub referrer_token_state: Option<AccountLoader<'info, ReferrerTokenState>>,

    #[account(mut)]
    pub referrer_account: Option<AccountInfo<'info>>,

    #[account(address = instructions::ID)]
    /// CHECK: address checked
    pub sysvar_info: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct FlashBorrowReserveLiquidity<'info> {
    pub user_transfer_authority: Signer<'info>,

    #[account(
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    #[account(mut,
        has_one = lending_market
    )]
    pub reserve: AccountLoader<'info, Reserve>,

    #[account(mut,
        address = reserve.load()?.liquidity.supply_vault
    )]
    pub reserve_source_liquidity: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user_destination_liquidity: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        address = reserve.load()?.liquidity.fee_vault
    )]
    pub reserve_liquidity_fee_receiver: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub referrer_token_state: Option<AccountLoader<'info, ReferrerTokenState>>,

    #[account(mut)]
    pub referrer_account: Option<AccountInfo<'info>>,

    #[account(address = instructions::ID)]
    /// CHECK: address checked
    pub sysvar_info: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BorrowObligationLiquidity<'info> {
    pub owner: Signer<'info>,

    #[account(mut,
        has_one = lending_market,
        has_one = owner @ LendingError::InvalidObligationOwner
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    pub lending_market: AccountLoader<'info, LendingMarket>,
    #[account(
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub borrow_reserve: AccountLoader<'info, Reserve>,

    #[account(mut,
        address = borrow_reserve.load()?.liquidity.supply_vault
    )]
    pub reserve_source_liquidity: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        address = borrow_reserve.load()?.liquidity.fee_vault
    )]
    pub borrow_reserve_liquidity_fee_receiver: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        token::mint = reserve_source_liquidity.mint
    )]
    pub user_destination_liquidity: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub referrer_token_state: Option<AccountInfo<'info>>,

    pub token_program: Program<'info, Token>,

    #[account(address = SysInstructions::id())]
    /// CHECK:address checked
    pub instruction_sysvar_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LiquidateObligationAndRedeemReserveCollateral<'info> {
    pub liquidator: Signer<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    pub lending_market: AccountLoader<'info, LendingMarket>,
    #[account(
        seeds = [seeds::LENDING_MARKET_AUTH, lending_market.key().as_ref()],
        bump = lending_market.load()?.bump_seed as u8,
    )]
    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub repay_reserve: AccountLoader<'info, Reserve>,
    #[account(mut,
        address = repay_reserve.load()?.liquidity.supply_vault
    )]
    pub repay_reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        has_one = lending_market
    )]
    pub withdraw_reserve: AccountLoader<'info, Reserve>,
    #[account(mut,
        address = withdraw_reserve.load()?.collateral.mint_pubkey
    )]
    pub withdraw_reserve_collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        address = withdraw_reserve.load()?.collateral.supply_vault
    )]
    pub withdraw_reserve_collateral_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        address = withdraw_reserve.load()?.liquidity.supply_vault
    )]
    pub withdraw_reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        address = withdraw_reserve.load()?.liquidity.fee_vault
    )]
    pub withdraw_reserve_liquidity_fee_receiver: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user_source_liquidity: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_destination_collateral: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_destination_liquidity: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    #[account(address = SysInstructions::id())]
    /// CHECK:address checked
    pub instruction_sysvar_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RepayObligationLiquidity<'info> {
    pub owner: Signer<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    #[account(mut,
        has_one = lending_market
    )]
    pub repay_reserve: AccountLoader<'info, Reserve>,

    #[account(mut,
        address = repay_reserve.load()?.liquidity.supply_vault
    )]
    pub reserve_destination_liquidity: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        token::mint = repay_reserve.load()?.liquidity.mint_pubkey
    )]
    pub user_source_liquidity: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    #[account(address = SysInstructions::id())]
    /// CHECK:address checked
    pub instruction_sysvar_account: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct RefreshObligation<'info> {
    pub lending_market: AccountLoader<'info, LendingMarket>,
    #[account(mut, has_one = lending_market)]
    pub obligation: AccountLoader<'info, Obligation>,
}

#[derive(Accounts)]
pub struct RefreshReserve<'info> {
    #[account(mut,
        has_one = lending_market,
    )]
    pub reserve: AccountLoader<'info, Reserve>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    pub pyth_oracle: Option<AccountInfo<'info>>,

    pub switchboard_price_oracle: Option<AccountInfo<'info>>,
    pub switchboard_twap_oracle: Option<AccountInfo<'info>>,

    pub scope_prices: Option<AccountInfo<'info>>,
}
