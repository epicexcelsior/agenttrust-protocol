use anchor_lang::prelude::*;
use crate::{TaskStatus, MAX_TASK_TITLE_LEN};
use crate::errors::AgentTrustError;

#[account]
pub struct Task {
    /// Client who created the task
    pub client: Pubkey,
    
    /// Agent assigned to task (None until claimed)
    pub agent: Option<Pubkey>,
    
    /// Task title (max 64 characters)
    pub title: String,
    
    /// IPFS hash of full description (32 bytes)
    pub description_hash: [u8; 32],
    
    /// Bounty amount in lamports (locked in escrow)
    pub bounty: u64,
    
    /// Deadline as Unix timestamp
    pub deadline: i64,
    
    /// Current task status
    pub status: TaskStatus,
    
    /// IPFS hash of deliverables (None until submitted)
    pub deliverable_hash: Option<[u8; 32]>,
    
    /// Unix timestamp when task created
    pub created_at: i64,
    
    /// Unix timestamp when task claimed (None until claimed)
    pub claimed_at: Option<i64>,
    
    /// Unix timestamp when deliverables submitted (None until submitted)
    pub submitted_at: Option<i64>,
    
    /// PDA bump seed
    pub bump: u8,
}

impl Task {
    /// Calculate space needed for Task account
    /// 8 (discriminator) + 32 (client) + 1 + 32 (agent option) + 4 + 64 (title) + 32 (desc hash) + 
    /// 8 (bounty) + 8 (deadline) + 1 (status) + 1 + 32 (deliverable option) + 8 (created) + 
    /// 1 + 8 (claimed option) + 1 + 8 (submitted option) + 1 (bump)
    pub const SPACE: usize = 8 + 32 + 33 + 4 + MAX_TASK_TITLE_LEN + 32 + 8 + 8 + 1 + 33 + 8 + 9 + 9 + 1;
    
    /// Create a new task
    pub fn create(
        &mut self,
        client: Pubkey,
        title: String,
        description_hash: [u8; 32],
        bounty: u64,
        deadline: i64,
        bump: u8,
    ) -> Result<()> {
        require!(
            title.len() <= MAX_TASK_TITLE_LEN,
            AgentTrustError::TitleTooLong
        );
        
        require!(
            bounty > 0,
            AgentTrustError::InvalidBounty
        );
        
        require!(
            deadline > Clock::get()?.unix_timestamp,
            AgentTrustError::InvalidDeadline
        );
        
        self.client = client;
        self.agent = None;
        self.title = title;
        self.description_hash = description_hash;
        self.bounty = bounty;
        self.deadline = deadline;
        self.status = TaskStatus::Open;
        self.deliverable_hash = None;
        self.created_at = Clock::get()?.unix_timestamp;
        self.claimed_at = None;
        self.submitted_at = None;
        self.bump = bump;
        
        Ok(())
    }
    
    /// Claim the task
    pub fn claim(&mut self, agent: Pubkey) -> Result<()> {
        require!(
            self.status == TaskStatus::Open,
            AgentTrustError::TaskNotOpen
        );
        
        require!(
            Clock::get()?.unix_timestamp < self.deadline,
            AgentTrustError::DeadlinePassed
        );
        
        self.agent = Some(agent);
        self.status = TaskStatus::Claimed;
        self.claimed_at = Some(Clock::get()?.unix_timestamp);
        
        Ok(())
    }
    
    /// Submit deliverables
    pub fn submit(&mut self, deliverable_hash: [u8; 32]) -> Result<()> {
        require!(
            self.status == TaskStatus::Claimed,
            AgentTrustError::TaskNotClaimed
        );
        
        self.deliverable_hash = Some(deliverable_hash);
        self.status = TaskStatus::Submitted;
        self.submitted_at = Some(Clock::get()?.unix_timestamp);
        
        Ok(())
    }
    
    /// Confirm task completion
    pub fn confirm(&mut self) -> Result<()> {
        require!(
            self.status == TaskStatus::Submitted,
            AgentTrustError::TaskNotSubmitted
        );
        
        self.status = TaskStatus::Confirmed;
        
        Ok(())
    }
    
    /// Dispute the task
    pub fn dispute(&mut self) -> Result<()> {
        require!(
            self.status == TaskStatus::Submitted,
            AgentTrustError::TaskNotSubmitted
        );
        
        self.status = TaskStatus::Disputed;
        
        Ok(())
    }
    
    /// Check if deadline has passed
    pub fn is_deadline_passed(&self) -> bool {
        Clock::get()
            .map(|c| c.unix_timestamp > self.deadline)
            .unwrap_or(false)
    }
}

/// Seeds for Task PDA: [b"task", client_pubkey, task_id_counter]
pub fn get_task_seeds(client: &Pubkey, task_id: u64) -> Vec<u8> {
    let mut seeds = b"task".to_vec();
    seeds.extend_from_slice(client.as_ref());
    seeds.extend_from_slice(&task_id.to_le_bytes());
    seeds
}
