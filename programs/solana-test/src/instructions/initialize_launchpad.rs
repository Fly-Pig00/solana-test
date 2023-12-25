use crate::state::{Details, launchpad_details};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct InitializeLaunchpad<'info> {
  #[account(
    init, 
    payer = creator, 
    space = Details::LEN,
    seeds = [
      b"launchpad", 
      token_mint.key().as_ref(),
      creator.key().as_ref()  
    ],
    bump
  )]
  pub launchpad_details: Account<'info, Details>,

  pub token_mint: Account<'info, Mint>,

  #[account(mut)]
  pub creator: Signer<'info>,

  pub system_program: Program<'info, System>
}

pub fn initialize_launchpad_handler(ctx: Context<InitializeLaunchpad>, price: u64, amount: u64) -> Result<()> {
  
  let token_mint = ctx.accounts.token_mint.key();
  let creator = ctx.accounts.creator.key();

  let launchpad_details = &mut ctx.accounts.launchpad_details;

  **launchpad_details = Details::init(
    token_mint, creator, price, amount
  );
  
  Ok(())
}
