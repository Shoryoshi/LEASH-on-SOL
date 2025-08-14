# LEASH V2 — Solana Edition

> **I bring you a new LEASH, on a new chain for my puppy.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/Solana-1.17.0-blue.svg)](https://solana.com/)
[![Anchor](https://img.shields.io/badge/Anchor-0.29.0-purple.svg)](https://anchor-lang.com/)

## 🚀 Executive Summary

LEASH V2 represents a paradigm shift in decentralized governance and protocol-owned liquidity, migrating from Ethereum's legacy constraints to Solana's high-performance, low-cost infrastructure. This is not merely a token migration—it's a complete architectural renaissance that leverages Solana's parallel execution, sub-second finality, and native programmability to create a self-sustaining, fee-generating ecosystem.

**Introducing SHI**: The algorithmic stablecoin that Ryoshi originally envisioned, now realized on Solana. SHI serves as the foundation for expanded liquidity pools and enhanced fee generation, creating a sustainable economic engine for the entire ecosystem.

## 🎯 Mission Statement

**Unify the LEASH brand and community under a Solana-native architecture that provides superior utility, governance, and economic incentives while maintaining backward compatibility through cross-chain bridges. SHI algorithmic stablecoin enables expanded DeFi operations and sustainable fee generation.**

---

## 🌉 **🚨 URGENT: Bridge Your LEASH to Solana Now!**

**Your Ethereum LEASH is at risk!** The contract still has active rebase functions that can change supply despite "renounced" claims.

**[📖 Read the Full Investigation](https://blog.shib.io/leash-investigation-how-supply-changed-despite-renounced-claims/)**

**[🌉 Bridge LEASH to Solana Now](docs/how-to-bridge-leash.md)** - Complete step-by-step guide to escape rebase risk!

**Why Bridge?**
- ✅ **Escape rebase risk** - No more supply manipulation
- ✅ **True immutable supply** - What you own stays yours
- ✅ **Join the revolution** - Be part of LEASH V2 on Solana
- ✅ **Community governance** - Real DAO control, no hidden backdoors

## 🏛️ **Governance & Current Status**

### **Doggy DAO Governance**
All economic parameters, fee structures, and protocol configurations will be determined through **Doggy DAO governance** on Solana. This ensures community-driven decision-making and flexibility to adapt to market conditions and community needs.

### **Current LEASH V2 Status**
- **Token Status**: ✅ **Fully deployed and freely tradable** on Solana
- **No Airdrops**: All tokens are in circulation with no reserves
- **Main Pool**: Currently routing 75% of fees to Ryoshi (subject to DAO governance)
- **Governance**: All future parameters to be determined by Doggy DAO

## 🔬 Technical Architecture Overview

### Core Design Principles

1. **Protocol-Owned Liquidity (POL)**: Self-sustaining fee generation through automated market making
2. **Time-Weighted Governance**: veLEASH system with lockup multipliers for long-term alignment
3. **Single-Sided Staking**: xLEASH rewards fed by LP fee sweeps and protocol revenue
4. **Cross-Chain Compatibility**: Wormhole integration for seamless Ethereum bridge
5. **Zero-Dilution Economics**: Fixed supply with deflationary pressure through fee burning
6. **SHI Stablecoin Foundation**: Algorithmic stability enabling expanded liquidity pools and fee farming

### Smart Contract Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   LEASH V2      │    │   xLEASH        │    │   veLEASH       │
│   (SPL Token)   │◄──►│   (Staking)     │◄──►│   (Governance)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   LP Pool       │    │   Treasury      │    │   DAO           │
│   (wLEASH/LEASH)│    │   (Fee Router)  │    │   (Proposals)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   SHI Stable    │    │   Multi-Pair    │    │   Fee Farming   │
│   (Algorithmic) │◄──►│   Liquidity     │◄──►│   (Enhanced)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🏗️ System Components

### 1. **LEASH V2 Token (SPL-20)**
- **Mint Authority**: Permanently revoked (immutable supply)
- **Total Supply**: 1,000,000,000 LEASH
- **Transfer Tax**: 0% (no friction)
- **Freeze Authority**: Revoked (censorship resistance)

### 2. **SHI Algorithmic Stablecoin**
- **Target Price**: $1.00 USD (algorithmically maintained)
- **Mechanism**: Rebase-based stability with LP arbitrage incentives
- **Utility**: Foundation for expanded liquidity pools and fee generation
- **Integration**: Seamless pairing with any SPL token for enhanced DeFi operations

### 3. **xLEASH Staking Protocol**
- **Mechanism**: Single-sided staking with LP fee distribution
- **Reward Source**: Automated LP fee sweeps + protocol revenue + SHI pair fees
- **Lock Period**: [TBD - To be determined by Doggy DAO governance]
- **Exit Penalty**: [TBD - To be determined by Doggy DAO governance]

### 4. **veLEASH Governance System**
- **Lockup Multiplier**: [TBD - To be determined by Doggy DAO governance]
- **Voting Power**: [TBD - To be determined by Doggy DAO governance]
- **Governance Rights**: Treasury allocation, fee routing, parameter updates, SHI stability parameters
- **Lockup Periods**: [TBD - To be determined by Doggy DAO governance]

### 5. **Doggy DAO Treasury & Fee Routing**
- **Revenue Sources**: LP fees, protocol fees, partner integrations, SHI pair fees
- **Distribution**: [TBD - To be determined by Doggy DAO governance]
- **Timelock**: [TBD - To be determined by Doggy DAO governance]
- **Emergency Pause**: Multi-sig controlled (3-of-5)

### 6. **Cross-Chain NFT Bridge**
- **Self-Custody Staking**: Users stake NFTs on Ethereum while maintaining wallet control
- **Attestation System**: Cryptographic proof linking EVM wallets to Solana addresses
- **Wrapped NFT Minting**: Exact metadata and image copies on Solana blockchain
- **Cross-Chain Utility**: Access Solana DeFi while preserving original NFT ownership
- **Bridge Fees**: [TBD - To be determined by Doggy DAO governance]

### 7. **SHI-Enhanced Liquidity Pools**
- **Multi-Pair Strategy**: SHI pairs with major tokens (SOL, USDC, USDT, etc.)
- **Fee Optimization**: [TBD - To be determined by Doggy DAO governance]
- **Arbitrage Incentives**: Automated market making for SHI stability
- **Treasury Integration**: [TBD - To be determined by Doggy DAO governance]

### 8. **Cross-Chain Bridge (Wormhole)**
- **Ethereum → Solana**: LEASH V1 → wLEASH → LEASH V2
- **Solana → Ethereum**: LEASH V2 → wLEASH → LEASH V1
- **Liquidity Provision**: Automated market making for bridge pairs
- **SHI Bridge**: Cross-chain SHI stability maintenance
- **NFT Bridge**: Cross-chain NFT interoperability and utility
- **Bridge Fees**: [TBD - To be determined by Doggy DAO governance]
- **Migration Guide**: [Bridge LEASH from Ethereum to Solana](docs/how-to-bridge-leash.md) - Escape rebase risk!

## 🔐 Security Architecture

### Smart Contract Security
- **Audit Status**: Planned (Trail of Bits, OpenZeppelin)
- **Upgrade Mechanism**: Timelock-controlled program upgrades
- **Access Control**: Multi-sig governance for privileged operations
- **Emergency Controls**: Circuit breakers and pause mechanisms

### Economic Security
- **No Transfer Taxes**: Prevents MEV extraction and front-running
- **Immutable Supply**: Eliminates inflation risk and dilution
- **Fee Transparency**: All fees visible on-chain with clear routing
- **Slippage Protection**: Dynamic slippage tolerance based on pool depth
- **SHI Stability**: Multi-layered stability mechanisms with emergency controls

## 📊 Tokenomics & Economics

### Supply Distribution
```
LEASH V2 Token:
├─ Total Supply:         1,000,000,000 LEASH 
├─ Current Status:       Fully deployed and freely tradable on Solana
├─ No Airdrops:          All tokens are in circulation
├─ No Reserves:          No tokens held back for future distribution

SHI Distribution:
├─ Stability Fund:       [TBD - To be determined by Doggy DAO]
├─ Liquidity Mining:     [TBD - To be determined by Doggy DAO]
├─ Treasury:             [TBD - To be determined by Doggy DAO]
└─ Emergency:            [TBD - To be determined by Doggy DAO]
```

### Fee Structure
```
LP Trading Fees:
├─ Standard Pairs:       [TBD - To be determined by Doggy DAO]
├─ SHI Pairs:           [TBD - To be determined by Doggy DAO]
├─ Staking Rewards:     [TBD - To be determined by Doggy DAO]
└─ Burn Mechanism:      [TBD - To be determined by Doggy DAO]

Protocol Fees:
├─ Staking Withdrawal:   [TBD - To be determined by Doggy DAO]
├─ Governance Unlock:    [TBD - To be determined by Doggy DAO]
├─ Bridge Fees:         [TBD - To be determined by Doggy DAO]
└─ SHI Stability:       [TBD - To be determined by Doggy DAO]

Current Main Pool Status:
├─ Main Pool Fees:      75% routed to Ryoshi (subject to DAO governance)
├─ Fee Routing:         [TBD - To be determined by Doggy DAO]
└─ Future Changes:      Subject to Doggy DAO governance proposals
```

### Deflationary Mechanisms
- **LP Fee Burning**: [TBD - To be determined by Doggy DAO]
- **Governance Penalties**: [TBD - To be determined by Doggy DAO]
- **Treasury Buybacks**: [TBD - To be determined by Doggy DAO]
- **SHI Pair Premium**: [TBD - To be determined by Doggy DAO]

## 🚀 Development Roadmap

### Phase 1: Foundation
- [x] Token deployment and LP initialization
- [ ] Staking program development and testing
- [ ] Basic governance framework
- [ ] Security audit initiation

### Phase 2: Core Features
- [ ] Advanced staking with LP fee integration
- [ ] veLEASH governance implementation
- [ ] Treasury and fee routing systems
- [ ] Cross-chain bridge optimization
- [ ] **SHI algorithmic stablecoin development**

### Phase 3: NFT Bridge & Cross-Chain Interoperability
- [ ] **Cross-chain NFT bridge development** (Ethereum ↔ Solana)
- [ ] **Self-custody staking contracts** on Ethereum
- [ ] **Attestation system** for wallet linking and verification
- [ ] **Wrapped NFT minting** with exact metadata preservation
- [ ] **IPFS integration** for cross-chain image and metadata linking
- [ ] **Partner integrations** and allowlists

### Phase 4: SHI Algorithmic Stablecoin
- [ ] **SHI stability mechanisms and LP deployment**
- [ ] **Multi-pair liquidity strategy implementation**
- [ ] Advanced governance features
- [ ] Mobile applications
- [ ] Cross-chain DeFi integrations

### Phase 5: Ecosystem Expansion & Governance
- [ ] Full security audit completion
- [ ] Community governance activation
- [ ] Ecosystem partnerships
- [ ] **SHI ecosystem launch and stability establishment**
- [ ] **NFT bridge full deployment** and community adoption

## 🛠️ Technical Specifications

### Solana Program Requirements
- **Solana Version**: 1.17.0+
- **Anchor Framework**: 0.29.0+
- **Rust Version**: 1.70.0+
- **Node.js**: 18.0.0+

### Performance Targets
- **Transaction Finality**: < 400ms
- **TPS Capacity**: 65,000+ transactions/second
- **Gas Costs**: < $0.01 per transaction
- **Block Time**: 400ms average
- **SHI Stability**: 99%+ time within $0.98-$1.02 range

## 🔗 Integration & Partnerships

### DeFi Protocols
- **Raydium**: Primary DEX and liquidity provider
- **Jupiter**: Aggregated trading and routing
- **Marinade**: Liquid staking integration
- **Orca**: Concentrated liquidity pools
- ****SHI Pairs**: Enhanced liquidity and fee generation

### Cross-Chain Bridges
- **Wormhole**: Primary Ethereum bridge
- **Portal**: Alternative bridge option
- **Allbridge**: Multi-chain connectivity
- **SHI Bridge**: Cross-chain stability maintenance

### Infrastructure
- **Helius**: RPC and indexing services
- **QuickNode**: Enterprise-grade infrastructure
- **Alchemy**: Developer tooling and analytics

## 📚 Documentation & Resources

- **Technical Docs**: [docs/](./docs/)
- **API Reference**: [sdk/ts/](./sdk/ts/)
- **Smart Contracts**: [programs/](./programs/)
- **Test Suite**: [tests/](./tests/)
- **Deployment Scripts**: [scripts/](./scripts/)
- **Bridge Guide**: [How to Bridge LEASH to Solana](docs/how-to-bridge-leash.md)

## 🤝 Contributing

We welcome contributions from the community! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details on:

- Code review process
- Security reporting
- Development setup
- Testing requirements
- Documentation standards

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This software is provided "as is" without warranty of any kind. Users should conduct their own research and due diligence before interacting with these smart contracts. Cryptocurrency investments carry significant risk, and past performance does not guarantee future results.

---

**Let us continue our journey together my frens.**

*For technical questions and development discussions, join our  [Telegram](https://t.me/officialleashtoken).*
