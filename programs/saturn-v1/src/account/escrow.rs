use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct Escrow {
    // 8 + 32*2 + 8*4 = 104
    pub creator: Pubkey,          //32
    pub token_mint: Pubkey,       //32
    pub token_amount: u64,        //8
    pub start_timestamp: i64,     //8
    pub end_timestamp: i64,       //8
    pub num_token_to_redeem: u64, //8
    pub is_finished: u64,
}
