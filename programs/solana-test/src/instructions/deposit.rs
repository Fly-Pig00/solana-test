use anchor_lang::prelude::*;
use crate::state::Customer;

#[derive(Account)]
pub struct Deposit<'info> {
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
  pub customer: Account<'info, Customer>,

  #[account(mut)]
  pub signer: Signer<'info>,

  pub system_program: Program<'info, System>
}