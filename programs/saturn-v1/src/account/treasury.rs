use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Treasury {
    pub treasury_admin: Pubkey, // 32
    pub staking_index: u64, // 8
    pub treasury_value: u64, // 8
    pub token_minted: u64, // 8
    pub token_staked: u64, // 8
    pub meteora_deposit_amount: u64,
    pub kamino_lend_amount: u64,
    pub marginfi_lend_amount: u64,
}  