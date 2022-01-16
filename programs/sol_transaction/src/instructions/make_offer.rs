use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint};
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub offerer: Signer<'info>,

    #[account(mut)]
    pub site_wallet: AccountInfo<'info>,

    #[account(
        mut,
        constraint = product_info_account.owner == program_id
    )]
    pub product_info_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = offer_nft_token_account.mint == *nft_mint_account.key,
        constraint = offer_nft_token_account.owner == offerer.key()
    )]
    pub offer_nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        owner = token::Token::id()
    )]
    pub nft_mint_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process(
    ctx: Context<MakeOffer>,
    offer_price: u64
) -> ProgramResult {
    // TODO
    Ok(())
}