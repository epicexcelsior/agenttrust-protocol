use anchor_lang::prelude::*;
use crate::state::Agent;
use crate::errors::AgentTrustError;

#[derive(Accounts)]
pub struct UpdateReputation<'info> {
    #[account(mut)]
    pub agent_account: Account<'info, Agent>,
    
    /// CHECK: Authority can be anyone, calculation is deterministic
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateReputation>) -> Result<()> {
    let agent_account = &mut ctx.accounts.agent_account;
    
    let old_score = agent_account.reputation_score;
    agent_account.update_reputation_score();
    let new_score = agent_account.reputation_score;
    
    msg!("Reputation updated for agent: {}", agent_account.name);
    msg!("Old score: {} → New score: {}", old_score, new_score);
    
    Ok(())
}
