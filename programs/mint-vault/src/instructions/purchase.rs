use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer as SOLTransfer},
};

use mpl_core::instructions::TransferV1CpiBuilder;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::{
    constants::{SEED_ASSET_MANAGER, SEED_PREFIX},
    error::CreateErrorCode,
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
/// 9. `[]` system program
///

#[derive(Accounts)]
pub struct Purchase<'info> {
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

    pub system_program: Program<'info, System>,
}

impl Purchase<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        //  using a constant rate of 2 SOL to buy/swap any asset in our vaults
        // check if user has required lamports
        let user_lamports = self.buyer.lamports();
        if user_lamports < self.protocol.rent {
            return Err(error!(CreateErrorCode::InsufficientLamportsForPurchase));
        }

        return Ok(());
    }

    /// buy a MPL core asset listed on the marketplace
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn purchase_asset(ctx: Context<Purchase>) -> Result<()> {
        let asset_manager = &mut ctx.accounts.asset_manager;
        let purchase_fee = 2 * LAMPORTS_PER_SOL;

        // transfer constant purchase fee of one SOL to previous owner
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let protocol_cpi_accounts = SOLTransfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.previous_owner.to_account_info(),
        };
        let protocol_cpi_context = CpiContext::new(cpi_program.clone(), protocol_cpi_accounts);
        system_program::transfer(protocol_cpi_context, purchase_fee)?;

        // transfer to buyer
        let bump = &[asset_manager.bump];
        let signer_seeds = &[&[SEED_PREFIX, SEED_ASSET_MANAGER, bump][..]];

        TransferV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .payer(&ctx.accounts.buyer)
            .collection(Some(&ctx.accounts.collection))
            .authority(Some(&asset_manager.to_account_info()))
            .new_owner(&ctx.accounts.buyer.to_account_info())
            .system_program(Some(&ctx.accounts.system_program))
            .invoke_signed(signer_seeds)?;

        Ok(())
    }
}
