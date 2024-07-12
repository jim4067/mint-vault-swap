use anchor_lang::prelude::*;
use mpl_core::instructions::CreateV1CpiBuilder;

use crate::{error::CreateErrorCode, CollectionData, Core};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintFromColParams {
    ///  name of our asset
    pub name: String,

    ///  off-chain metadata uri
    pub uri: String,
}

/// Create MPL Core Asset context
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable, signer]` asset
/// 3. `[writeable]` collection
/// 4. `[writeable]` collection_data
/// 5. `[]` core program
/// 6. `[]` `system program`
///
/// ### Parameters
///
/// 1. params: [MintFromColParams]
///
#[derive(Accounts)]
#[instruction(params: MintFromColParams)]
pub struct MintFromCollection<'info> {
    pub payer: Signer<'info>,

    /// CHECK: we are passing this in ourselves
    #[account(mut, signer)]
    pub asset: UncheckedAccount<'info>,

    /// CHECK: we are passing this in ourselves
    #[account(
        mut,
        address = collection_data.collection @CreateErrorCode::PubkeyMismatch
    )]
    pub collection: UncheckedAccount<'info>,

    /// CHECK: we are passing this in ourselves
    #[account(mut)]
    pub collection_data: Account<'info, CollectionData>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl MintFromCollection<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        // collection contains items to be minted from
        if self.collection_data.items_available == 0 {
            return Err(error!(CreateErrorCode::CollectionMintedOut));
        }

        return Ok(());
    }

    /// CPI into mpl_core program and mint our asset.
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn mint_from_collection(
        ctx: Context<MintFromCollection>,
        params: MintFromColParams,
    ) -> Result<()> {
        let collection_data = &mut ctx.accounts.collection_data;

        collection_data.items_available -= 1;

        CreateV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .collection(Some(&ctx.accounts.collection))
            .authority(Some(&ctx.accounts.payer))
            .payer(&ctx.accounts.payer)
            .owner(Some(&ctx.accounts.payer))
            .system_program(&ctx.accounts.system_program)
            .name(params.name)
            .uri(params.uri)
            .invoke()?;

        Ok(())
    }
}
