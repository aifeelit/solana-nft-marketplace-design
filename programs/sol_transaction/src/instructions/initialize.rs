use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint};
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub site_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process(
    ctx: Context<Initialize>
) -> ProgramResult {
    // TODO
    Ok(())
}