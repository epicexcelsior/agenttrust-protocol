use anchor_lang::prelude::*;

#[error_code]
pub enum AgentTrustError {
    #[msg("Agent name exceeds maximum length")]
    NameTooLong,
    
    #[msg("Task title exceeds maximum length")]
    TitleTooLong,
    
    #[msg("Insufficient stake. Minimum 0.02 SOL required")]
    InsufficientStake,
    
    #[msg("Invalid bounty amount")]
    InvalidBounty,
    
    #[msg("Invalid deadline")]
    InvalidDeadline,
    
    #[msg("Task is not open for claiming")]
    TaskNotOpen,
    
    #[msg("Task has not been claimed")]
    TaskNotClaimed,
    
    #[msg("Task has not been submitted")]
    TaskNotSubmitted,
    
    #[msg("Task deadline has passed")]
    DeadlinePassed,
    
    #[msg("Unauthorized caller")]
    Unauthorized,
    
    #[msg("Agent not found")]
    AgentNotFound,
    
    #[msg("Task not found")]
    TaskNotFound,
    
    #[msg("Dispute not found")]
    DisputeNotFound,
    
    #[msg("Dispute is not in evidence period")]
    DisputeNotInEvidencePeriod,
    
    #[msg("Evidence period is over")]
    EvidencePeriodOver,
    
    #[msg("Evidence period is still active")]
    EvidencePeriodActive,
    
    #[msg("Evidence already submitted")]
    EvidenceAlreadySubmitted,
    
    #[msg("Dispute already resolved")]
    DisputeAlreadyResolved,
    
    #[msg("Invalid dispute status")]
    InvalidDisputeStatus,
    
    #[msg("Task already has an agent assigned")]
    TaskAlreadyClaimed,
    
    #[msg("Only client can perform this action")]
    NotClient,
    
    #[msg("Only assigned agent can perform this action")]
    NotAssignedAgent,
    
    #[msg("Calculation overflow")]
    Overflow,
    
    #[msg("Insufficient funds")]
    InsufficientFunds,
    
    #[msg("Fee calculation error")]
    FeeCalculationError,
}
