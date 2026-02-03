use anchor_lang::prelude::*;
use crate::state::{Task, get_task_seeds};
use crate::errors::AgentTrustError;
use crate::TRANSACTION_FEE_BPS;

#[derive(Accounts)]
#[instruction(title: String, description_hash: [u8; 32], bounty: u64)]
pub struct CreateTask<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    
    /// CHECK: Task PDA will be initialized
    #[account(
        init,
        payer = client,
        space = Task::SPACE,
        seeds = [b"task", client.key().as_ref(), &client_task_count.to_le_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,
    
    /// CHECK: Escrow account to hold funds
    #[account(
        init,
        payer = client,
        space = 8 + 8, // discriminator + balance
        seeds = [b"escrow", task.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Escrow {
    pub balance: u64,
    pub bump: u8,
}

impl Escrow {
    pub const SPACE: usize = 8 + 8 + 1;
}

pub fn handler(
    ctx: Context<CreateTask>,
    title: String,
    description_hash: [u8; 32],
    bounty: u64,
    deadline: i64,
) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let client = ctx.accounts.client.key();
    let bump = ctx.bumps.task;
    
    // Calculate fee (1%)
    let fee = (bounty * TRANSACTION_FEE_BPS) / 10_000;
    let total_required = bounty + fee;
    
    // Verify client has enough funds
    require!(
        ctx.accounts.client.lamports() >= total_required,
        AgentTrustError::InsufficientFunds
    );
    
    // Initialize task
    task.create(
        client,
        title.clone(),
        description_hash,
        bounty,
        deadline,
        bump,
    )?;
    
    // Initialize escrow
    let escrow = &mut ctx.accounts.escrow;
    escrow.balance = bounty;
    escrow.bump = ctx.bumps.escrow;
    
    // Transfer funds to escrow
    // Note: In a complete implementation, we'd use system_program::transfer
    // For now, we're tracking the intent
    
    msg!("Task created: {}", title);
    msg!("Bounty: {} lamports", bounty);
    msg!("Fee: {} lamports (1%)", fee);
    msg!("Deadline: {}", deadline);
    
    Ok(())
}
