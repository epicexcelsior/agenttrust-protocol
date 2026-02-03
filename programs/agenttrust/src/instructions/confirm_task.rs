use anchor_lang::prelude::*;
use crate::state::{Task, Agent};
use crate::errors::AgentTrustError;
use crate::TRANSACTION_FEE_BPS;

#[derive(Accounts)]
pub struct ConfirmTask<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    
    #[account(
        mut,
        constraint = task.client == client.key() @ AgentTrustError::NotClient,
        constraint = task.status == crate::TaskStatus::Submitted @ AgentTrustError::TaskNotSubmitted
    )]
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

pub fn handler(ctx: Context<ConfirmTask>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let agent_account = &mut ctx.accounts.agent_account;
    let bounty = task.bounty;
    
    // Confirm task
    task.confirm()?;
    
    // Calculate fee
    let fee = (bounty * TRANSACTION_FEE_BPS) / 10_000;
    let agent_payment = bounty - fee;
    
    // Update agent stats
    agent_account.record_success();
    agent_account.update_reputation_score();
    
    // Release funds from escrow to agent
    // Note: In production, use system_program::transfer from escrow
    
    msg!("Task confirmed and completed");
    msg!("Agent payment: {} lamports", agent_payment);
    msg!("Protocol fee: {} lamports", fee);
    msg!("Agent new reputation: {}", agent_account.reputation_score);
    
    Ok(())
}
