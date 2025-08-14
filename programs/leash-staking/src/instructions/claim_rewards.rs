use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"global_staking_state"],
        bump = global_staking_state.bump,
        has_one = leash_mint,
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
    
    /// User's LEASH token account for receiving rewards
    #[account(
        mut,
        constraint = user_leash_account.mint == leash_mint.key(),
        constraint = user_leash_account.owner == user.key(),
    )]
    pub user_leash_account: Account<'info, TokenAccount>,
    
    /// LEASH token mint
    pub leash_mint: Account<'info, Mint>,
    
    /// Treasury account for distributing rewards
    #[account(
        mut,
        constraint = treasury.key() == global_staking_state.treasury,
    )]
    pub treasury: Account<'info, TokenAccount>,
    
    /// User signing the transaction
    pub user: Signer<'info>,
    
    /// Token program for token operations
    pub token_program: Program<'info, Token>,
}

impl<'info> ClaimRewards<'info> {
    pub fn validate(&self) -> Result<()> {
        // Check if staking is paused
        require!(!self.global_staking_state.is_paused, StakingError::StakingPaused);
        
        // Check if user has any staked tokens
        require!(
            self.user_staking_position.staked_amount > 0,
            StakingError::InsufficientBalance
        );
        
        Ok(())
    }
}

pub fn handler(ctx: Context<ClaimRewards>) -> Result<()> {
    let accounts = &ctx.accounts;
    
    // Validate the claim operation
    accounts.validate()?;
    
    // Update global rewards
    let global_staking_state = &mut accounts.global_staking_state;
    global_staking_state.update_rewards();
    
    // Update user rewards and calculate claimable amount
    let user_position = &mut accounts.user_staking_position;
    user_position.update_rewards(global_staking_state);
    
    let claimable_amount = user_position.pending_rewards;
    
    // Check if there are rewards to claim
    require!(
        claimable_amount > 0,
        StakingError::NoRewardsToClaim
    );
    
    // Transfer rewards from treasury to user
    let transfer_ctx = CpiContext::new(
        accounts.token_program.to_account_info(),
        Transfer {
            from: accounts.treasury.to_account_info(),
            to: accounts.user_leash_account.to_account_info(),
            authority: accounts.treasury.to_account_info(),
        },
    );
    
    token::transfer(transfer_ctx, claimable_amount)?;
    
    // Update user position - mark rewards as claimed
    user_position.claim_rewards(claimable_amount);
    
    // Update global statistics
    global_staking_state.total_rewards_distributed = global_staking_state.total_rewards_distributed
        .checked_add(claimable_amount)
        .ok_or(StakingError::MathOverflow)?;
    
    msg!("Successfully claimed {} LEASH rewards", claimable_amount);
    msg!("Total rewards distributed: {} LEASH", global_staking_state.total_rewards_distributed);
    
    Ok(())
}
