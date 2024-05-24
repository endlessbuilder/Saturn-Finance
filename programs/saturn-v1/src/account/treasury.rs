use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Treasury {
    pub treasury: Pubkey, // 32
    pub sstf: u64, // 8
    pub treasury_value: u64, // 8
    pub token_minted: u64, // 8
    pub token_staked: u64 // 8
}   