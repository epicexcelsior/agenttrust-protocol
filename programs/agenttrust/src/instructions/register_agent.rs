use anchor_lang::prelude::*;
use crate::state::{Agent, get_agent_seeds};
use crate::errors::AgentTrustError;
use crate::MINIMUM_STAKE_LAMPORTS;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct RegisterAgent<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init,
        payer = owner,
        space = Agent::SPACE,
        seeds = get_agent_seeds(&owner.key()),
        bump
    )]
    pub agent: Account<'info, Agent>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterAgent>,
    name: String,
    initial_stake: u64,
) -> Result<()> {
    let agent = &mut ctx.accounts.agent;
    let owner = ctx.accounts.owner.key();
    let bump = ctx.bumps.agent;
    
    // Initialize agent account
    agent.register(owner, name, initial_stake, bump)?;
    
    // Transfer stake to agent account (stored in the PDA)
    // Note: In production, you might want a separate stake vault
    // For simplicity, we're tracking stake in the agent account
    
    msg!("Agent registered: {}", agent.name);
    msg!("Initial stake: {} lamports", initial_stake);
    msg!("Starting reputation: {}", agent.reputation_score);
    
    Ok(())
}
