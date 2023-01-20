use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token,};

use crate::state::*;
use crate::constants::*;

pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {

   *ctx.accounts.mint_state = MintState {
        mint_key: ctx.accounts.token_mint.key(),
        authority: ctx.accounts.signer.key(),
    };

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [
            GLOBAL_MINT_AUTHORITY.as_bytes()
        ],
        bump,
        space = 10
    )]
    /// CHECK: Safe because this read-only account only gets used as a constraint
    pub global_mint_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = global_mint_authority,
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [
            token_mint.key().as_ref(),
        ],
        bump,
        payer = signer,
        space = MintState::LEN
    )]
    pub mint_state: Account<'info, MintState>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}