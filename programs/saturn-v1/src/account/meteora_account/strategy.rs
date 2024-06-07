#[account]
#[derive(Default, Debug)]
pub struct Strategy {
    pub reserve: Pubkey,
    pub collateral_vault: Pubkey,
    pub strategy_type: StrategyType,
    pub current_liquidity: u64,
    pub bumps: [u8; MAX_BUMPS],
    pub vault: Pubkey,
    pub is_disable: u8,
}