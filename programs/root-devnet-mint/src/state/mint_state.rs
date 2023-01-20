use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct MintState {
    pub mint_key: Pubkey, // 32
    pub authority: Pubkey, // 32
}

impl MintState {
    pub const LEN: usize = 8 + (2 * 32);
}