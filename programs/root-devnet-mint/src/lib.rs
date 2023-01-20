use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod constants;

use instructions::*;

declare_id!("7jzCBubNfhcL5jwxpA7zcA431ssYAGmVxQAnPLL4HKMb");

#[program]
pub mod root_devnet_mint {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        instructions::initialize_mint(ctx)
    }
}
