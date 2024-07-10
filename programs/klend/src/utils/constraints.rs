use anchor_lang::{
    err,
    prelude::{AccountLoader, Context},
    Bumps, Result,
};

use crate::{state::LendingMarket, LendingError};

pub fn emergency_mode_disabled(lending_market: &AccountLoader<LendingMarket>) -> Result<()> {
    if lending_market.load()?.emergency_mode > 0 {
        return err!(LendingError::GlobalEmergencyMode);
    }
    Ok(())
}

pub fn check_remaining_accounts<T>(ctx: &Context<T>) -> Result<()>
where
    T: Bumps,
{
    if !ctx.remaining_accounts.is_empty() {
        return err!(LendingError::InvalidAccountInput);
    }

    Ok(())
}

// pub mod token_2022 {
//     use crate::{xmsg, LendingError};
//     use anchor_lang::err;
//     use anchor_spl::token::spl_token;
//     use anchor_spl::token_2022::spl_token_2022;
//     use anchor_spl::token_2022::spl_token_2022::extension::confidential_transfer::EncryptedBalance;
//     use anchor_spl::token_interface::spl_token_2022::extension::ExtensionType;
//     use anchor_spl::token_interface::spl_token_2022::extension::{
//         BaseStateWithExtensions, StateWithExtensions,
//     };
//     use bytemuck::Zeroable;
//     use solana_program::account_info::AccountInfo;
//     use solana_program::pubkey::Pubkey;

//     const VALID_LIQUIDITY_TOKEN_EXTENSIONS: &[ExtensionType] = &[
//         ExtensionType::ConfidentialTransferFeeConfig,
//         ExtensionType::ConfidentialTransferMint,
//         ExtensionType::MintCloseAuthority,
//         ExtensionType::MetadataPointer,
//         ExtensionType::PermanentDelegate,
//         ExtensionType::TransferFeeConfig,
//         ExtensionType::TokenMetadata,
//         ExtensionType::TransferHook,
//     ];
// }
