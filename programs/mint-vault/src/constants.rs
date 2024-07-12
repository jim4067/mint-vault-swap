use anchor_lang::{prelude::*, solana_program::pubkey};

#[constant]
pub const SEED_PREFIX: &[u8] = b"anchor";

pub const SEED_COLLECTION_DATA: &[u8] = b"collection";

pub const SEED_ASSET_MANAGER: &[u8] = b"asset manager";

pub const SEED_PROTOCOL: &[u8] = b"protocol";

#[constant]
pub const ADMIN_ADDRESS: Pubkey = pubkey!("4kg8oh3jdNtn7j2wcS7TrUua31AgbLzDVkBZgTAe44aF");
