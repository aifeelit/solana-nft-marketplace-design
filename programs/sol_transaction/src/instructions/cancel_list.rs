use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint};
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(vault_authority_bump: u8)]
pub struct CancelList<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        constraint = product_info_account.owner == program_id
    )]
    pub product_info_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = nft_token_account.mint == *nft_mint_account.key,
        constraint = nft_token_account.owner == seller.key()
    )]
    pub nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        owner = token::Token::id()
    )]
    pub nft_mint_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = vault_token_account.mint == *nft_mint_account.key
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"vault_authority".as_ref()],
        bump = vault_authority_bump
    )]
    pub vault_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process(
    ctx: Context<CancelList>,
    vault_authority_bump: u8,
) -> ProgramResult {
    // TODO
    Ok(())
}