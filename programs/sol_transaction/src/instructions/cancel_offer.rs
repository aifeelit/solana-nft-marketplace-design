use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint};
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(wallet_authority_bump: u8)]
pub struct CancelOffer<'info> {
    #[account(mut)]
    pub offerer: Signer<'info>,

    #[account(mut)]
    pub site_wallet: AccountInfo<'info>,
    
    #[account(
        seeds = [b"wallet_authority".as_ref()],
        bump = wallet_authority_bump
    )]
    pub wallet_authority: AccountInfo<'info>,

    #[account(
        mut,
        constraint = product_info_account.owner == program_id
    )]
    pub product_info_account: AccountInfo<'info>,

    #[account(
        owner = token::Token::id()
    )]
    pub nft_mint_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process(
    ctx: Context<CancelOffer>,
    offer_id: u16,
    wallet_authority_bump: u8
) -> ProgramResult {
    // TODO
    Ok(())
}