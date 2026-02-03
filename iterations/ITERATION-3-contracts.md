# ITERATION-3: Smart Contract Implementation

**Date:** 2026-02-02  
**Focus:** Anchor smart contracts — Agent registry, tasks, escrow, disputes  
**Previous:** ITERATION-2 (Architecture complete)  
**Next:** ITERATION-4 (SDK & API)

## 🎯 Implementation Goals

1. **Anchor project scaffold** — Initialize with proper structure
2. **Agent Registry** — PDA for agent data + reputation
3. **Task Lifecycle** — Create, claim, submit, confirm, dispute
4. **Escrow Program** — Lock funds, release on completion/slashing
5. **Dispute State Machine** — 24-hour window, evidence, judgment
6. **Reputation Calculation** — On-chain scoring algorithm

## 🏗️ Contract Architecture

```
programs/agenttrust/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── state/
│   │   ├── agent.rs          # Agent PDA definition
│   │   ├── task.rs           # Task PDA definition
│   │   ├── escrow.rs         # Escrow PDA definition
│   │   └── dispute.rs        # Dispute PDA definition
│   ├── instructions/
│   │   ├── initialize.rs     # Program initialization
│   │   ├── register_agent.rs # Agent registration
│   │   ├── create_task.rs    # Task creation
│   │   ├── claim_task.rs     # Agent claims task
│   │   ├── submit_task.rs    # Submit deliverables
│   │   ├── confirm_task.rs   # Client confirms
│   │   ├── dispute_task.rs   # File dispute
│   │   ├── submit_evidence.rs # Submit evidence
│   │   ├── resolve_dispute.rs # AI resolves (autonomous)
│   │   └── update_reputation.rs # Recalculate reputation
│   └── errors.rs
```

## 📋 Implementation Checklist

### Phase 1: Scaffold & State (Hours 1-4)
- [ ] Initialize Anchor project
- [ ] Define Agent PDA structure
- [ ] Define Task PDA structure
- [ ] Define Escrow PDA structure
- [ ] Define Dispute PDA structure
- [ ] Implement error types

### Phase 2: Core Instructions (Hours 5-12)
- [ ] `register_agent` — Stake 0.02 SOL, create agent PDA
- [ ] `create_task` — Client creates task, funds escrow
- [ ] `claim_task` — Agent claims, updates task state
- [ ] `submit_task` — Agent submits deliverables (IPFS hash)
- [ ] `confirm_task` — Client confirms, releases funds
- [ ] `dispute_task` — Client disputes, opens 24hr window

### Phase 3: Dispute & Reputation (Hours 13-20)
- [ ] `submit_evidence` — Both parties submit proof
- [ ] `resolve_dispute` — AI judgment, execute outcome
- [ ] `slash_agent` — Penalize losing agent
- [ ] `update_reputation` — Recalculate agent score
- [ ] Time-lock logic for 24-hour window

### Phase 4: Testing (Hours 21-24)
- [ ] Unit tests for each instruction
- [ ] Integration test: happy path
- [ ] Integration test: dispute path
- [ ] Deploy to devnet

## 🔧 Technical Details

### Agent PDA Structure
```rust
#[account]
pub struct Agent {
    pub owner: Pubkey,              // Agent's wallet
    pub name: String,               // Agent name (max 32 chars)
    pub reputation_score: u64,      // 0-100 calculated score
    pub successful_tasks: u64,      // Completed successfully
    pub total_tasks: u64,           // Total attempted
    pub total_stake: u64,           // Lamports staked
    pub dispute_losses: u64,        // Lost disputes
    pub created_at: i64,            // Timestamp
    pub bump: u8,                   // PDA bump
}
```

### Task PDA Structure
```rust
#[account]
pub struct Task {
    pub client: Pubkey,             // Task creator
    pub agent: Option<Pubkey>,      // Assigned agent (None until claimed)
    pub title: String,              // Task title (max 64 chars)
    pub description_hash: [u8; 32], // IPFS hash of full description
    pub bounty: u64,                // Lamports locked in escrow
    pub deadline: i64,              // Unix timestamp
    pub status: TaskStatus,         // Enum: Open, Claimed, Submitted, Confirmed, Disputed
    pub deliverable_hash: Option<[u8; 32]>, // IPFS hash of deliverables
    pub created_at: i64,            // Timestamp
    pub bump: u8,                   // PDA bump
}

pub enum TaskStatus {
    Open,       // Available to claim
    Claimed,    // Agent assigned, in progress
    Submitted,  // Deliverables submitted
    Confirmed,  // Client confirmed, completed
    Disputed,   // Dispute filed
}
```

### Dispute PDA Structure
```rust
#[account]
pub struct Dispute {
    pub task: Pubkey,               // Associated task
    pub client_evidence: Option<[u8; 32]>, // IPFS hash
    pub agent_evidence: Option<[u8; 32]>,  // IPFS hash
    pub filed_at: i64,              // Dispute filed timestamp
    pub resolved_at: Option<i64>,   // Resolution timestamp
    pub resolution: Option<DisputeResolution>, // Outcome
    pub status: DisputeStatus,      // Enum
    pub bump: u8,                   // PDA bump
}

pub enum DisputeStatus {
    EvidencePeriod,    // 24-hour window open
    JudgmentPending,   // Window closed, awaiting AI
    Resolved,          // Complete
}

pub enum DisputeResolution {
    ClientWins,        // Return bounty + slash agent
    AgentWins,         // Release bounty to agent
}
```

### Reputation Calculation (On-Chain)
```rust
pub fn calculate_reputation(
    successful_tasks: u64,
    total_tasks: u64,
    total_stake_lamports: u64,
    dispute_losses: u64,
) -> u64 {
    // Success rate component (60% weight)
    let success_rate = if total_tasks > 0 {
        (successful_tasks * 100) / total_tasks
    } else {
        0
    };
    let success_component = success_rate * 60 / 100;
    
    // Stake component (40% weight) - normalized
    // Max expected stake: 100 SOL = 100,000,000,000 lamports
    let max_stake = 100_000_000_000u64;
    let stake_normalized = (total_stake_lamports.min(max_stake) * 100) / max_stake;
    let stake_component = stake_normalized * 40 / 100;
    
    // Dispute penalty: -10 points per loss
    let dispute_penalty = dispute_losses * 10;
    
    let raw_score = success_component + stake_component;
    raw_score.saturating_sub(dispute_penalty)
}
```

## 💰 Fee Structure (On-Chain)

```rust
pub const TRANSACTION_FEE_BPS: u64 = 100; // 1% = 100 basis points
pub const MINIMUM_STAKE_LAMPORTS: u64 = 20_000_000; // 0.02 SOL

pub fn calculate_fee(bounty: u64) -> u64 {
    (bounty * TRANSACTION_FEE_BPS) / 10_000
}
```

## ⏰ Time Constants

```rust
pub const DISPUTE_WINDOW_SECONDS: i64 = 24 * 60 * 60; // 24 hours
pub const EVIDENCE_SUBMISSION_DEADLINE: i64 = 12 * 60 * 60; // 12 hours each
```

## 🧪 Testing Strategy

### Happy Path Test
1. Agent A registers (stakes 0.02 SOL)
2. Client C creates task (1 SOL bounty)
3. Agent A claims task
4. Agent A submits deliverables
5. Client C confirms
6. Agent A receives 0.99 SOL (1% fee deducted)
7. Agent A reputation increases

### Dispute Path Test
1. Agent A registers
2. Client C creates task (1 SOL bounty)
3. Agent A claims task
4. Agent A submits deliverables
5. Client C disputes (24hr window opens)
6. Both submit evidence
7. AI resolves in favor of client
8. Client receives 1 SOL back + Agent A slashed 0.02 SOL
9. Agent A reputation decreases

## 🚀 Deployment Plan

1. **Localnet** — Initial development and testing
2. **Devnet** — Integration testing, demo preparation
3. **Mainnet** — Production deployment (post-hackathon)

## 📊 Success Criteria

✅ **ITERATION-3 Complete when:**
- [ ] All PDAs defined and tested
- [ ] All instructions implemented
- [ ] Happy path integration test passes
- [ ] Dispute path integration test passes
- [ ] Deployed to devnet
- [ ] Ready for SDK integration

## 🔄 Next Steps

**Proceed to ITERATION-4 when:**
- Smart contracts complete and tested
- OR human requests SDK development

---

**Status:** 🔄 IN PROGRESS  
**Started:** 2026-02-02  
**ETA:** 24 hours for MVP contracts
