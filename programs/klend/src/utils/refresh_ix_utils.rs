use anchor_lang::prelude::*;


#[derive(Debug, Clone)]
pub enum RequiredIxType {
    RefreshReserve,
    RefreshFarmsForObligationForReserve,
    RefreshObligation,
}

#[derive(Debug, Clone)]
pub struct RequiredIx {
    pub kind: RequiredIxType,
    pub accounts: Vec<(Pubkey, usize)>,
}
// enum AppendedIxType {
//     PreIxs,
//     PostIxs,
// }
