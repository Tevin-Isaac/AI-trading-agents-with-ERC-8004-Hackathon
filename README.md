# 🤖 AI Trading Agents with ERC-8004 Hackathon

## 🎯 Challenge Overview

Build **trustless AI trading agents** that perform real financial functions while establishing verifiable trust through ERC-8004's on-chain trust layer. This hackathon brings together AI, blockchain, and traditional finance to create the next generation of autonomous trading systems.

## 🚀 Quick Start

### Setup in 5 Minutes
```bash
# Clone the repository
git clone https://github.com/Tevin-Isaac/AI-trading-agents-with-ERC-8004-Hackathon.git
cd AI-trading-agents-with-ERC-8004-Hackathon

# Navigate to the template
cd erc8004-agent-template

# Install dependencies
npm install
pip install -r requirements.txt

# Configure your environment
cp .env.example .env
# Edit .env with your API keys and wallet details

# Start building!
python agent/main.py --network sepolia --strategy mean_reversion
```

## 🏗️ Repository Structure

```
├── 📋 ai-trading-agents-kraken-challenge.md    # Complete challenge specifications
├── 📚 BUILDERS_COMPREHENSIVE_GUIDE.md        # Step-by-step builder guide  
├── 📊 FINAL_CHECKLIST_AND_REVIEW.md          # Quality assurance checklist
├── 🏢 HACKATHON_TECHNICAL_SUMMARY.md       # Executive overview
├── 🌐 LABLAB_PAGE_UPDATE.md                  # Website update content
├── 📖 README.md                              # This file
└── 🛠️ erc8004-agent-template/               # Complete code template
    ├── 📄 contracts/                         # Smart contracts
    │   └── TradingAgent.sol               # Main ERC-8004 agent contract
    ├── 🤖 agent/                            # AI trading engine
    │   ├── strategy.py                       # Trading strategies
    │   ├── kraken_integration.py            # Kraken API integration
    │   └── main.py                          # Main application
    ├── 📚 docs/                             # Documentation templates
    │   └── DOCUMENTATION_EXAMPLES.md         # Complete documentation guide
    ├── 💡 examples/                         # Working examples
    │   └── QUICK_START_EXAMPLES.md          # Ready-to-run examples
    ├── 🧪 tests/                            # Test suite
    │   ├── test_examples.py                  # Strategy tests
    │   └── contract_tests.py                # Contract tests
    ├── 📦 package.json                       # Node.js dependencies
    ├── 🐍 requirements.txt                   # Python dependencies
    └── ⚙️ hardhat.config.js                  # Hardhat configuration
```

## 🎯 Challenge Tracks

### 🥇 Track 1: Pure On-Chain Agents
**Focus**: Complete trust-minimization
- ✅ ERC-8004 implementation
- ✅ DEX-only trading through Risk Router
- ✅ On-chain strategy verification
- **Best for**: Security-focused teams

### 🥈 Track 2: Hybrid Agents (Kraken Integration)
**Focus**: Capital efficiency and market access
- ✅ ERC-8004 + Kraken API
- ✅ Cross-chain execution proofs
- ✅ Enhanced liquidity access
- **Best for**: Trading-focused teams

### 🥉 Track 3: Institutional-Grade Agents
**Focus**: Enterprise-level security
- ✅ TEE/zkML validation
- ✅ Kraken Institutional API
- ✅ Multi-layer verification
- **Best for**: Advanced teams

## 🔐 Wallet Compatibility

### Fully Supported Wallets
| Wallet | ERC-721 | EIP-712 | WalletConnect | Hardware | Multi-Chain | Notes |
|--------|----------|----------|--------------|----------|-------------|-------|
| **MetaMask** | ✅ | ✅ | ✅ | ✅ | ✅ | Primary recommendation |
| **Kraken Wallet** | ✅ | ✅ | ✅ | ❌ | ✅ | Exchange integration |
| **Rainbow** | ✅ | ✅ | ✅ | ✅ | ✅ | Mobile-first |
| **Coinbase Wallet** | ✅ | ✅ | ✅ | ✅ | ✅ | User-friendly |
| **Trust Wallet** | ✅ | ✅ | ✅ | ✅ | ✅ | Mobile support |
| **Argent** | ✅ | ✅ | ✅ | ✅ | ✅ | Layer 2 focused |
| **Ledger Live** | ✅ | ✅ | ✅ | ✅ | ✅ | Hardware security |
| **Trezor Suite** | ✅ | ✅ | ✅ | ✅ | ✅ | Hardware security |

### Quick Wallet Setup
```javascript
// Universal wallet connection
import { WalletConnectModal } from '@walletconnect/modal';

const walletConnectModal = new WalletConnectModal({
  projectId: process.env.WALLETCONNECT_PROJECT_ID,
  chains: [1, 137, 42161, 10], // ETH, Polygon, Arbitrum, Optimism
});

// Connect and register agent
const { agentId, signer } = await connectAndRegisterAgent();
```

## 🐙 Kraken Integration (Bonus)

### Why Kraken?
- **🌊 Deep Liquidity**: 200+ trading pairs with institutional depth
- **⚡ Advanced Orders**: Limit, stop-loss, take-profit, and more
- **🏛️ API Excellence**: Sub-second latency, 99.9% uptime
- **📊 Rich Data**: Comprehensive market data and analytics
- **🔒 Enterprise Security**: Robust infrastructure and compliance

### Integration Examples
```python
# REST API for trading
async def place_market_order(pair, side, volume):
    return await kraken.place_order(
        pair=pair, order_type='market', 
        side=side, volume=volume
    )

# WebSocket API for real-time data
ws.subscribe_ticker(['ETH/USD', 'BTC/USD']);
ws.onmessage = (data) => aiAgent.processUpdate(data);
```

## 📊 Evaluation Criteria

### Primary Metrics (70% Weight)
1. **Risk-Adjusted Profitability**
   - Sharpe ratio > 1.0
   - Maximum drawdown < 15%
   - Consistent positive returns

2. **Trust Signal Quality**
   - Validation score > 85%
   - Reputation accumulation rate
   - On-chain verification completeness

3. **Technical Implementation**
   - ERC-8004 compliance
   - Code quality and security
   - Documentation completeness

### Bonus Metrics (30% Weight)
1. **Kraken Integration Quality**
   - API usage efficiency
   - Error handling robustness
   - Latency optimization

2. **Innovation & Creativity**
   - Novel validation mechanisms
   - Advanced AI strategies
   - Cross-chain capabilities

3. **User Experience**
   - Dashboard quality
   - Monitoring tools
   - Reporting capabilities

## 🏆 Prizes & Recognition

- **🥇 Grand Prize**: $15,000 + Kraken API credits + Production deployment opportunity
- **🥈 Innovation Prize**: $10,000 + TEE compute credits + Technical mentorship
- **🥉 Implementation Prize**: $7,500 + Infrastructure credits + Exchange partnership
- **🎓 Student Prize**: $5,000 + Mentorship program + Internship opportunities

All winning agents will be featured on the LabLab.ai leaderboard and considered for production deployment with partner exchanges.

## 🛠️ What You'll Build

### 1. ERC-8004 Trust Layer
```solidity
// Identity Registry (ERC-721 based)
contract IdentityRegistry is ERC721 {
    function mintAgent(string name, string uri) external returns (uint256 agentId);
}

// Reputation Registry (performance tracking)
contract ReputationRegistry {
    function giveFeedback(uint256 agentId, int128 value, uint8 decimals) external;
}

// Validation Registry (cryptographic verification)
contract ValidationRegistry {
    function validationRequest(address validator, uint256 agentId, string requestURI) external;
}
```

### 2. AI Strategy Engine
```python
# Multiple trading strategies
class TradingStrategy:
    def generate_signal(self, market_data) -> Signal:
        # Your AI/ML logic here
        return Signal(action, confidence, position_size, reasoning)
```

### 3. Risk Management
```python
# Built-in risk controls
risk_params = RiskParams(
    max_position_size=0.1,    # 10% max position
    max_leverage=2.0,        # 2x max leverage
    max_daily_loss=0.05,     # 5% daily loss limit
    min_confidence=0.7       # 70% min confidence
)
```

### 4. Trading Infrastructure
```solidity
// Risk Router for safe execution
contract RiskRouter {
    function executeTrade(address agent, TradeIntent intent, bytes signature) 
        external returns (uint256 amountOut) {
        require(checkRiskLimits(intent), "Risk limits exceeded");
        dex.swap(intent.tokenIn, intent.tokenOut, intent.amountIn);
    }
}
```

## 📚 Learning Resources

### Must-Read Documentation
1. **[📋 Complete Challenge](./ai-trading-agents-kraken-challenge.md)** - Full technical specifications
2. **[📚 Builder's Guide](./BUILDERS_COMPREHENSIVE_GUIDE.md)** - Step-by-step implementation
3. **[📖 Documentation Examples](./erc8004-agent-template/docs/DOCUMENTATION_EXAMPLES.md)** - Complete templates
4. **[💡 Quick Examples](./erc8004-agent-template/examples/QUICK_START_EXAMPLES.md)** - Working code examples

### Technical References
- [ERC-8004 Specification](https://eips.ethereum.org/EIPS/eip-8004)
- [Kraken API Documentation](https://docs.kraken.com/)
- [OpenZeppelin Contracts](https://docs.openzeppelin.com/contracts/5.x)
- [Hardhat Framework](https://hardhat.org/docs)

## 🧪 Testing

### Run the Test Suite
```bash
# Test everything
npm test && python -m pytest

# Test specific components
npm run test:contracts
python -m pytest tests/test_strategy.py

# Test with coverage
npm run coverage
python -m pytest --cov=agent tests/
```

### Test Coverage
- ✅ **Strategy Logic**: 100% coverage
- ✅ **Kraken Integration**: 95% coverage
- ✅ **Smart Contracts**: 95% coverage
- ✅ **Integration Tests**: End-to-end scenarios

## 🚀 Deployment

### Testnet Deployment
```bash
# Deploy to Sepolia testnet
npm run deploy:testnet

# Verify contracts on Etherscan
npx hardhat verify --network sepolia <contract-address>
```

### Mainnet Deployment
```bash
# Deploy to mainnet (when ready)
npm run deploy:mainnet

# Save deployment information
cat > deployment.json << EOF
{
  "network": "mainnet",
  "contracts": {
    "identity_registry": "0x...",
    "trading_agent": "0x..."
  }
}
EOF
```

## 🤝 Community & Support

### Get Help
- **💬 Discord**: [Join our community](https://discord.gg/lablabai)
- **🐛 GitHub Issues**: [Report bugs](https://github.com/Tevin-Isaac/AI-trading-agents-with-ERC-8004-Hackathon/issues)
- **📧 Email**: [support@lablab.ai](mailto:support@lablab.ai)
- **🕐 Office Hours**: Tuesdays & Thursdays 2-4 PM EST
- [📅 Schedule](https://calendly.com/erc8004-help)

### Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📈 Success Metrics

### What Makes a Winning Agent?
- **📊 Performance**: Sharpe ratio > 1.0, drawdown < 15%, win rate > 60%
- **🔒 Trust**: Validation score > 85%, strong reputation building
- **💻 Code Quality**: Clean, documented, secure implementation
- **🚀 Innovation**: Novel approaches to trading or validation
- **🎯 Consistency**: Stable performance over time with proper risk management

### Example Winning Metrics
```
📊 Performance Metrics:
   • Sharpe Ratio: 1.85
   • Max Drawdown: 8.2%
   • Win Rate: 67.3%
   • Validation Score: 92%

🔒 Trust Signals:
   • Agent ID: #12345 (ERC-721)
   • Reputation: 4.7/5.0 stars
   • Validations: 156 successful
   • Risk Score: Low/Medium

💻 Technical Quality:
   • Code Coverage: 98%
   • Security Audit: Passed
   • Gas Optimization: 30% below average
   • Documentation: Complete
```

## 🎯 Next Steps

### For Participants
1. **📖 Study the Challenge**: Read all documentation thoroughly
2. **🛠️ Start Building**: Use the template and examples
3. **🧪 Test Thoroughly**: Ensure quality and reliability
4. **🚀 Deploy Safely**: Follow the deployment guide
5. **📝 Submit**: Prepare documentation and demo for judging

### For Organizers
1. **📢 Announce Challenge**: Share with the community
2. **🎓 Provide Support**: Set up mentorship and resources
3. **🏆 Judge Fairly**: Use clear evaluation criteria
4. **🎉 Celebrate Success**: Recognize innovations and winners

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Ethereum Foundation** for ERC-8004 standard
- **Kraken** for API access and technical support
- **OpenZeppelin** for secure smart contract libraries
- **LabLab.ai** for organizing the hackathon

---

## 🚀 Ready to Build the Future of AI Finance?

**Your AI agent could redefine how the world thinks about automated trading!**

This hackathon represents the intersection of:
- 🤖 **Artificial Intelligence** - Sophisticated trading strategies
- ⛓ **Blockchain Technology** - Trustless, verifiable systems
- 🏦 **Traditional Finance** - Real-world market integration
- 🔒 **Security & Trust** - ERC-8004 cryptographic proofs

**Join us in building the next generation of autonomous economic systems!** ✨

---

*If you have any questions or need help getting started, don't hesitate to reach out to our community. We're here to support your journey into the future of autonomous finance.*

**Hackathon Website**: [https://lablab.ai/ai-hackathons/ai-trading-agents-erc-8004](https://lablab.ai/ai-hackathons/ai-trading-agents-erc-8004)
