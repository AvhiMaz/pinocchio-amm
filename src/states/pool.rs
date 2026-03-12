use bytemuck::{Pod, Zeroable};
use pinocchio::error::ProgramError;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Pool {
    pub authority: [u8; 32],
    pub token_a: [u8; 32],
    pub token_b: [u8; 32],
    pub lp_mint: [u8; 32],
    pub vault_a: [u8; 32],
    pub vault_b: [u8; 32],
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_rate: u16,
    pub bump: u8,
    pub lp_mint_bump: u8,
    pub _padding: [u8; 4],
}

impl Pool {
    pub const LEN: usize = core::mem::size_of::<Self>();

    pub fn set_inner_full(&mut self, args: Pool) {
        *self = args;
    }

    pub fn load_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        bytemuck::try_from_bytes_mut(data).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        bytemuck::try_from_bytes(data).map_err(|_| ProgramError::InvalidAccountData)
    }
}
