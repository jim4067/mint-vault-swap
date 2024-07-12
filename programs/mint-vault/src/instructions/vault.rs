use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer as SOLTransfer},
};

use mpl_core::instructions::TransferV1CpiBuilder;

use crate::{error::CreateErrorCode, AssetManager, Core, Protocol};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct LockInVaultParams {
    ///  name of our asset
    pub name: String,

    ///  off-chain metadata uri
    pub uri: String,
}

/// Create MPL Core Asset context
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` asset
/// 3. `[writeable]` collection
/// 4. `[writeable]` asset manager
/// 5. `[writeable]` protocol
/// 6. `[]` core program
/// 7. `[]` `system program`
///
#[derive(Accounts)]
pub struct LockAssetInVault<'info> {
    pub payer: Signer<'info>,

    #[account(mut, address = protocol.treasury @CreateErrorCode::PubkeyMismatch)]
    pub treasury: SystemAccount<'info>,

    /// CHECK: we are passing this in ourselves
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    /// CHECK: we are passing this in ourselves
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,

    #[account(mut)]
    pub asset_manager: Account<'info, AssetManager>,

    pub protocol: Account<'info, Protocol>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl LockAssetInVault<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        // when locking asset with program, user must pay rental fee of one sol
        // check if user has required lamports
        let user_lamports = self.payer.lamports();
        if user_lamports < self.protocol.rent {
            return Err(error!(CreateErrorCode::InsufficientLamportsForRent));
        }

        return Ok(());
    }

    /// lock asset in vault
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn lock_asset_in_vault(ctx: Context<LockAssetInVault>) -> Result<()> {
        let protocol = &mut ctx.accounts.protocol;

        // take fee of one sol for locking asset
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let protocol_cpi_accounts = SOLTransfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        };
        let protocol_cpi_context = CpiContext::new(cpi_program.clone(), protocol_cpi_accounts);
        system_program::transfer(protocol_cpi_context, protocol.rent)?;

        TransferV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .collection(Some(&ctx.accounts.collection))
            .payer(&ctx.accounts.payer)
            .authority(Some(&ctx.accounts.payer))
            .new_owner(&ctx.accounts.asset_manager.to_account_info())
            .system_program(Some(&ctx.accounts.system_program))
            .invoke()?;

        Ok(())
    }
}
