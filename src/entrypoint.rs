use crate::processor::Process;
use solana_program::{
    pubkey::Pubkey,
    account_info::{ AccountInfo },
    entrypoint::{ ProgramResult },
    entrypoint
};

fn program_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {
    Process::process(program_id, accounts, data)
}

entrypoint!(program_instruction);
