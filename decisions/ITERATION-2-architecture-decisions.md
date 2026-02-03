# ITERATION-2: Architecture Decisions

**Date:** 2026-02-02
**Decision Maker:** Hunter (human)

## ✅ Technical Decisions Made

### Q1: Evidence Storage → IPFS + Pinata
**Decision:** Use IPFS with Pinata free tier for evidence storage
**Rationale:**
- Free tier: 1GB storage, unlimited pins
- Sufficient for hackathon MVP (research reports, evidence files)
- Can upgrade to Arweave for permanence if we win prize money
- **Utility:** Minimizes costs while maintaining functionality
- **Effect:** Store file hashes on-chain, content on IPFS, Pinata ensures persistence

### Q2: Transaction Fee → 1%
**Decision:** 1% fee on all completed transactions
**Rationale:**
- Industry standard for marketplace protocols
- Sustainable for protocol operations
- Not greedy (competitive with Web2 alternatives)
- **Utility:** Funds protocol maintenance and future development
- **Effect:** On $100 task = $1 fee, negligible for users, sustainable for us

### Q3: Minimum Stake → 0.02 SOL (~$4)
**Decision:** 0.02 SOL minimum stake for new agents
**Rationale:**
- Low enough for broad adoption ($2-10 range as requested)
- High enough to prevent spam accounts
- At $4, serious agents can afford it, trolls won't bother
- **Utility:** Skin-in-game without excluding small agents
- **Effect:** New agents must stake ~$4 to join, can increase stake to boost reputation

## 📊 Updated Parameters

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| Evidence Storage | IPFS + Pinata | Free, sufficient for MVP |
| Transaction Fee | 1% | Sustainable, standard |
| Minimum Stake | 0.02 SOL (~$4) | Low barrier, spam prevention |
| Dispute Window | 24 hours | Fair evidence collection |
| Reputation Weight | 60% success, 40% stake | Balance competence vs commitment |

## 🔄 Ready for ITERATION-3

All architecture decisions complete. Proceeding to smart contract implementation.
