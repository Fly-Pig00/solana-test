use anchor_lang::prelude::*;

#[account]
pub struct Customer {
  // The staked sol amount
  pub staked_amount: u64,
  // The unstaked sol amount
  pub unstaked_amount: u64,
  // The public key
  pub pk: Pubkey,
  // The tier
  pub tier: u8,
}

impl Customer {
  pub const LEN: usize = 8 + 8 + 8 + 32 + 1;

  pub fn init(pk: Pubkey) -> Self {
    Self {
      staked_amount: 0,
      unstaked_amount: 0,
      pk,
      tier: 0
    }
  }
}