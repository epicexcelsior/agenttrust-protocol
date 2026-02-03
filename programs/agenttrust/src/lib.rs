use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("AGENTtrust111111111111111111111111111111111");

#[program]
pub mod agenttrust {
    use super::*;

    // Agent management
    pub fn register_agent(
        ctx: Context<RegisterAgent>,
        name: String,
        initial_stake: u64,
    ) -> Result<()> {
        instructions::register_agent::handler(ctx, name, initial_stake)
    }

    pub fn increase_stake(
        ctx: Context<IncreaseStake>,
        amount: u64,
    ) -> Result<()> {
        instructions::increase_stake::handler(ctx, amount)
    }

    // Task lifecycle
    pub fn create_task(
        ctx: Context<CreateTask>,
        title: String,
        description_hash: [u8; 32],
        bounty: u64,
        deadline: i64,
    ) -> Result<()> {
        instructions::create_task::handler(ctx, title, description_hash, bounty, deadline)
    }

    pub fn claim_task(ctx: Context<ClaimTask>) -> Result<()> {
        instructions::claim_task::handler(ctx)
    }

    pub fn submit_task(
        ctx: Context<SubmitTask>,
        deliverable_hash: [u8; 32],
    ) -> Result<()> {
        instructions::submit_task::handler(ctx, deliverable_hash)
    }

    pub fn confirm_task(ctx: Context<ConfirmTask>) -> Result<()> {
        instructions::confirm_task::handler(ctx)
    }

    // Dispute resolution
    pub fn dispute_task(ctx: Context<DisputeTask>) -> Result<()> {
        instructions::dispute_task::handler(ctx)
    }

    pub fn submit_evidence(
        ctx: Context<SubmitEvidence>,
        evidence_hash: [u8; 32],
    ) -> Result<()> {
        instructions::submit_evidence::handler(ctx, evidence_hash)
    }

    pub fn resolve_dispute(
        ctx: Context<ResolveDispute>,
        resolution: DisputeResolution,
    ) -> Result<()> {
        instructions::resolve_dispute::handler(ctx, resolution)
    }

    // Reputation
    pub fn update_reputation(ctx: Context<UpdateReputation>) -> Result<()> {
        instructions::update_reputation::handler(ctx)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Open,
    Claimed,
    Submitted,
    Confirmed,
    Disputed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum DisputeStatus {
    EvidencePeriod,
    JudgmentPending,
    Resolved,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum DisputeResolution {
    ClientWins,
    AgentWins,
}

// Constants
pub const TRANSACTION_FEE_BPS: u64 = 100; // 1% = 100 basis points
pub const MINIMUM_STAKE_LAMPORTS: u64 = 20_000_000; // 0.02 SOL
pub const DISPUTE_WINDOW_SECONDS: i64 = 24 * 60 * 60; // 24 hours
pub const MAX_AGENT_NAME_LEN: usize = 32;
pub const MAX_TASK_TITLE_LEN: usize = 64;
