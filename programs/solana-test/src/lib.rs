use anchor_lang::prelude::*;
mod instructions;
mod state;
use instructions::*;

declare_id!("7sFgZK838pHnfjRVbeYPkjs6YbABTmKVWBhsQ5rmZ35t");

#[program]
pub mod solana_test {

	use super::*;

	pub fn initialize_launchpad(
		ctx: Context<InitializeLaunchpad>,
		price: u64,
		amount: u64,
	) -> Result<()> {
		initialize_launchpad_handler(ctx, price, amount)
	}
}
