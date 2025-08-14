use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, MintTo};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct Stake<'info> {
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
        init_if_needed,
        payer = user,
        space = UserStakingPosition::LEN,
        seeds = [
            b"user_staking_position",
            user.key().as_ref(),
            global_staking_state.key().as_ref()
        ],
        bump
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
    
    /// System program for account creation
    pub system_program: Program<'info, System>,
    
    /// Token program for token operations
    pub token_program: Program<'info, Token>,
    
    /// Associated token program
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    
    /// Rent sysvar for account creation
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Stake<'info> {
    pub fn validate(&self) -> Result<()> {
        // Check if staking is paused
        require!(!self.global_staking_state.is_paused, StakingError::StakingPaused);
        
        // Validate stake amount
        require!(
            self.amount >= self.global_staking_state.min_stake_amount,
            StakingError::InvalidStakeAmount
        );
        
        require!(
            self.amount <= self.global_staking_state.max_stake_amount,
            StakingError::InvalidStakeAmount
        );
        
        // Check if user has sufficient balance
        require!(
            self.user_leash_account.amount >= self.amount,
            StakingError::InsufficientBalance
        );
        
        // Check if staking pool has capacity
        let new_total = self.global_staking_state.total_staked
            .checked_add(self.amount)
            .ok_or(StakingError::MathOverflow)?;
        
        require!(
            new_total <= self.global_staking_state.max_stake_amount,
            StakingError::StakingPoolFull
        );
        
        Ok(())
    }
}

pub fn handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let accounts = &ctx.accounts;
    
    // Validate the stake operation
    accounts.validate()?;
    
    // Update global rewards before staking
    let global_staking_state = &mut accounts.global_staking_state;
    global_staking_state.update_rewards();
    
    // Calculate xLEASH amount to mint (1:1 ratio for now, can be adjusted)
    let xleash_amount = amount;
    
    // Transfer LEASH from user to treasury
    let transfer_ctx = CpiContext::new(
        accounts.token_program.to_account_info(),
        Transfer {
            from: accounts.user_leash_account.to_account_info(),
            to: accounts.treasury.to_account_info(),
            authority: accounts.user.to_account_info(),
        },
    );
    
    token::transfer(transfer_ctx, amount)?;
    
    // Mint xLEASH to user
    let mint_ctx = CpiContext::new(
        accounts.token_program.to_account_info(),
        MintTo {
            mint: accounts.xleash_mint.to_account_info(),
            to: accounts.user_xleash_account.to_account_info(),
            authority: accounts.treasury.to_account_info(),
        },
    );
    
    token::mint_to(mint_ctx, xleash_amount)?;
    
    // Update user staking position
    let user_position = &mut accounts.user_staking_position;
    
    // If this is a new position, initialize it
    if user_position.owner == Pubkey::default() {
        let bump = *ctx.bumps.get("user_staking_position").unwrap();
        user_position.initialize(
            accounts.user.key(),
            accounts.global_staking_state.key(),
            bump,
        );
    }
    
    // Update rewards before staking
    user_position.update_rewards(global_staking_state);
    
    // Add to staked amount
    user_position.stake(amount, xleash_amount);
    
    // Update global state
    global_staking_state.total_staked = global_staking_state.total_staked
        .checked_add(amount)
        .ok_or(StakingError::MathOverflow)?;
    
    global_staking_state.total_xleash_minted = global_staking_state.total_xleash_minted
        .checked_add(xleash_amount)
        .ok_or(StakingError::MathOverflow)?;
    
    msg!("Successfully staked {} LEASH tokens", amount);
    msg!("Received {} xLEASH tokens", xleash_amount);
    msg!("Total staked: {} LEASH", global_staking_state.total_staked);
    
    Ok(())
}
