use anchor_lang::prelude::*;
use mpl_core::instructions::CreateCollectionV1CpiBuilder;

use crate::{
    constants::{SEED_COLLECTION_DATA, SEED_PREFIX},
    CollectionData, Core,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionParams {
    ///  name of our collection
    pub name: String,

    ///  off-chain metadata uri
    pub uri: String,

    /// no. of items collection should hold
    pub items: u32,
}

/// Create MPL Core collection where our
///
/// ### Accounts:
///
/// 1. `[writeable, signer]` payer
/// 2. `[writeable, signer]` collection
/// 3. `[writeable]` collection_data
/// 4. `[]` core program
/// 5. `[]` `system program`
///
/// ### Parameters
///
/// 1. params: [CreateCollectionParams]
///
#[derive(Accounts)]
#[instruction(params: CreateCollectionParams)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: we are passing this in ourselves
    #[account(mut, signer)]
    pub collection: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = CollectionData::LEN,
        seeds = [SEED_PREFIX, SEED_COLLECTION_DATA, collection.key().as_ref()],
        bump
    )]
    pub collection_data: Account<'info, CollectionData>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl CreateCollection<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Mint a collection asset.
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn create_collection(
        ctx: Context<CreateCollection>,
        params: CreateCollectionParams,
    ) -> Result<()> {
        // update our collection data
        let collection_data = &mut ctx.accounts.collection_data;

        **collection_data = CollectionData::new(
            ctx.bumps.collection_data,
            params.items,
            ctx.accounts.payer.key(),
            ctx.accounts.collection.key(),
        );

        //CPI into mpl_core program and create collection
        CreateCollectionV1CpiBuilder::new(&ctx.accounts.core_program)
            .collection(&ctx.accounts.collection)
            .payer(&ctx.accounts.payer)
            .update_authority(Some(&ctx.accounts.payer))
            .system_program(&ctx.accounts.system_program)
            .name(params.name)
            .uri(params.uri)
            .invoke()?;

        Ok(())
    }
}
