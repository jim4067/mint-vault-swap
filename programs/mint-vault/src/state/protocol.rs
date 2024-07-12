use anchor_lang::prelude::*;

#[account]
pub struct AssetManager {
    /// PDA bump
    pub bump: u8,

    ///  Unused reserved byte space for additive future changes.
    pub _reserved: [u8; 128],
}

impl AssetManager {
    pub const LEN: usize = 
      8 +  // anchor account discriminator 
      1 +  // PDA bump
      128 // reserved space
     ;
}

#[account]
pub struct Protocol {
    /// PDA bump
    pub bump: u8,

    // admin address
    pub treasury: Pubkey,

    // admin address
    pub rent: u64,

    ///  Unused reserved byte space for additive future changes.
    pub _reserved: [u8; 64],
}

impl Protocol {
    pub const LEN: usize = 
      8 +  // anchor account discriminator 
      1 +  // PDA bump
      32 +  // treasury
      8 +  // rental fees
      64 // reserved space
     ;
}