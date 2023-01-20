use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, Token, TokenAccount}, associated_token::AssociatedToken};

use crate::constants::*;

pub fn mint_tokens(ctx: Context<MintTokens>, qty: u64) -> Result<()> {

    let seeds = &[GLOBAL_MINT_AUTHORITY.as_bytes(), &[*ctx.bumps.get("global_mint_authority").unwrap()]];
    let signer_seeds = &[&seeds[..]];

    // Mint tokens to recipient
    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.token_mint.to_account_info().clone(),
                to: ctx.accounts.signer_token_ata.to_account_info().clone(),
                authority: ctx.accounts.global_mint_authority.to_account_info().clone(),
            },
            signer_seeds,
        ),
        qty * 10u64.pow(ctx.accounts.token_mint.decimals as u32),
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct MintTokens<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [
            GLOBAL_MINT_AUTHORITY.as_bytes()
        ],
        bump,
    )]
    /// CHECK: Safe because this read-only account only gets used as a constraint
    pub global_mint_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = token_mint.mint_authority.unwrap() == global_mint_authority.key()
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = signer,
        constraint = signer_token_ata.mint == token_mint.key(),
    )]
    pub signer_token_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}