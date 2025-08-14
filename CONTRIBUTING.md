# 🤝 Contributing to LEASH V2

> **Join the pack and help build the future of decentralized governance!**

We're thrilled that you're interested in contributing to LEASH V2! This document provides everything you need to know to get started, whether you're a developer, designer, community manager, or just someone passionate about DeFi.

---

## 🎯 **How to Contribute**

### **🐛 Bug Reports**
Found a bug? Help us squash it!
- **Check existing issues** first to avoid duplicates
- **Use the bug report template** for structured reporting
- **Include reproduction steps** and expected vs. actual behavior
- **Add screenshots/logs** if applicable

### **💡 Feature Requests**
Have an idea for a new feature?
- **Search existing requests** to avoid duplicates
- **Use the feature request template** for structured proposals
- **Explain the problem** your feature solves
- **Describe the solution** you envision
- **Consider implementation complexity** and user impact

### **🔧 Code Contributions**
Ready to write some code?
- **Fork the repository** and create a feature branch
- **Follow our coding standards** (see below)
- **Write tests** for new functionality
- **Update documentation** as needed
- **Submit a pull request** with clear description

### **📚 Documentation**
Help improve our docs!
- **Fix typos and grammar** errors
- **Add missing information** or examples
- **Improve clarity** of existing content
- **Translate content** to other languages
- **Create tutorials** or guides

### **🧪 Testing**
Help ensure quality!
- **Test new features** on devnet
- **Report edge cases** and unexpected behavior
- **Suggest test scenarios** we might have missed
- **Help improve test coverage**

### **🌐 Community Building**
Help grow our community!
- **Answer questions** in Telegram
- **Share knowledge** with newcomers
- **Organize events** or meetups
- **Create content** (blog posts, videos, tutorials)

---

## 🚀 **Getting Started**

### **Prerequisites**
- **Git** installed on your machine
- **Node.js** 18.0.0 or higher
- **Rust** 1.70.0 or higher
- **Solana CLI** tools
- **Anchor Framework** 0.29.0 or higher

### **Development Setup**
1. **Fork the repository**
   ```bash
   git clone https://github.com/Shoryoshi/LEASH-on-SOL.git
   cd LEASH-on-SOL
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Build the programs**
   ```bash
   anchor build
   ```

4. **Run tests**
   ```bash
   anchor test
   ```

5. **Start local validator** (optional)
   ```bash
   solana-test-validator -r --reset
   anchor deploy
   ```

### **IDE Setup**
We recommend using **VS Code** with these extensions:
- **Rust Analyzer** - Rust language support
- **Anchor** - Solana/Anchor development
- **TypeScript** - TypeScript support
- **GitLens** - Git integration
- **Error Lens** - Inline error display

---

## 📝 **Coding Standards**

### **Rust (Smart Contracts)**
- **Follow Rust conventions** and use `cargo fmt`
- **Use meaningful names** for variables and functions
- **Add comprehensive comments** explaining complex logic
- **Handle errors gracefully** with proper error types
- **Write unit tests** for all public functions
- **Use safe math operations** to prevent overflows

### **TypeScript (SDK & Tests)**
- **Use strict TypeScript** configuration
- **Follow ESLint rules** and use `pnpm lint:fix`
- **Use meaningful variable names** and types
- **Add JSDoc comments** for public functions
- **Write comprehensive tests** with clear assertions
- **Handle async operations** properly

### **General Principles**
- **Keep functions small** and focused on single responsibility
- **Use descriptive commit messages** following conventional commits
- **Write self-documenting code** that's easy to understand
- **Consider security implications** of every change
- **Test edge cases** and error conditions

---

## 🔄 **Pull Request Process**

### **Before Submitting**
1. **Ensure tests pass** locally
2. **Update documentation** if needed
3. **Check code formatting** and linting
4. **Rebase on main** to avoid conflicts
5. **Write clear commit messages**

### **Pull Request Template**
```markdown
## Description
Brief description of what this PR accomplishes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
```

### **Review Process**
1. **Automated checks** must pass (CI/CD)
2. **Code review** by maintainers
3. **Address feedback** and make requested changes
4. **Maintainer approval** required for merge
5. **Squash and merge** to main branch

---

## 🏗️ **Project Structure**

```
LEASH/
├─ programs/                    # Solana programs (Rust)
│  ├─ leash-staking/          # Staking contract
│  ├─ veleash/                # Governance contract
│  ├─ dao/                    # DAO framework
│  ├─ treasury/               # Treasury management
│  ├─ gatekeeper/             # Access control
│  └─ merkle-distributor/     # Token distribution
├─ sdk/ts/                    # TypeScript SDK
├─ apps/                      # Frontend applications
├─ tests/                     # Test suite
├─ scripts/                   # Deployment scripts
└─ docs/                      # Documentation
```

### **Where to Contribute**
- **Smart Contracts**: `programs/` directory
- **SDK & APIs**: `sdk/ts/` directory
- **Frontend**: `apps/` directory
- **Tests**: `tests/` directory
- **Documentation**: `docs/` directory

---

## 🧪 **Testing Guidelines**

### **Unit Tests**
- **Test all public functions** with various inputs
- **Cover edge cases** and error conditions
- **Mock external dependencies** when possible
- **Use descriptive test names** that explain the scenario

### **Integration Tests**
- **Test complete workflows** end-to-end
- **Use real Solana programs** on devnet
- **Test cross-program interactions** thoroughly
- **Verify state changes** and side effects

### **Security Tests**
- **Test access controls** and permissions
- **Verify input validation** and sanitization
- **Check for common vulnerabilities** (overflow, reentrancy)
- **Test emergency procedures** and pause mechanisms

---

## 📚 **Documentation Standards**

### **Code Documentation**
- **Document all public functions** with clear descriptions
- **Explain complex algorithms** and business logic
- **Provide usage examples** for common scenarios
- **Document error conditions** and recovery procedures

### **User Documentation**
- **Write for the target audience** (developers vs. users)
- **Use clear, concise language** and avoid jargon
- **Include screenshots** and visual aids when helpful
- **Provide step-by-step instructions** for complex processes

### **API Documentation**
- **Document all endpoints** with request/response examples
- **Explain authentication** and authorization requirements
- **Provide error codes** and troubleshooting guides
- **Include rate limiting** and usage guidelines

---

## 🚨 **Security Guidelines**

### **Code Security**
- **Never commit secrets** or private keys
- **Use secure random generation** for cryptographic operations
- **Validate all inputs** and sanitize data
- **Implement proper access controls** and permissions
- **Use safe math operations** to prevent overflows

### **Reporting Security Issues**
- **DO NOT** create public issues for security vulnerabilities
- **Email security@leash.community** with details
- **Include proof-of-concept** if possible
- **Allow time** for investigation and fix
- **Coordinate disclosure** with the security team

---

## 🎉 **Recognition & Rewards**

### **Contributor Recognition**
- **GitHub profile** shows your contributions
- **Contributor hall of fame** in our documentation
- **Special Telegram roles** for active contributors
- **Mention in release notes** for significant contributions

### **Community Rewards**
- **Early access** to new features and beta testing
- **Exclusive Telegram channels** for contributors
- **Invitation to contributor events** and meetups
- **LEASH token rewards** for major contributions (TBD)

---

## 🤔 **Need Help?**

### **Getting Started**
- **Read the documentation** in the `docs/` directory
- **Join our Telegram** for real-time help
- **Check existing issues** for similar problems
- **Review pull requests** to see how others contribute

### **Technical Questions**
- **Telegram**: Development help channel
- **GitHub Discussions**: Use the Discussions tab
- **Stack Overflow**: Tag questions with `leash-v2`

### **Community Support**
- **Telegram**: General community channels
- **Telegram**: Community group
- **Twitter**: @LEASH_Community

---

## 📋 **Code of Conduct**

### **Our Standards**
- **Be respectful** and inclusive of all community members
- **Use welcoming language** and constructive feedback
- **Focus on the code** and technical merits
- **Respect different viewpoints** and experiences
- **Show empathy** towards other community members

### **Unacceptable Behavior**
- **Harassment** or discrimination of any kind
- **Trolling** or inflammatory comments
- **Spam** or off-topic discussions
- **Personal attacks** or insults
- **Sharing private information** without consent

---

## 🎯 **Ready to Contribute?**

**Great! Here's your next steps:**

1. **Fork the repository** and clone it locally
2. **Set up your development environment** following the setup guide
3. **Pick an issue** from our GitHub project board
4. **Create a feature branch** and start coding
5. **Submit a pull request** when ready
6. **Join our Telegram** to connect with other contributors

**Remember**: Every contribution, no matter how small, helps make LEASH V2 better for everyone! 🚀

---

*This document is a living guide that will be updated as our project evolves. Have suggestions for improvements? Submit a pull request!*
