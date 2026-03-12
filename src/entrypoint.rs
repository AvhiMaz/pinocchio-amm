#![allow(unexpected_cfgs)]

use pinocchio::{
    AccountView, Address, ProgramResult, default_panic_handler, error::ProgramError, no_allocator,
    program_entrypoint,
};

use crate::instructions::{
    add_liquidity::process_add_liquidity, initializer::process_initialize, swap::process_swap,
    withdraw::process_withdraw,
};

program_entrypoint!(process_instruction);

no_allocator!();

default_panic_handler!();

fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((0, rest)) => process_initialize(program_id, accounts, rest),
        Some((1, rest)) => process_add_liquidity(program_id, accounts, rest),
        Some((2, rest)) => process_swap(program_id, accounts, rest),
        Some((3, rest)) => process_withdraw(program_id, accounts, rest),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
