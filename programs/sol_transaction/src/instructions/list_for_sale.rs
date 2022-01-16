use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
#[instruction(vault_authority_bump: u8)]
pub struct ListForSale<'info> {

    #[account(mut, signer)]
    pub seller: AccountInfo<'info>,

    #[account(
        mut,
        constraint = product_info_account.owner == program_id
    )]
    pub product_info_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = nft_token_account.mint == nft_mint_account.key(),
        constraint = nft_token_account.owner == seller.key()
    )]
    pub nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        owner = token::Token::id()
    )]
    pub nft_mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = nft_mint_account,
        associated_token::authority = vault_authority
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"vault_authority".as_ref()],
        bump = vault_authority_bump
    )]
    pub vault_authority: AccountInfo<'info>,

    pub token_program: Program<'info, token::Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}

pub fn process(
    ctx: Context<ListForSale>,
    vault_authority_bump: u8,
) -> ProgramResult {
    // TODO
    Ok(())
}