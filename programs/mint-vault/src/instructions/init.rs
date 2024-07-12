use anchor_lang::prelude::*;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::{constants::{ADMIN_ADDRESS, SEED_ASSET_MANAGER, SEED_PREFIX}, AssetManager, error::CreateErrorCode, Core, Protocol, SEED_PROTOCOL,  };

/// Initialize AssetManager escrow and Protocol account
///
/// ### Accounts:
/// 
/// 1. `[writeable, signer]` payer
/// 2. `[writeable,]` assetManager
/// 3. `[writeable]` protocol
/// 4. `[writeable]` treasury
/// 5. `[]` core program
/// 6. `[]` `system program`

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut, address = ADMIN_ADDRESS @CreateErrorCode::PubkeyMismatch)]
    pub payer: Signer<'info>,

    #[account(
        init, 
        payer=payer,
        space=AssetManager::LEN,
        seeds = [SEED_PREFIX, SEED_ASSET_MANAGER],
        bump
    )]
    pub asset_manager: Account<'info, AssetManager>,

    #[account(
        init, 
        payer=payer,
        space=Protocol::LEN,
        seeds = [SEED_PREFIX, SEED_PROTOCOL],
        bump
    )]
    pub protocol: Account<'info, Protocol>,

    pub treasury: SystemAccount<'info>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl Init<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Initialize the Asset Manager escrow account
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn init(ctx: Context<Init>) -> Result<()> {
        msg!("initialized escrow account");

        let asset_manager = &mut ctx.accounts.asset_manager;
        asset_manager.bump = ctx.bumps.asset_manager; 

        let protocol = &mut ctx.accounts.protocol;
        protocol.treasury = ctx.accounts.treasury.key();
        protocol.rent = 1 * LAMPORTS_PER_SOL; // ! fixed rental fees

        Ok(())
    }
}