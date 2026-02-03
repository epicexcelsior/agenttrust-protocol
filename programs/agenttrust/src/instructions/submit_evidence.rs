use anchor_lang::prelude::*;
use crate::state::{Dispute, Task};
use crate::errors::AgentTrustError;

#[derive(Accounts)]
pub struct SubmitEvidence<'info> {
    #[account(mut)]
    pub participant: Signer<'info>,
    
    #[account(
        constraint = dispute.task == task.key() @ AgentTrustError::InvalidDisputeStatus
    )]
    pub task: Account<'info, Task>,
    
    #[account(mut)]
    pub dispute: Account<'info, Dispute>,
}

pub fn handler(
    ctx: Context<SubmitEvidence>,
    evidence_hash: [u8; 32],
) -> Result<()> {
    let dispute = &mut ctx.accounts.dispute;
    let task = &ctx.accounts.task;
    let participant = ctx.accounts.participant.key();
    
    // Determine if participant is client or agent
    let is_client = task.client == participant;
    let is_agent = task.agent == Some(participant);
    
    require!(
        is_client || is_agent,
        AgentTrustError::Unauthorized
    );
    
    dispute.submit_evidence(evidence_hash, is_client)?;
    
    if is_client {
        msg!("Client submitted evidence");
    } else {
        msg!("Agent submitted evidence");
    }
    
    Ok(())
}
