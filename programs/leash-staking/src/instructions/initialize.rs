use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = GlobalStakingState::LEN,
        seeds = [b"global_staking_state"],
        bump
    )]
    pub global_staking_state: Account<'info, GlobalStakingState>,
    
    /// LEASH token mint
    pub leash_mint: Account<'info, Mint>,
    
    /// xLEASH token mint (staking receipt token)
    pub xleash_mint: Account<'info, Mint>,
    
    /// Treasury account for collecting fees
    /// CHECK: This is the treasury account
    pub treasury: UncheckedAccount<'info>,
    
    /// Authority that can update staking parameters
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// System program for account creation
    pub system_program: Program<'info, System>,
    
    /// Token program for token operations
    pub token_program: Program<'info, Token>,
    
    /// Rent sysvar for account creation
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn validate(&self) -> Result<()> {
        // Validate reward rate is reasonable (not too high to prevent overflow)
        require!(
            self.reward_rate <= 1_000_000_000, // Max 1 token per second per staked token
            StakingError::InvalidRewardRate
        );
        
        // Validate stake amounts
        require!(
            self.min_stake_amount > 0,
            StakingError::InvalidStakeAmount
        );
        
        require!(
            self.max_stake_amount >= self.min_stake_amount,
            StakingError::InvalidStakeAmount
        );
        
        // Validate lock period (minimum 1 day, maximum 4 years)
        require!(
            self.lock_period >= 86400 && self.lock_period <= 126144000,
            StakingError::InvalidStakeAmount
        );
        
        Ok(())
    }
}

pub fn handler(
    ctx: Context<Initialize>,
    reward_rate: u64,
    min_stake_amount: u64,
    max_stake_amount: u64,
    lock_period: i64,
) -> Result<()> {
    let accounts = &ctx.accounts;
    
    // Validate inputs
    accounts.validate()?;
    
    // Get bump seed
    let bump = *ctx.bumps.get("global_staking_state").unwrap();
    
    // Initialize global staking state
    let global_staking_state = &mut accounts.global_staking_state;
    global_staking_state.initialize(
        accounts.authority.key(),
        accounts.leash_mint.key(),
        accounts.xleash_mint.key(),
        accounts.treasury.key(),
        reward_rate,
        min_stake_amount,
        max_stake_amount,
        lock_period,
        bump,
    );
    
    msg!("Staking program initialized successfully");
    msg!("Reward rate: {} tokens per second per staked token", reward_rate);
    msg!("Min stake amount: {}", min_stake_amount);
    msg!("Max stake amount: {}", max_stake_amount);
    msg!("Lock period: {} seconds", lock_period);
    
    Ok(())
}
