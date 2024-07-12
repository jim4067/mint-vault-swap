pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

#[derive(Clone)]
pub struct Core;

impl anchor_lang::Id for Core {
    fn id() -> Pubkey {
        mpl_core::ID
    }
}

declare_id!("6VVXJ3hHsXn8kFqCWRPT6VeigbGkcHkUZhhopritHdMi");

#[program]
pub mod mint_vault {
    use super::*;

    /// Init protocol config and accounts
    ///
    pub fn init(ctx: Context<Init>) -> Result<()> {
        Init::init(ctx)
    }
    /// Create a MPL Core collection
    ///
    pub fn create_collection(
        ctx: Context<CreateCollection>,
        params: CreateCollectionParams,
    ) -> Result<()> {
        CreateCollection::create_collection(ctx, params)
    }

    /// Create a MPL Core asset from a collection
    ///
    pub fn mint_asset(ctx: Context<MintFromCollection>, params: MintFromColParams) -> Result<()> {
        MintFromCollection::mint_from_collection(ctx, params)
    }

    // lock asset in vault
    pub fn lock_in_vault(ctx: Context<LockAssetInVault>) -> Result<()> {
        LockAssetInVault::lock_asset_in_vault(ctx)
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        Purchase::purchase_asset(ctx)
    }
}
