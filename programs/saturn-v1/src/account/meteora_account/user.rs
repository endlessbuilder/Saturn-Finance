use anchor_lang::prelude::*;
use meteora::{
    PERFORMANCE_FEE_DENOMINATOR,
    PERFORMANCE_FEE_NUMERATOR,
};

use crate::constants::{
    FEE_DENOMINATOR,
    PRICE_PRECISION,
};

// User struct
#[account]
#[derive(Default, Debug)]
pub struct User {
    pub owner: Pubkey,
    /// partner address, each user can integrate with more partners
    pub partner: Pubkey,
    
}

impl User {
    
}