use crate::state::Details;
use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer as SplTransfer, self}, associated_token::AssociatedToken};
use crate::errors::LaunchpadError;

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

  #[account(
    mut,
    associated_token::mint = token_mint,
    associated_token::authority = creator,
  )]
  pub from_ata: Account<'info, TokenAccount>,

  #[account(
    init,
    payer = creator,
    associated_token::mint = token_mint,
    associated_token::authority = launchpad_details,
  )]
  pub to_ata: Account<'info, TokenAccount>,

  pub token_program: Program<'info, Token>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub system_program: Program<'info, System>
}

pub fn initialize_launchpad_handler(ctx: Context<InitializeLaunchpad>, price: u64, amount: u64, tiers: [u64; 4], soft_cap: u64, hard_cap: u64, launch_time: i64, end_time: i64) -> Result<()> {
  let _sum: u64 = tiers.iter().sum();

  require_eq!(_sum, amount, LaunchpadError::NotMatchTiersSumAmount);

  let token_mint = ctx.accounts.token_mint.key();
  let creator = ctx.accounts.creator.key();

  let launchpad_details = &mut ctx.accounts.launchpad_details;

  // Token Deposit
  let destination = &ctx.accounts.to_ata;
  let source = &ctx.accounts.from_ata;
  let token_program = &ctx.accounts.token_program;
  let authority = &ctx.accounts.creator;

  // Transfer tokens from taker to initializer
  let cpi_accounts = SplTransfer {
    from: source.to_account_info().clone(),
    to: destination.to_account_info().clone(),
    authority: authority.to_account_info().clone(),
  };
  
  let cpi_program = token_program.to_account_info();

  token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

  **launchpad_details = Details::init(
    token_mint, creator, price, amount, tiers, soft_cap, hard_cap, launch_time, end_time
  );
  
  Ok(())
}
