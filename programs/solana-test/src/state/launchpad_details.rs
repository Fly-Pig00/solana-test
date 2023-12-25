use anchor_lang::prelude::*;

#[account]
pub struct Details {
	/// The status of the launchpad
	pub is_active: bool,
	/// The mint of the token to be sold
	pub token_mint: Pubkey,
	/// The creator of the launchpad
	pub creator: Pubkey,
	/// The price of the token
	pub price: u64,
	/// The total amount of the token
	pub amount: u64,
}

impl Details {
	pub const LEN: usize = 8 + 1 + 32 + 8;

	pub fn init(token_mint: Pubkey, creator: Pubkey, price: u64, amount: u64) -> Self {
		Self {
			is_active: true,
			token_mint,
			creator,
			price,
			amount,
		}
	}

	pub fn close_launchpad(&mut self) -> Result<()> {
		self.is_active = false;
		Ok(())
	}
}
