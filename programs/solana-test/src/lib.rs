use anchor_lang::prelude::*;
mod errors;
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
        tiers: [u64; 4],
        soft_cap: u64,
        hard_cap: u64,
        launch_time: i64,
        end_time: i64,
    ) -> Result<()> {
        initialize_launchpad_handler(
            ctx,
            price,
            amount,
            tiers,
            soft_cap,
            hard_cap,
            launch_time,
            end_time,
        )
    }
}
