pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("xnrMV3UCFqDefZW3oEY4QGVX8fFmopJGETwWDSfCiUd");

#[program]
pub mod swap {
    use super::*;

    /// Swap Tokens for NFTs
    pub fn swap(ctx: Context<Swap>) -> Result<()> {
        Swap::swap(ctx)
    }
}
