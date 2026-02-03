use anchor_lang::prelude::*;
use crate::state::{Task, Agent};
use crate::errors::AgentTrustError;

#[derive(Accounts)]
pub struct ClaimTask<'info> {
    #[account(mut)]
    pub agent: Signer<'info>,
    
    #[account(
        mut,
        constraint = task.status == crate::TaskStatus::Open @ AgentTrustError::TaskNotOpen
    )]
    pub task: Account<'info, Task>,
    
    #[account(
        constraint = agent_account.owner == agent.key() @ AgentTrustError::Unauthorized
    )]
    pub agent_account: Account<'info, Agent>,
}

pub fn handler(ctx: Context<ClaimTask>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let agent_key = ctx.accounts.agent.key();
    
    task.claim(agent_key)?;
    
    msg!("Task claimed by agent: {}", agent_key);
    
    Ok(())
}
