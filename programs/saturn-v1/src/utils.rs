use anchor_lang::prelude::*;
use solana_program::program::{invoke, invoke_signed};

pub fn sol_transfer_user<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    Ok(invoke(&ix, &[source, destination, system_program])?)
}