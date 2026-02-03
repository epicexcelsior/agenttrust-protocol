use anchor_lang::prelude::*;
use crate::state::{Dispute, Task, Agent};
use crate::errors::AgentTrustError;
use crate::{DisputeResolution, TRANSACTION_FEE_BPS, MINIMUM_STAKE_LAMPORTS};

#[derive(Accounts)]
pub struct ResolveDispute<'info> {
    /// The AI/authority resolving the dispute
    /// In production, this would be a verified AI agent or DAO
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub dispute: Account<'info, Dispute>,
    
    #[account(mut)]
    pub task: Account<'info, Task>,
    
    #[account(mut)]
    pub agent_account: Account<'info, Agent>,
    
    /// CHECK: Escrow account
    #[account(
        mut,
        seeds = [b"escrow", task.key().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, crate::instructions::create_task::Escrow>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ResolveDispute>,
    resolution: DisputeResolution,
) -> Result<()> {
    let dispute = &mut ctx.accounts.dispute;
    let task = &mut ctx.accounts.task;
    let agent_account = &mut ctx.accounts.agent_account;
    let bounty = task.bounty;
    
    // Resolve dispute
    dispute.resolve(resolution)?;
    
    match resolution {
        DisputeResolution::ClientWins => {
            // Return bounty to client
            // Slash agent's stake
            let slash_amount = MINIMUM_STAKE_LAMPORTS / 2; // Slash 50% of minimum stake
            agent_account.decrease_stake(slash_amount)?;
            agent_account.record_dispute_loss();
            
            msg!("Dispute resolved: Client wins");
            msg!("Bounty returned to client: {} lamports", bounty);
            msg!("Agent slashed: {} lamports", slash_amount);
        }
        DisputeResolution::AgentWins => {
            // Release bounty to agent (minus fee)
            let fee = (bounty * TRANSACTION_FEE_BPS) / 10_000;
            let agent_payment = bounty - fee;
            
            agent_account.record_success();
            
            msg!("Dispute resolved: Agent wins");
            msg!("Agent payment: {} lamports", agent_payment);
            msg!("Protocol fee: {} lamports", fee);
        }
    }
    
    // Update agent reputation
    agent_account.update_reputation_score();
    
    msg!("Agent new reputation: {}", agent_account.reputation_score);
    
    Ok(())
}
