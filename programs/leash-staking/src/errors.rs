use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Invalid stake amount")]
    InvalidStakeAmount,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Staking is paused")]
    StakingPaused,
    #[msg("Lock period not met")]
    LockPeriodNotMet,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Invalid reward rate")]
    InvalidRewardRate,
    #[msg("Staking pool is full")]
    StakingPoolFull,
    #[msg("No rewards to claim")]
    NoRewardsToClaim,
    #[msg("Invalid LP fee distribution")]
    InvalidLpFeeDistribution,
    #[msg("Math overflow")]
    MathOverflow,
}
