use anchor_lang::prelude::*;
use crate::state::Task;
use crate::errors::AgentTrustError;

#[derive(Accounts)]
pub struct SubmitTask<'info> {
    #[account(mut)]
    pub agent: Signer<'info>,
    
    #[account(
        mut,
        constraint = task.agent == Some(agent.key()) @ AgentTrustError::NotAssignedAgent,
        constraint = task.status == crate::TaskStatus::Claimed @ AgentTrustError::TaskNotClaimed
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(
    ctx: Context<SubmitTask>,
    deliverable_hash: [u8; 32],
) -> Result<()> {
    let task = &mut ctx.accounts.task;
    
    task.submit(deliverable_hash)?;
    
    msg!("Task submitted with deliverable hash: {:?}", deliverable_hash);
    
    Ok(())
}
