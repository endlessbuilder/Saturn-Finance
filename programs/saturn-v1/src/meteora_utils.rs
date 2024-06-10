use anchor_lang::prelude::*;

/// Admin address, only admin can initialize a partner
pub fn get_admin_address() -> Pubkey {
    Pubkey::from_str("DHLXnJdACTY83yKwnUkeoDjqi4QBbsYGa1v8tJL76ViX")
        .expect("Must be correct Solana address")
}