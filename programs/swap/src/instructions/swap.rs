use anchor_lang::prelude::*;

use mint_vault::{
    cpi::{accounts::Purchase, purchase},
    program::MintVault,
    AssetManager, Core, Protocol,
};

/// Buy a listed MPL core asset on soundwork
///
///  ### Accounts:
///
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` buyer
/// 3. `[writeable]` previous_owner
/// 4. `[writeable]` asset
/// 5. `[writeable]` collection
/// 6. `[]` asset manager
/// 7. `[]` protocol
/// 8. `[]` core program
/// 9. `[]` mint_vault_program
/// 10. `[]` system program
///

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub buyer: SystemAccount<'info>,

    #[account(mut)]
    pub previous_owner: SystemAccount<'info>,

    /// CHECK: checked by us
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    /// CHECK: we are passing this in ourselves
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,

    pub asset_manager: Box<Account<'info, AssetManager>>,

    pub protocol: Box<Account<'info, Protocol>>,

    pub core_program: Program<'info, Core>,

    pub mint_vault_program: Program<'info, MintVault>,

    pub system_program: Program<'info, System>,
}

impl Swap<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// swap nft for sol
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn swap(ctx: Context<Swap>) -> Result<()> {
        let cpi_program = ctx.accounts.mint_vault_program.to_account_info();

        let purchase_cpi_accounts = Purchase {
            payer: ctx.accounts.payer.to_account_info(),
            buyer: ctx.accounts.buyer.to_account_info(),
            previous_owner: ctx.accounts.previous_owner.to_account_info(),
            asset: ctx.accounts.asset.to_account_info(),
            collection: ctx.accounts.collection.to_account_info(),
            asset_manager: ctx.accounts.asset_manager.to_account_info(),
            protocol: ctx.accounts.protocol.to_account_info(),
            core_program: ctx.accounts.core_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let purchase_asset_ctx = CpiContext::new(cpi_program, purchase_cpi_accounts);

        purchase(purchase_asset_ctx)?;

        Ok(())
    }
}
