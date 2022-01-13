use anchor_lang::prelude::*;

declare_id!("GDjZMBgXhMciUm7o6w24LTWX9a56oqnkBe3YXiK8P3se");

pub mod errors;
pub mod state;
pub mod utils;
pub mod instructions;

use instructions::*;

#[program]
pub mod sol_transaction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        instructions::initialize::process(ctx)
    }    
    
    pub fn list_for_sale(
        ctx: Context<ListForSale>, 
        vault_authority_bump: u8
    ) -> ProgramResult {
        instructions::list_for_sale::process(ctx, vault_authority_bump)
    }

    pub fn cancel_list(
        ctx: Context<CancelList>,
        vault_authority_bump: u8,
    ) -> ProgramResult {
        instructions::cancel_list::process(ctx, vault_authority_bump)
    }

    pub fn make_offer(
        ctx: Context<MakeOffer>, 
        offer_price: u64
    ) -> ProgramResult {
        instructions::make_offer::process(ctx, offer_price)
    }

    pub fn accept_offer(
        ctx: Context<AcceptOffer>,
        offer_id: u16,
        wallet_authority_bump: u8,
        vault_authority_bump: u8
    ) -> ProgramResult {
        instructions::accept_offer::process(ctx, offer_id, wallet_authority_bump, vault_authority_bump)
    }

    pub fn cancel_offer(
        ctx: Context<CancelOffer>,
        offer_id: u16,
        wallet_authority_bump: u8
    ) -> ProgramResult {
        instructions::cancel_offer::process(ctx, offer_id, wallet_authority_bump)
    }
}
