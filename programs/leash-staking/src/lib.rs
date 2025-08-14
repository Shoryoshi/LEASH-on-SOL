use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;
use state::*;

#[program]
pub mod leash_staking {
    use super::*;

    /// Initialize the staking program
    /// Sets up the global staking state and reward parameters
    pub fn initialize(
        ctx: Context<Initialize>,
        reward_rate: u64,
        min_stake_amount: u64,
        max_stake_amount: u64,
        lock_period: i64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, reward_rate, min_stake_amount, max_stake_amount, lock_period)
    }

    /// Stake LEASH tokens to receive xLEASH
    /// Users can stake their LEASH tokens to start earning rewards
    pub fn stake(
        ctx: Context<Stake>,
        amount: u64,
    ) -> Result<()> {
        instructions::stake::handler(ctx, amount)
    }

    /// Unstake LEASH tokens by burning xLEASH
    /// Users can unstake their tokens and claim accumulated rewards
    pub fn unstake(
        ctx: Context<Unstake>,
        amount: u64,
    ) -> Result<()> {
        instructions::unstake::handler(ctx, amount)
    }

    /// Claim accumulated staking rewards
    /// Users can claim their earned rewards without unstaking
    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
    ) -> Result<()> {
        instructions::claim_rewards::handler(ctx)
    }

    /// Update staking parameters (admin only)
    /// Allows governance to adjust staking parameters
    pub fn update_staking_params(
        ctx: Context<UpdateStakingParams>,
        reward_rate: Option<u64>,
        min_stake_amount: Option<u64>,
        max_stake_amount: Option<u64>,
        lock_period: Option<i64>,
    ) -> Result<()> {
        instructions::update_staking_params::handler(
            ctx,
            reward_rate,
            min_stake_amount,
            max_stake_amount,
            lock_period,
        )
    }

    /// Distribute LP fees to staking rewards
    /// Called by treasury to distribute collected LP fees
    pub fn distribute_lp_fees(
        ctx: Context<DistributeLpFees>,
        amount: u64,
    ) -> Result<()> {
        instructions::distribute_lp_fees::handler(ctx, amount)
    }

    /// Emergency pause staking operations
    /// Allows governance to pause staking in emergency situations
    pub fn emergency_pause(
        ctx: Context<EmergencyPause>,
        pause: bool,
    ) -> Result<()> {
        instructions::emergency_pause::handler(ctx, pause)
    }

    /// Get staking statistics
    /// Returns current staking metrics and state
    pub fn get_staking_stats(
        ctx: Context<GetStakingStats>,
    ) -> Result<StakingStats> {
        instructions::get_staking_stats::handler(ctx)
    }
}

/// Error codes for the staking program
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
