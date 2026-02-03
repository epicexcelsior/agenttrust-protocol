use anchor_lang::prelude::*;
use crate::state::Agent;
use crate::errors::AgentTrustError;

#[derive(Accounts)]
pub struct IncreaseStake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        constraint = agent.owner == owner.key() @ AgentTrustError::Unauthorized
    )]
    pub agent: Account<'info, Agent>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<IncreaseStake>,
    amount: u64,
) -> Result<()> {
    let agent = &mut ctx.accounts.agent;
    
    require!(amount > 0, AgentTrustError::InvalidBounty);
    
    agent.increase_stake(amount);
    
    // Update reputation with new stake
    agent.update_reputation_score();
    
    msg!("Stake increased by: {} lamports", amount);
    msg!("New total stake: {} lamports", agent.total_stake);
    msg!("New reputation: {}", agent.reputation_score);
    
    Ok(())
}
