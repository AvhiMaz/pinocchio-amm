use pinocchio::{AccountView, ProgramResult, error::ProgramError};
use pinocchio_token::ID;

pub fn validate_signer(account: &AccountView) -> ProgramResult {
    if !account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

pub fn validate_token_program(token_program: &AccountView) -> ProgramResult {
    if token_program.address() != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

pub fn validate_instruction_length(instruction: &[u8], expected_len: usize) -> ProgramResult {
    if instruction.len() != expected_len {
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(())
}

pub fn validate_pubkey_match(actual: &[u8; 32], expected: &[u8; 32]) -> ProgramResult {
    if actual != expected {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

pub fn validate_non_zero(amount: u64) -> ProgramResult {
    if amount == 0 {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}
