use anchor_lang::prelude::*;

#[account]
pub struct CollectionData {
    /// PDA bump
    pub bump: u8,

    /// no. of items that have yet to be minted from collection
    pub items_available: u32,

    /// asset owner
    pub authority: Pubkey,

    /// collection address
    pub collection: Pubkey,

    ///  Unused reserved byte space for additive future changes.
    pub _reserved: [u8; 64],
}

impl CollectionData {
    pub const LEN: usize = 8 // anchor discriminator 
    + 1 // bump 
    + 4 // items available
    + 32 // authority 
    + 32 // collection
    + 64; // reserved

    /// instantiate the bid data account with provided args
    pub fn new(bump: u8, items_available: u32, authority: Pubkey, collection: Pubkey) -> Self {
        Self {
            bump,
            items_available,
            authority,
            collection,
            _reserved: [0; 64],
        }
    }
}
