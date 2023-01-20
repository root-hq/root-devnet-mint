use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token,};

use crate::constants::*;

pub fn initialize_mint(_ctx: Context<InitializeMint>) -> Result<()> {

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

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}