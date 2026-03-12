use bytemuck::{Pod, Zeroable};
use pinocchio::{
    AccountView, Address, ProgramResult,
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;

use crate::{
    constants::{LP_MINT_SEED, POOL_SEED, SYSTEM_PROGRAM_ID},
    states::Pool,
};
use pinocchio_token::{
    ID,
    instructions::InitializeMint2,
    state::{Mint, TokenAccount},
};

#[repr(C)]
#[derive(Clone, Debug, Copy, PartialEq, Pod, Zeroable)]
pub struct InitializeInstructionData {
    pub fee_rate: u16,
    pub pool_bump: u8,
    pub lp_mint_bump: u8,
}

impl InitializeInstructionData {
    pub const LEN: usize = core::mem::size_of::<Self>();
}

pub fn process_initialize(
    program_id: &Address,
    accounts: &[AccountView],
    instruction: &[u8],
) -> ProgramResult {
    let [
        authority,
        pool,
        token_a,
        token_b,
        lp_mint,
        vault_a,
        vault_b,
        system_program,
        token_program,
        _remaining @ ..,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !pool.is_data_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if token_a.address() == token_b.address() {
        return Err(ProgramError::InvalidArgument);
    }

    if !lp_mint.is_data_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if system_program.address().as_array() != &SYSTEM_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    if token_program.address() != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let vault_a_account = TokenAccount::from_account_view(vault_a)?;

    let vault_b_account = TokenAccount::from_account_view(vault_b)?;

    if vault_a_account.mint() != token_a.address() {
        return Err(ProgramError::InvalidAccountData);
    }
    if vault_a_account.amount() != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if vault_b_account.mint() != token_b.address() {
        return Err(ProgramError::InvalidAccountData);
    }
    if vault_b_account.amount() != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if instruction.len() != InitializeInstructionData::LEN {
        return Err(ProgramError::InvalidInstructionData);
    }

    let data = bytemuck::checked::pod_read_unaligned::<InitializeInstructionData>(instruction);

    //  - 1 basis point = 0.01%
    //  - 10000 basis points = 100%
    if data.fee_rate > 10000 {
        return Err(ProgramError::InvalidArgument);
    }

    let pool_pda = Address::create_program_address(
        &[
            POOL_SEED.as_bytes(),
            token_a.address().as_ref(),
            token_b.address().as_ref(),
            &[data.pool_bump],
        ],
        program_id,
    )
    .map_err(|_| ProgramError::InvalidSeeds)?;

    if pool.address() != &pool_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    let lp_mint_pda = Address::create_program_address(
        &[
            LP_MINT_SEED.as_bytes(),
            pool.address().as_ref(),
            &[data.lp_mint_bump],
        ],
        program_id,
    )
    .map_err(|_| ProgramError::InvalidSeeds)?;

    if lp_mint.address() != &lp_mint_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    let rent = Rent::get()?;

    let binding = [data.pool_bump];
    let pool_seed = [
        Seed::from(POOL_SEED.as_bytes()),
        Seed::from(token_a.address().as_ref()),
        Seed::from(token_b.address().as_ref()),
        Seed::from(&binding),
    ];

    let pool_seed_signer = Signer::from(&pool_seed[..]);

    (CreateAccount {
        from: authority,
        to: pool,
        space: Pool::LEN as u64,
        lamports: rent
            .try_minimum_balance(Pool::LEN)
            .map_err(|_| ProgramError::InvalidAccountData)?,
        owner: program_id,
    })
    .invoke_signed(&[pool_seed_signer])?;

    let mut pool_data = pool.try_borrow_mut()?;
    let pool_state = Pool::load_mut(&mut pool_data)?;

    pool_state.set_inner_full(Pool {
        authority: *pool.address().as_array(),
        token_a: *token_a.address().as_array(),
        token_b: *token_b.address().as_array(),
        lp_mint: *lp_mint.address().as_array(),
        vault_a: *vault_a.address().as_array(),
        vault_b: *vault_b.address().as_array(),
        reserve_a: 0,
        reserve_b: 0,
        fee_rate: data.fee_rate,
        bump: data.pool_bump,
        lp_mint_bump: data.lp_mint_bump,
        _padding: [0; 4],
    });

    let binding = [data.lp_mint_bump];
    let lp_mint_seed = [
        Seed::from(LP_MINT_SEED.as_bytes()),
        Seed::from(pool.address().as_ref()),
        Seed::from(&binding),
    ];

    (CreateAccount {
        from: authority,
        to: lp_mint,
        space: Mint::LEN as u64,
        lamports: rent
            .try_minimum_balance(Mint::LEN)
            .map_err(|_| ProgramError::InvalidAccountData)?,
        owner: &ID,
    })
    .invoke_signed(&[Signer::from(&lp_mint_seed[..])])?;

    InitializeMint2 {
        mint: lp_mint,
        decimals: 6,
        mint_authority: pool.address(),
        freeze_authority: None,
    }
    .invoke()?;

    Ok(())
}
