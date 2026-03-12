use crate::{constants::POOL_SEED, states::Pool};
use pinocchio::{
    AccountView,
    cpi::{Seed, Signer},
    error::ProgramError,
};

pub fn create_pool_seed<'a>(
    pool_bump: &'a [u8; 1],
    token_a: &'a [u8; 32],
    token_b: &'a [u8; 32],
) -> [Seed<'a>; 4] {
    [
        Seed::from(POOL_SEED.as_bytes()),
        Seed::from(token_a.as_ref()),
        Seed::from(token_b.as_ref()),
        Seed::from(pool_bump.as_ref()),
    ]
}

pub fn load_pool_data(pool: &AccountView) -> Result<(u8, [u8; 32], [u8; 32]), ProgramError> {
    let pool_data = pool.try_borrow()?;
    let pool_state = Pool::load(&pool_data)?;
    Ok((pool_state.bump, pool_state.token_a, pool_state.token_b))
}

pub fn create_pool_signer<'a, 'b>(pool_seed: &'a [Seed<'b>; 4]) -> Signer<'a, 'b> {
    Signer::from(&pool_seed[..])
}
