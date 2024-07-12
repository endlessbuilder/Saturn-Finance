use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Treasury {
    pub treasury_admin: Pubkey, // 32
    pub staking_index: u64, // 8
    pub treasury_value: u64, // 8
    pub token_minted: u64, // 8
    pub token_staked: u64, // 8
    
    pub meteora_deposit_assets: u64,
    pub meteora_deposit_value: u64,
    pub meteora_allocation: f64,
    pub kamino_lend_assets: u64,
    pub kamino_lend_value: u64,
    pub kamino_allocation: f64,
    pub marginfi_lend_assets: u64,
    pub marginfi_lend_value: u64,
    pub marginfi_allocation: f64,
    pub jupiter_perps_assets: u64,
    pub jupiter_perps_value: u64,
    pub jupiter_allocation: f64,
    pub usdc_allocation: f64,
    pub wbtc_allocation: f64,
    pub sol_allocation: f64,
}  