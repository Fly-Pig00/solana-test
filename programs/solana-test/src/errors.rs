use anchor_lang::error_code;

#[error_code]
pub enum LaunchpadError {
  #[msg("The tiers sum must be amount.")]
  NotMatchTiersSumAmount,
}