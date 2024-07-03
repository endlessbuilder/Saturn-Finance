use anchor_lang::prelude::*;
use anchor_spl::token::accessor::amount;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};


/// Need to check whether we can convert to unchecked account
#[derive(Accounts)]
pub struct MeteoraDeposit<'info> {
    /// partner info CHECK:
    #[account(mut, has_one = vault)]
    pub partner: Box<Account<'info, Partner>>,
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
    /// CHECK:
    pub vault_program: Program<'info, MeteoraProgram>,
    /// CHECK:
    #[account(mut)]
    pub vault: Box<Account<'info, Vault>>,
    /// CHECK:
    #[account(mut)]
    pub token_vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub vault_lp_mint: Box<Account<'info, Mint>>,
    /// treasury token CHECK:
    #[account(mut)]
    pub treasury_token: UncheckedAccount<'info>,
    /// treasury lp CHECK:
    #[account(mut, constraint = treasury_lp.owner == treasury_authority.key())] //mint to account of treasury PDA
    pub treasury_lp: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    // pub owner: Signer<'info>,
    /// CHECK:
    pub token_program: Program<'info, Token>,
}

#[allow(unused_variables)]
pub fn handle(
    ctx: Context<MeteoraDeposit>,
    treasur: [[u64; 5]; 7],
    platform_allocation: [u64; 4]
) -> Result<()> {
    

    Ok(())
}
