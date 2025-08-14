use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, Burn};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        mut,
        seeds = [b"global_staking_state"],
        bump = global_staking_state.bump,
        has_one = leash_mint,
        has_one = xleash_mint,
    )]
    pub global_staking_state: Account<'info, GlobalStakingState>,
    
    /// User's staking position
    #[account(
        mut,
        seeds = [
            b"user_staking_position",
            user.key().as_ref(),
            global_staking_state.key().as_ref()
        ],
        bump = user_staking_position.bump,
        has_one = owner,
        has_one = global_staking_state,
    )]
    pub user_staking_position: Account<'info, UserStakingPosition>,
    
    /// User's LEASH token account
    #[account(
        mut,
        constraint = user_leash_account.mint == leash_mint.key(),
        constraint = user_leash_account.owner == user.key(),
    )]
    pub user_leash_account: Account<'info, TokenAccount>,
    
    /// User's xLEASH token account
    #[account(
        mut,
        constraint = user_xleash_account.mint == xleash_mint.key(),
        constraint = user_xleash_account.owner == user.key(),
    )]
    pub user_xleash_account: Account<'info, TokenAccount>,
    
    /// LEASH token mint
    pub leash_mint: Account<'info, Mint>,
    
    /// xLEASH token mint
    #[account(mut)]
    pub xleash_mint: Account<'info, Mint>,
    
    /// Treasury account for collecting fees
    #[account(
        mut,
        constraint = treasury.key() == global_staking_state.treasury,
    )]
    pub treasury: Account<'info, TokenAccount>,
    
    /// User signing the transaction
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// Token program for token operations
    pub token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    pub fn validate(&self) -> Result<()> {
        // Check if staking is paused
        require!(!self.global_staking_state.is_paused, StakingError::StakingPaused);
        
        // Validate unstake amount
        require!(
            self.amount > 0,
            StakingError::InvalidStakeAmount
        );
        
        // Check if user has sufficient xLEASH
        require!(
            self.user_xleash_account.amount >= self.amount,
            StakingError::InsufficientBalance
        );
        
        // Check if user has sufficient staked amount
        require!(
            self.user_staking_position.staked_amount >= self.amount,
            StakingError::InsufficientBalance
        );
        
        // Check if lock period has been met (if position is locked)
        if self.user_staking_position.is_locked {
            let current_time = Clock::get().unwrap().unix_timestamp;
            require!(
                current_time >= self.user_staking_position.lock_end_time,
                StakingError::LockPeriodNotMet
            );
        }
        
        Ok(())
    }
}

pub fn handler(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    let accounts = &ctx.accounts;
    
    // Validate the unstake operation
    accounts.validate()?;
    
    // Update global rewards before unstaking
    let global_staking_state = &mut accounts.global_staking_state;
    global_staking_state.update_rewards();
    
    // Update user rewards before unstaking
    let user_position = &mut accounts.user_staking_position;
    user_position.update_rewards(global_staking_state);
    
    // Calculate the amount of LEASH to return (1:1 ratio for now)
    let leash_amount = amount;
    
    // Burn xLEASH from user
    let burn_ctx = CpiContext::new(
        accounts.token_program.to_account_info(),
        Burn {
            mint: accounts.xleash_mint.to_account_info(),
            from: accounts.user_xleash_account.to_account_info(),
            authority: accounts.user.to_account_info(),
        },
    );
    
    token::burn(burn_ctx, amount)?;
    
    // Transfer LEASH from treasury to user
    let transfer_ctx = CpiContext::new(
        accounts.token_program.to_account_info(),
        Transfer {
            from: accounts.treasury.to_account_info(),
            to: accounts.user_leash_account.to_account_info(),
            authority: accounts.treasury.to_account_info(),
        },
    );
    
    token::transfer(transfer_ctx, leash_amount)?;
    
    // Update user staking position
    user_position.unstake(amount, amount);
    
    // Update global state
    global_staking_state.total_staked = global_staking_state.total_staked
        .checked_sub(amount)
        .ok_or(StakingError::MathOverflow)?;
    
    global_staking_state.total_xleash_minted = global_staking_state.total_xleash_minted
        .checked_sub(amount)
        .ok_or(StakingError::MathOverflow)?;
    
    msg!("Successfully unstaked {} LEASH tokens", amount);
    msg!("Burned {} xLEASH tokens", amount);
    msg!("Total staked: {} LEASH", global_staking_state.total_staked);
    
    Ok(())
}
