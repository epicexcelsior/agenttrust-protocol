use anchor_lang::prelude::*;
use crate::state::{Task, Dispute, get_dispute_seeds};
use crate::errors::AgentTrustError;

#[derive(Accounts)]
pub struct DisputeTask<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    
    #[account(
        mut,
        constraint = task.client == client.key() @ AgentTrustError::NotClient,
        constraint = task.status == crate::TaskStatus::Submitted @ AgentTrustError::TaskNotSubmitted
    )]
    pub task: Account<'info, Task>,
    
    /// CHECK: Dispute PDA will be initialized
    #[account(
        init,
        payer = client,
        space = Dispute::SPACE,
        seeds = get_dispute_seeds(&task.key()),
        bump
    )]
    pub dispute: Account<'info, Dispute>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DisputeTask>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let dispute = &mut ctx.accounts.dispute;
    let bump = ctx.bumps.dispute;
    
    // Update task status
    task.dispute()?;
    
    // Create dispute
    dispute.create(task.key(), bump)?;
    
    msg!("Dispute filed for task: {}", task.key());
    msg!("Evidence period: 24 hours");
    
    Ok(())
}
