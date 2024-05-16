use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
#[derive(Default)]
pub struct Treasury {
    pub treasury: Pubkey, // 32
}

#[account(zero_copy)]
pub struct Escrow {
    // 8 + 32*2 + 8*3 = 96
    pub creator: Pubkey,                    //32    
    pub token_mint: Pubkey,                   //32
    pub token_amount: u64,                  //8
    pub start_timestamp: i64,               //8
    pub end_timestamp: i64,                 //8    
}