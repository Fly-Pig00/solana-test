use anchor_lang::prelude::*;
// use stake_program:: create_account;

#[derive(Account)]
pub struct CreateStake<'info> {
  #[account(
    init_if_needed,
    payer = signer,
    space = Customer::LEN,
    seeds = [
      b"customer", 
      signer.key().as_ref()  
    ],
    bump
  )]
  pub stake_account: Account<'info>,

  #[account(mut)]
  pub signer: Signer<'info>,

  pub system_program: Program<'info, System>
}