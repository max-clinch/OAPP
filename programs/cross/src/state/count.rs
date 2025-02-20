use anchor_lang::prelude::*;

#[account]
pub struct Count {
    pub id: u8,                  // Unique identifier
    pub admin: Pubkey,           // Address of the contract admin
    pub count: u64,              // Tracks the number of received messages
    pub composed_count: u64,     // Tracks the number of composed messages
    pub bump: u8,                // PDA bump seed
    pub endpoint_program: Pubkey // LayerZero Endpoint Program ID
}

impl Count {
    pub const SIZE: usize = 8 + std::mem::size_of::<Self>();
}
