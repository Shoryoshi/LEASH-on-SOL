use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

/// Global staking state account
/// Stores program-wide staking parameters and statistics
#[account]
pub struct GlobalStakingState {
    /// Authority that can update staking parameters
    pub authority: Pubkey,
    
    /// LEASH token mint
    pub leash_mint: Pubkey,
    
    /// xLEASH token mint (staking receipt token)
    pub xleash_mint: Pubkey,
    
    /// Treasury account for collecting fees
    pub treasury: Pubkey,
    
    /// Current reward rate (rewards per second per staked token)
    pub reward_rate: u64,
    
    /// Minimum stake amount
    pub min_stake_amount: u64,
    
    /// Maximum stake amount
    pub max_stake_amount: u64,
    
    /// Lock period for staked tokens (in seconds)
    pub lock_period: i64,
    
    /// Total amount of LEASH currently staked
    pub total_staked: u64,
    
    /// Total amount of xLEASH minted
    pub total_xleash_minted: u64,
    
    /// Accumulated rewards per token (scaled by 1e18)
    pub accumulated_rewards_per_token: u128,
    
    /// Last time rewards were updated
    pub last_update_time: i64,
    
    /// Total rewards distributed
    pub total_rewards_distributed: u64,
    
    /// LP fees collected for distribution
    pub lp_fees_collected: u64,
    
    /// Whether staking is paused
    pub is_paused: bool,
    
    /// Bump seed for PDA derivation
    pub bump: u8,
    
    /// Reserved space for future upgrades
    pub reserved: [u8; 64],
}

impl GlobalStakingState {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 16 + 8 + 8 + 8 + 1 + 1 + 64;
    
    /// Initialize a new global staking state
    pub fn initialize(
        &mut self,
        authority: Pubkey,
        leash_mint: Pubkey,
        xleash_mint: Pubkey,
        treasury: Pubkey,
        reward_rate: u64,
        min_stake_amount: u64,
        max_stake_amount: u64,
        lock_period: i64,
        bump: u8,
    ) {
        self.authority = authority;
        self.leash_mint = leash_mint;
        self.xleash_mint = xleash_mint;
        self.treasury = treasury;
        self.reward_rate = reward_rate;
        self.min_stake_amount = min_stake_amount;
        self.max_stake_amount = max_stake_amount;
        self.lock_period = lock_period;
        self.total_staked = 0;
        self.total_xleash_minted = 0;
        self.accumulated_rewards_per_token = 0;
        self.last_update_time = Clock::get().unwrap().unix_timestamp;
        self.total_rewards_distributed = 0;
        self.lp_fees_collected = 0;
        self.is_paused = false;
        self.bump = bump;
        self.reserved = [0; 64];
    }
    
    /// Update accumulated rewards per token
    pub fn update_rewards(&mut self) {
        let current_time = Clock::get().unwrap().unix_timestamp;
        let time_diff = current_time - self.last_update_time;
        
        if time_diff > 0 && self.total_staked > 0 {
            let rewards = (self.reward_rate as u128)
                .checked_mul(time_diff as u128)
                .unwrap_or(0);
            
            self.accumulated_rewards_per_token = self.accumulated_rewards_per_token
                .checked_add(rewards)
                .unwrap_or(0);
        }
        
        self.last_update_time = current_time;
    }
    
    /// Add LP fees to the collection
    pub fn add_lp_fees(&mut self, amount: u64) {
        self.lp_fees_collected = self.lp_fees_collected.checked_add(amount).unwrap_or(0);
    }
}

/// User staking position account
/// Stores individual user staking data and rewards
#[account]
pub struct UserStakingPosition {
    /// Owner of this staking position
    pub owner: Pubkey,
    
    /// Global staking state this position belongs to
    pub global_staking_state: Pubkey,
    
    /// Amount of LEASH staked
    pub staked_amount: u64,
    
    /// Amount of xLEASH held
    pub xleash_amount: u64,
    
    /// Accumulated rewards per token when user last staked/unstaked
    pub user_rewards_per_token: u128,
    
    /// Pending rewards for this user
    pub pending_rewards: u64,
    
    /// Timestamp when user first staked
    pub staking_start_time: i64,
    
    /// Last time user claimed rewards
    pub last_claim_time: i64,
    
    /// Whether this position is locked
    pub is_locked: bool,
    
    /// Lock end time (if locked)
    pub lock_end_time: i64,
    
    /// Bump seed for PDA derivation
    pub bump: u8,
    
    /// Reserved space for future upgrades
    pub reserved: [u8; 32],
}

impl UserStakingPosition {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 16 + 8 + 8 + 8 + 1 + 8 + 1 + 32;
    
    /// Initialize a new user staking position
    pub fn initialize(
        &mut self,
        owner: Pubkey,
        global_staking_state: Pubkey,
        bump: u8,
    ) {
        self.owner = owner;
        self.global_staking_state = global_staking_state;
        self.staked_amount = 0;
        self.xleash_amount = 0;
        self.user_rewards_per_token = 0;
        self.pending_rewards = 0;
        self.staking_start_time = Clock::get().unwrap().unix_timestamp;
        self.last_claim_time = Clock::get().unwrap().unix_timestamp;
        self.is_locked = false;
        self.lock_end_time = 0;
        self.bump = bump;
        self.reserved = [0; 32];
    }
    
    /// Calculate pending rewards for this user
    pub fn calculate_pending_rewards(&self, global_state: &GlobalStakingState) -> u64 {
        let rewards_per_token_diff = global_state.accumulated_rewards_per_token
            .checked_sub(self.user_rewards_per_token)
            .unwrap_or(0);
        
        let pending = (self.staked_amount as u128)
            .checked_mul(rewards_per_token_diff)
            .unwrap_or(0);
        
        pending.checked_add(self.pending_rewards as u128).unwrap_or(0) as u64
    }
    
    /// Update user rewards
    pub fn update_rewards(&mut self, global_state: &GlobalStakingState) {
        let pending = self.calculate_pending_rewards(global_state);
        self.pending_rewards = pending;
        self.user_rewards_per_token = global_state.accumulated_rewards_per_token;
    }
    
    /// Stake tokens
    pub fn stake(&mut self, amount: u64, xleash_amount: u64) {
        self.staked_amount = self.staked_amount.checked_add(amount).unwrap_or(0);
        self.xleash_amount = self.xleash_amount.checked_add(xleash_amount).unwrap_or(0);
        self.staking_start_time = Clock::get().unwrap().unix_timestamp;
    }
    
    /// Unstake tokens
    pub fn unstake(&mut self, amount: u64, xleash_amount: u64) {
        self.staked_amount = self.staked_amount.checked_sub(amount).unwrap_or(0);
        self.xleash_amount = self.xleash_amount.checked_sub(xleash_amount).unwrap_or(0);
    }
    
    /// Claim rewards
    pub fn claim_rewards(&mut self, amount: u64) {
        self.pending_rewards = self.pending_rewards.checked_sub(amount).unwrap_or(0);
        self.last_claim_time = Clock::get().unwrap().unix_timestamp;
    }
}

/// Staking statistics for queries
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct StakingStats {
    /// Total amount staked
    pub total_staked: u64,
    
    /// Total xLEASH minted
    pub total_xleash_minted: u64,
    
    /// Current reward rate
    pub reward_rate: u64,
    
    /// Accumulated rewards per token
    pub accumulated_rewards_per_token: u128,
    
    /// Total rewards distributed
    pub total_rewards_distributed: u64,
    
    /// LP fees collected
    pub lp_fees_collected: u64,
    
    /// Number of active stakers
    pub active_stakers: u32,
    
    /// Whether staking is paused
    pub is_paused: bool,
    
    /// Current timestamp
    pub current_time: i64,
}
