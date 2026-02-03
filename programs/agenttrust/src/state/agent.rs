use anchor_lang::prelude::*;
use crate::{MAX_AGENT_NAME_LEN, MINIMUM_STAKE_LAMPORTS};
use crate::errors::AgentTrustError;

#[account]
pub struct Agent {
    /// Agent's wallet address (owner)
    pub owner: Pubkey,
    
    /// Agent name (max 32 characters)
    pub name: String,
    
    /// Calculated reputation score (0-100)
    pub reputation_score: u64,
    
    /// Number of tasks completed successfully
    pub successful_tasks: u64,
    
    /// Total number of tasks attempted
    pub total_tasks: u64,
    
    /// Total stake in lamports
    pub total_stake: u64,
    
    /// Number of disputes lost
    pub dispute_losses: u64,
    
    /// Unix timestamp when agent registered
    pub created_at: i64,
    
    /// PDA bump seed
    pub bump: u8,
}

impl Agent {
    /// Calculate space needed for Agent account
    /// 8 (discriminator) + 32 (owner) + 4 + 32 (name) + 8*6 (u64 fields) + 8 (created_at) + 1 (bump)
    pub const SPACE: usize = 8 + 32 + 4 + MAX_AGENT_NAME_LEN + 48 + 8 + 1;
    
    /// Register a new agent
    pub fn register(
        &mut self,
        owner: Pubkey,
        name: String,
        initial_stake: u64,
        bump: u8,
    ) -> Result<()> {
        require!(
            name.len() <= MAX_AGENT_NAME_LEN,
            AgentTrustError::NameTooLong
        );
        
        require!(
            initial_stake >= MINIMUM_STAKE_LAMPORTS,
            AgentTrustError::InsufficientStake
        );
        
        self.owner = owner;
        self.name = name;
        self.reputation_score = 50; // Start with neutral score
        self.successful_tasks = 0;
        self.total_tasks = 0;
        self.total_stake = initial_stake;
        self.dispute_losses = 0;
        self.created_at = Clock::get()?.unix_timestamp;
        self.bump = bump;
        
        Ok(())
    }
    
    /// Increase stake amount
    pub fn increase_stake(&mut self, amount: u64) {
        self.total_stake += amount;
    }
    
    /// Decrease stake (for slashing)
    pub fn decrease_stake(&mut self, amount: u64) -> Result<()> {
        require!(
            self.total_stake >= amount,
            AgentTrustError::InsufficientStake
        );
        self.total_stake -= amount;
        Ok(())
    }
    
    /// Record successful task completion
    pub fn record_success(&mut self) {
        self.successful_tasks += 1;
        self.total_tasks += 1;
    }
    
    /// Record task attempt (for disputes or failures)
    pub fn record_attempt(&mut self) {
        self.total_tasks += 1;
    }
    
    /// Record dispute loss
    pub fn record_dispute_loss(&mut self) {
        self.dispute_losses += 1;
    }
    
    /// Calculate reputation score
    /// Formula: (Success% * 0.6) + (NormalizedStake * 0.4) - (DisputeLosses * 10)
    pub fn calculate_reputation(&self) -> u64 {
        // Success rate component (60% weight)
        let success_rate = if self.total_tasks > 0 {
            (self.successful_tasks * 100) / self.total_tasks
        } else {
            0
        };
        let success_component = (success_rate * 60) / 100;
        
        // Stake component (40% weight) - normalized to 0-100
        // Max expected stake: 100 SOL = 100,000,000,000 lamports
        let max_stake = 100_000_000_000u64;
        let stake_normalized = ((self.total_stake.min(max_stake)) * 100) / max_stake;
        let stake_component = (stake_normalized * 40) / 100;
        
        // Dispute penalty: -10 points per loss
        let dispute_penalty = self.dispute_losses * 10;
        
        // Calculate final score with floor at 0
        let raw_score = success_component + stake_component;
        raw_score.saturating_sub(dispute_penalty)
    }
    
    /// Update stored reputation score
    pub fn update_reputation_score(&mut self) {
        self.reputation_score = self.calculate_reputation();
    }
}

/// Seeds for Agent PDA: [b"agent", owner_pubkey]
pub fn get_agent_seeds(owner: &Pubkey) -> Vec<&[u8]> {
    vec![b"agent", owner.as_ref()]
}
