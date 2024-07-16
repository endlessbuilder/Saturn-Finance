use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct SequenceFlag {
    pub flag_calcu_balance: bool, // 1
    pub flag_reallocate: bool, // 1
    pub flag_marginfi: bool, // 1
    pub flag_kamino: bool, // 1
    pub flag_meteora: bool, // 1
    pub flag_jupiter: bool, // 1
    pub flag_swap: bool, // 1 
}
