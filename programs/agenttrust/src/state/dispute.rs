use anchor_lang::prelude::*;
use crate::{DisputeStatus, DisputeResolution, DISPUTE_WINDOW_SECONDS};
use crate::errors::AgentTrustError;

#[account]
pub struct Dispute {
    /// Associated task pubkey
    pub task: Pubkey,
    
    /// Client's evidence IPFS hash (None until submitted)
    pub client_evidence: Option<[u8; 32]>,
    
    /// Agent's evidence IPFS hash (None until submitted)
    pub agent_evidence: Option<[u8; 32]>,
    
    /// Unix timestamp when dispute filed
    pub filed_at: i64,
    
    /// Unix timestamp when resolved (None until resolved)
    pub resolved_at: Option<i64>,
    
    /// Resolution outcome (None until resolved)
    pub resolution: Option<DisputeResolution>,
    
    /// Current dispute status
    pub status: DisputeStatus,
    
    /// PDA bump seed
    pub bump: u8,
}

impl Dispute {
    /// Calculate space needed for Dispute account
    /// 8 (discriminator) + 32 (task) + 33 (client evidence option) + 33 (agent evidence option) +
    /// 8 (filed_at) + 9 (resolved option) + 2 (resolution option) + 1 (status) + 1 (bump)
    pub const SPACE: usize = 8 + 32 + 33 + 33 + 8 + 9 + 2 + 1 + 1;
    
    /// Create a new dispute
    pub fn create(
        &mut self,
        task: Pubkey,
        bump: u8,
    ) -> Result<()> {
        self.task = task;
        self.client_evidence = None;
        self.agent_evidence = None;
        self.filed_at = Clock::get()?.unix_timestamp;
        self.resolved_at = None;
        self.resolution = None;
        self.status = DisputeStatus::EvidencePeriod;
        self.bump = bump;
        
        Ok(())
    }
    
    /// Submit evidence (can be called by client or agent)
    pub fn submit_evidence(
        &mut self,
        evidence_hash: [u8; 32],
        is_client: bool,
    ) -> Result<()> {
        require!(
            self.status == DisputeStatus::EvidencePeriod,
            AgentTrustError::DisputeNotInEvidencePeriod
        );
        
        require!(
            !self.is_evidence_period_over(),
            AgentTrustError::EvidencePeriodOver
        );
        
        if is_client {
            require!(
                self.client_evidence.is_none(),
                AgentTrustError::EvidenceAlreadySubmitted
            );
            self.client_evidence = Some(evidence_hash);
        } else {
            require!(
                self.agent_evidence.is_none(),
                AgentTrustError::EvidenceAlreadySubmitted
            );
            self.agent_evidence = Some(evidence_hash);
        }
        
        Ok(())
    }
    
    /// Resolve the dispute (called by AI/authority)
    pub fn resolve(&mut self, resolution: DisputeResolution) -> Result<()> {
        require!(
            self.status == DisputeStatus::EvidencePeriod || 
            self.status == DisputeStatus::JudgmentPending,
            AgentTrustError::DisputeAlreadyResolved
        );
        
        // Ensure evidence period is over or both parties submitted
        if self.status == DisputeStatus::EvidencePeriod {
            require!(
                self.is_evidence_period_over() || 
                (self.client_evidence.is_some() && self.agent_evidence.is_some()),
                AgentTrustError::EvidencePeriodActive
            );
        }
        
        self.resolution = Some(resolution);
        self.resolved_at = Some(Clock::get()?.unix_timestamp);
        self.status = DisputeStatus::Resolved;
        
        Ok(())
    }
    
    /// Move to judgment pending (when evidence period ends)
    pub fn move_to_judgment(&mut self) -> Result<()> {
        require!(
            self.status == DisputeStatus::EvidencePeriod,
            AgentTrustError::InvalidDisputeStatus
        );
        
        require!(
            self.is_evidence_period_over(),
            AgentTrustError::EvidencePeriodActive
        );
        
        self.status = DisputeStatus::JudgmentPending;
        
        Ok(())
    }
    
    /// Check if evidence period is over
    pub fn is_evidence_period_over(&self) -> bool {
        let current_time = Clock::get()
            .map(|c| c.unix_timestamp)
            .unwrap_or(0);
        
        current_time > self.filed_at + DISPUTE_WINDOW_SECONDS
    }
    
    /// Get time remaining in evidence period (seconds)
    pub fn get_time_remaining(&self) -> i64 {
        let current_time = Clock::get()
            .map(|c| c.unix_timestamp)
            .unwrap_or(0);
        
        let end_time = self.filed_at + DISPUTE_WINDOW_SECONDS;
        (end_time - current_time).max(0)
    }
}

/// Seeds for Dispute PDA: [b"dispute", task_pubkey]
pub fn get_dispute_seeds(task: &Pubkey) -> Vec<&[u8]> {
    vec![b"dispute", task.as_ref()]
}
