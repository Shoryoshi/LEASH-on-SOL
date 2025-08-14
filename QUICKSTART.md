# 🚀 LEASH V2 Quick Start Guide

> **Get up and running with LEASH V2 in under 10 minutes!**

This guide will help you quickly set up your development environment and start contributing to LEASH V2. Whether you're a seasoned Solana developer or just getting started, this guide has everything you need.

---

## ⚡ **Quick Setup (5 minutes)**

### **1. Prerequisites Check**
```bash
# Check if you have the required tools
node --version          # Should be 18.0.0+
rustc --version         # Should be 1.70.0+
solana --version        # Should be 1.17.0+
anchor --version        # Should be 0.29.0+
```

### **2. Clone & Setup**
```bash
# Fork and clone the repository
git clone https://github.com/Shoryoshi/LEASH-on-SOL.git
cd LEASH-on-SOL

# Install dependencies
pnpm install

# Build the programs
anchor build
```

### **3. Run Tests**
```bash
# Run the test suite
anchor test

# You should see: ✅ All tests passed!
```

**🎉 Congratulations! You're ready to contribute!**

---

## 🔧 **Full Development Setup (10 minutes)**

### **Step 1: Install Required Tools**

#### **Node.js & pnpm**
```bash
# Install Node.js 18+ from https://nodejs.org/
# Then install pnpm
npm install -g pnpm
```

#### **Rust & Cargo**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

#### **Anchor Framework**
```bash
# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.29.0
avm use 0.29.0
```

### **Step 2: Configure Solana**
```bash
# Set to devnet for development
solana config set --url devnet

# Create a new wallet (or use existing)
solana-keygen new --outfile ~/.config/solana/id.json

# Get some devnet SOL
solana airdrop 2
```

### **Step 3: Project Setup**
```bash
# Clone the repository
git clone https://github.com/Shoryoshi/LEASH-on-SOL.git
cd LEASH-on-SOL

# Install dependencies
pnpm install

# Build programs
anchor build

# Run tests
anchor test
```

---

## 🧪 **Testing Your Setup**

### **Run Basic Tests**
```bash
# Run all tests
anchor test

# Run specific test file
anchor test tests/staking.spec.ts

# Run with verbose output
anchor test --verbose
```

### **Expected Output**
```
  leash-staking
    ✅ Initializes the staking program
    ✅ Stakes LEASH tokens  
    ✅ Gets staking statistics

  3 passing (2s)
```

### **If Tests Fail**
1. **Check Solana connection**: `solana config get`
2. **Verify Anchor version**: `anchor --version`
3. **Check Rust version**: `rustc --version`
4. **Restart local validator**: `solana-test-validator -r --reset`

---

## 🏗️ **Project Structure Overview**

```
LEASH/
├─ programs/                    # Solana smart contracts
│  └─ leash-staking/          # Staking program (✅ Complete)
│      ├─ src/
│      │  ├─ lib.rs           # Main program entry point
│      │  ├─ state.rs         # Data structures
│      │  ├─ instructions/    # Instruction handlers
│      │  └─ errors.rs        # Custom error types
│      └─ Cargo.toml          # Rust dependencies
├─ tests/                      # Test suite
│  └─ staking.spec.ts         # Staking tests
├─ sdk/ts/                     # TypeScript SDK (🚧 Coming Soon)
├─ apps/                       # Frontend apps (🚧 Coming Soon)
└─ docs/                       # Documentation
```

---

## 🎯 **What You Can Do Right Now**

### **1. Explore the Code**
- **Read the staking contract** in `programs/leash-staking/src/`
- **Review the test suite** in `tests/staking.spec.ts`
- **Check the documentation** in `README.md` and `ROADMAP.md`

### **2. Run & Modify Tests**
```bash
# Run tests in watch mode
anchor test --watch

# Modify test values and see what happens
# Try changing stake amounts, reward rates, etc.
```

### **3. Build & Deploy Locally**
```bash
# Start local validator
solana-test-validator -r --reset

# Deploy to localnet
anchor deploy

# Test with real deployment
anchor test --provider.cluster localnet
```

### **4. Contribute Code**
- **Fix bugs** you find in the code
- **Add new test cases** for edge scenarios
- **Improve documentation** and comments
- **Optimize performance** where possible

---

## 🚀 **Next Steps**

### **Immediate (This Week)**
1. **Join our Telegram** for real-time discussions
2. **Pick an issue** from our GitHub project board
3. **Submit a small PR** (documentation, tests, etc.)
4. **Get familiar** with the codebase structure

### **Short Term (Next Month)**
1. **Complete the staking program** (add missing instructions)
2. **Start building veLEASH governance**
3. **Add comprehensive test coverage**
4. **Begin security review process**

### **Medium Term (Next Quarter)**
1. **Deploy to devnet** for community testing
2. **Build frontend applications**
3. **Integrate with other DeFi protocols**
4. **Prepare for mainnet launch**

---

## 🤝 **Get Help & Connect**

### **Community Channels**
- **Telegram**: [https://t.me/officialleatoken](https://t.me/officialleatoken)
- **GitHub**: [https://github.com/Shoryoshi/LEASH-on-SOL](https://github.com/Shoryoshi/LEASH-on-SOL)
- **X/Twitter**: [https://x.com/leashonbags](https://x.com/leashonbags)

### **Development Help**
- **Technical Questions**: Telegram development help channel
- **Code Reviews**: Submit pull requests for feedback
- **Architecture Discussions**: Telegram architecture channel

### **Resources**
- **Solana Docs**: [https://docs.solana.com/](https://docs.solana.com/)
- **Anchor Docs**: [https://www.anchor-lang.com/](https://www.anchor-lang.com/)
- **Rust Book**: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)

---

## 🎉 **You're Ready!**

**You now have everything you need to start contributing to LEASH V2!**

**Next steps:**
1. **Fork the repository** if you haven't already
2. **Join our Telegram** to meet the community
3. **Pick an issue** and start coding
4. **Submit your first pull request**

**Remember**: Every contribution helps, no matter how small! 🚀

---

*Questions? Issues? Just want to say hi? Join our Telegram and let's build the future of DeFi together!*
