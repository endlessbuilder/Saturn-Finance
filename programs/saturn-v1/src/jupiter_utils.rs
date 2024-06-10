use anchor_lang::{
    prelude::*,
    solana_program::{entrypoint::ProgramResult, instruction::Instruction, program::{invoke, invoke_signed}},
    system_program,
};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{constants::{TREASURY_AUTHORITY_SEED, TREASURY_SEED}, error::*};
use crate::account::*;

pub fn sol_transfer_user<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    Ok(invoke(&ix, &[source, destination, system_program])?)
}

pub const WSOL_SEED: &[u8] = b"wsol";

mod jupiter {
    use anchor_lang::declare_id;
    declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
}

#[derive(Clone)]
pub struct Jupiter;

impl anchor_lang::Id for Jupiter {
    fn id() -> Pubkey {
        jupiter::id()
    }
}


pub fn swap_on_jupiter<'info>(
    remaining_accounts: &[AccountInfo],
    jupiter_program: Program<'info, Jupiter>,
    data: Vec<u8>,
    signer_seeds: &[&[&[u8]]],
    treasury_authority: &Pubkey,
) -> ProgramResult {
    
    let accounts: Vec<AccountMeta> = remaining_accounts
        .iter()
        .map(|acc| AccountMeta {
            pubkey: *acc.key,
            is_signer: if acc.key == treasury_authority {true} else {acc.is_signer},
            is_writable: acc.is_writable,
        })
        .collect();

    let accounts_infos: Vec<AccountInfo> = remaining_accounts
        .iter()
        .map(|acc| AccountInfo { ..acc.clone() })
        .collect();

    // TODO: Check the first 8 bytes. Only Jupiter Route CPI allowed.

    invoke_signed(
        &Instruction {
            program_id: *jupiter_program.key,
            accounts,
            data,
        },
        &accounts_infos,
        signer_seeds,
    )
}

pub fn create_wsol_token_idempotent<'info>(
    treasury_authority: UncheckedAccount<'info>,
    treasury_wsol_account: UncheckedAccount<'info>,
    sol_mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    authority_bump: u8,
    wsol_bump: u8,
    from_amount: u64
) -> Result<TokenAccount> {
    if treasury_wsol_account.data_is_empty() {
        let signer_seeds: &[&[&[u8]]] = &[
            &[TREASURY_AUTHORITY_SEED.as_bytes(), &[authority_bump]],
            &[WSOL_SEED, &[wsol_bump]],
        ];

        msg!("Initialize program wSOL account");
        let rent = Rent::get()?;
        let space = TokenAccount::LEN;
        let lamports = rent.minimum_balance(space) + from_amount;
        system_program::create_account(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                system_program::CreateAccount {
                    from: treasury_authority.to_account_info(),
                    to: treasury_wsol_account.to_account_info(),
                },
                signer_seeds,
            ),
            lamports,
            space as u64,
            token_program.key,
        )?;

        msg!("Initialize program wSOL token account");
        token::initialize_account3(CpiContext::new(
            token_program.to_account_info(),
            token::InitializeAccount3 {
                account: treasury_wsol_account.to_account_info(),
                mint: sol_mint.to_account_info(),
                authority: treasury_authority.to_account_info(),
            },
        ))?;

        let data = treasury_wsol_account.try_borrow_data()?;
        let wsol_token_account = TokenAccount::try_deserialize(&mut data.as_ref())?;

        Ok(wsol_token_account)
    } else {
        let data = treasury_wsol_account.try_borrow_data()?;
        let wsol_token_account = TokenAccount::try_deserialize(&mut data.as_ref())?;
        if &wsol_token_account.owner != treasury_authority.key {
            // TODO: throw error
            return err!(BondError::IncorrectOwner);
        }

        Ok(wsol_token_account)
    }
}

pub fn close_treasury_wsol<'info>(
    treasury_authority: UncheckedAccount<'info>,
    treasury_wsol_account: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    authority_bump: &[u8],
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[TREASURY_AUTHORITY_SEED.as_ref(), authority_bump.as_ref()]];

    msg!("Close program wSOL token account");
    token::close_account(CpiContext::new_with_signer(
        token_program.to_account_info(),
        token::CloseAccount {
            account: treasury_wsol_account.to_account_info(),
            destination: treasury_authority.to_account_info(),
            authority: treasury_authority.to_account_info(),
        },
        signer_seeds,
    ))
}

