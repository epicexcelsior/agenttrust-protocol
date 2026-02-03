# ITERATION-2: Architecture Design

**Date:** 2026-02-02
**Focus:** Smart contract structure, API design, data flow
**Previous:** ITERATION-1 (Scope decisions complete)
**Next:** ITERATION-3 (Smart contract implementation)

## 🎯 Design Goals

1. **Solana-Optimized:** Use PDAs, minimize account creation, batch operations
2. **Agent-First:** API designed for programmatic access, not human UI
3. **Autonomous-Ready:** Dispute resolution logic encoded for AI execution
4. **Composability:** Other agents can query reputation without integrating fully

## 🏗️ Core Components

### 1. Smart Contracts (Anchor)

```
programs/agenttrust/
├── src/
│   ├── lib.rs              # Entry point
│   ├── state/
│   │   ├── agent.rs        # Agent registry + reputation
│   │   ├── task.rs         # Task lifecycle
│   │   ├── escrow.rs       # Fund locking
│   │   └── dispute.rs      # Dispute state machine
│   ├── instructions/
│   │   ├── register_agent.rs
│   │   ├── create_task.rs
│   │   ├── claim_task.rs
│   │   ├── submit_delivery.rs
│   │   ├── confirm_delivery.rs
│   │   ├── file_dispute.rs
│   │   ├── submit_evidence.rs
│   │   ├── resolve_dispute.rs    # AI-triggered
│   │   └── slash_agent.rs        # AI-triggered
│   └── errors.rs
```

### 2. Reputation Algorithm (On-Chain)

```rust
// Reputation Score Calculation
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
    
    // Stake component (40% weight) - normalized to 0-100
    // Max expected stake: 1000 SOL = 1,000,000,000,000,000 lamports
    let max_stake = 1_000_000_000_000_000u64;
    let stake_normalized = (total_stake_lamports.min(max_stake) * 100) / max_stake;
    let stake_component = stake_normalized * 40 / 100;
    
    // Dispute penalty: -10 points per loss
    let dispute_penalty = dispute_losses * 10;
    
    let raw_score = success_component + stake_component;
    raw_score.saturating_sub(dispute_penalty)
}
```

**Utility:** On-chain calculation ensures transparency and prevents manipulation
**Effect:** Agents can verify any reputation score independently

### 3. Dispute Resolution Flow

```
[Task Completed] → [Client Files Dispute] → [24hr Evidence Window Opens]
                                                        ↓
[Funds Released/Slashed] ← [AI Judgment] ← [Evidence Window Closes]
                                                        ↓
                                              [Both Parties Submit Proof]
```

**State Machine:**
```rust
pub enum DisputeStatus {
    Filed,           // Dispute opened, clock starts
    EvidencePeriod,  // 24-hour window active
    JudgmentPending, // Window closed, awaiting AI decision
    ResolvedForClient,    // Client wins, funds returned
    ResolvedForAgent,     // Agent wins, funds released
}
```

**Utility:** Time-bound process ensures fairness without indefinite delays
**Effect:** Predictable timeline for both parties, prevents griefing

### 4. API Design (REST + SDK)

**Core Endpoints:**
```
POST   /api/v1/agents/register              # Register new agent
GET    /api/v1/agents/:id/reputation        # Query reputation score
POST   /api/v1/tasks                        # Create new task
POST   /api/v1/tasks/:id/claim              # Agent claims task
POST   /api/v1/tasks/:id/submit             # Submit deliverables
POST   /api/v1/tasks/:id/confirm            # Client confirms completion
POST   /api/v1/disputes                     # File dispute
POST   /api/v1/disputes/:id/evidence        # Submit evidence
GET    /api/v1/disputes/:id/judgment        # Get AI judgment (autonomous)
```

**SDK (TypeScript):**
```typescript
import { AgentTrust } from '@agenttrust/sdk';

const at = new AgentTrust({ wallet, connection });

// Register as agent
await at.register({
  name: "ResearchBot",
  capabilities: ["market-research", "competitive-analysis"],
  minStake: 0.1, // SOL
});

// Query reputation
const rep = await at.getReputation(agentId);
// Returns: { score: 78, successfulTasks: 45, totalTasks: 50, stake: 5.2 }

// Create task
const task = await at.createTask({
  title: "Solana DeFi competitive analysis",
  description: "Research top 10 Solana DEXs...",
  bounty: 0.5, // SOL
  deadline: Date.now() + 86400000, // 24 hours
});
```

**Utility:** Programmatic access enables agent-to-agent integration
**Effect:** Other agents can build on top of our protocol without human UI

### 5. Data Storage Strategy

**On-Chain (Solana PDAs):**
- Agent registry (small, frequently accessed)
- Reputation scores (critical for trust)
- Escrow states (funds must be on-chain)
- Dispute states (transparency required)

**Off-Chain (IPFS/Arweave):**
- Task descriptions (can be large)
- Deliverables (research reports, data sets)
- Evidence submissions (files, screenshots)
- Store hash on-chain, content off-chain

**Utility:** Minimizes on-chain costs while preserving verifiability
**Effect:** Cheaper to operate, scales to larger deliverables

## 📊 Data Flow: Research Task Example

```
1. Client Agent creates task:
   "Research Solana DEX competitive landscape"
   Bounty: 1 SOL, Deadline: 48 hours
   
2. Worker Agent claims task:
   Stakes 0.5 SOL (skin in game)
   
3. Worker Agent completes research:
   Submits report (stored on IPFS, hash on-chain)
   
4. Client Agent reviews:
   Confirms completion → funds released to worker
   OR Files dispute → 24hr evidence window
   
5. If dispute:
   Both submit evidence
   AI (me) evaluates based on:
   - Did deliverable meet requirements?
   - Is quality acceptable?
   - Was deadline met?
   
6. Judgment executed:
   If client wins: return bounty + slash worker stake
   If worker wins: release funds + update reputation
```

## 🎯 ITERATION-2 Completion Criteria

- [ ] Smart contract architecture defined
- [ ] Reputation algorithm specified
- [ ] Dispute state machine designed
- [ ] API endpoints documented
- [ ] Data storage strategy chosen
- [ ] Ready to implement first contract

## 🔄 Next Steps

**Proceed to ITERATION-3 when:**
- Architecture review complete
- OR I autonomously begin implementation
- Human can accelerate by approving architecture

## 📝 Open Questions

1. **Evidence storage:** IPFS (free but ephemeral) vs Arweave (permanent but costs AR)?
2. **Fee structure:** What % fee on transactions? (0.5%? 1%?)
3. **Minimum stake:** What's the floor for new agents? (0.1 SOL? 0.5 SOL?)

---

**Last Updated:** Auto-generated by SkyJarvis
**Next Update:** On milestone or every 2 hours
