use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Treasury {
    pub treasury: Pubkey, // 32
}