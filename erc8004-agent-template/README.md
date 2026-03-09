# ERC-8004 AI Trading Agent Template

## 🚀 Quick Start Template for Hackathon Participants

**Now with Kraken CLI Partnership!** This template provides a solid foundation for building AI trading agents with ERC-8004 trust layer and **official Kraken CLI MCP integration**.

### 🌍 **Multi-Language Support**
- **🦀 Rust** - High-performance, memory-safe implementation
- **📗 TypeScript** - Modern Node.js/TypeScript for web integration

### � **Why This Template?**

- ✅ **Kraken CLI Partnership**: Official integration with Kraken's new AI-native CLI
- ✅ **MCP Server Support**: Built-in Model Context Protocol for seamless AI agent communication
- ✅ **ERC-8004 Compliant**: Identity, reputation, and validation registries on-chain
- ✅ **Paper Trading**: Safe testing environment with Kraken's simulation
- ✅ **50+ Agent Skills**: Leverage Kraken's pre-built trading strategies
- ✅ **Production Ready**: Battle-tested infrastructure from Kraken + your ERC-8004 trust layer
- ✅ **Multi-Language**: Choose your preferred language (Rust or TypeScript)

## Project Structure

```
erc8004-agent-template/
├── 🦀 src/                      # Rust implementation
│   ├── lib.rs                  # Main library entry
│   ├── main.rs                 # CLI application
│   ├── kraken.rs               # Kraken MCP client
│   ├── erc8004.rs              # ERC-8004 blockchain integration
│   ├── agent.rs                # Main trading agent
│   ├── strategies.rs           # Trading strategies
│   └── config.rs               # Configuration management
├── 📗 typescript/              # TypeScript implementation
│   ├── src/
│   │   ├── index.ts            # Main entry point
│   │   ├── config/             # Configuration
│   │   ├── kraken/             # Kraken MCP client
│   │   ├── erc8004/            # ERC-8004 integration
│   │   ├── agent/              # Trading agent
│   │   ├── strategies/         # Trading strategies
│   │   └── types/              # Type definitions
│   ├── package.json
│   └── tsconfig.json
├── contracts/                 # Solidity smart contracts
│   ├── IdentityRegistry.sol
│   ├── ReputationRegistry.sol
│   ├── ValidationRegistry.sol
│   └── TradingAgent.sol
├── config/                    # 🆕 Configuration files
│   ├── mcp_config.json        # MCP server configuration
│   └── kraken_services_config.json
├── examples/                  # 🆕 Usage examples
│   ├── mcp_examples.rs        # Rust examples
│   └── mcp_examples.ts        # TypeScript examples
├── frontend/                  # Dashboard UI (React)
├── scripts/                   # Deployment and utility scripts
└── tests/                     # Test suites
```

## 🛠️ Installation

### Prerequisites

1. **Install Kraken CLI** (NEW!)
   ```bash
   # Install Kraken CLI - AI-native trading interface
   curl -sSL https://github.com/krakenfx/kraken-cli/releases/latest/download/kraken-cli-linux-amd64.tar.gz | tar -xz
   sudo mv kraken /usr/local/bin/
   
   # Verify installation
   kraken --version
   ```

2. **Choose Your Language**
   ```bash
   # Rust (recommended for performance)
   rustc --version
   cargo --version
   
   # TypeScript/Node.js (recommended for web integration)
   node --version
   npm --version
   ```

### Project Setup

#### 🦀 **Rust Implementation**
```bash
# Install Rust dependencies
cargo build --release

# Setup environment
cp .env.example .env
# Edit .env with your configuration

# Run examples
cargo run --bin mcp-examples

# Start agent
cargo run --bin kraken-agent -- start --strategy mean_reversion --symbols BTCUSD,ETHUSD
```

#### 📗 **TypeScript Implementation**
```bash
# Install dependencies
cd typescript
npm install

# Setup environment
cp ../.env.example .env
# Edit .env with your configuration

# Build project
npm run build

# Run examples
npm run examples

# Start agent
npm run agent
```

### Environment Configuration

Edit `.env` file:

```bash
# Kraken CLI Configuration (Recommended)
KRAKEN_CLI_PATH=kraken
KRAKEN_MCP_SERVICES=market,account,paper  # Safe services
KRAKEN_ALLOW_DANGEROUS=false  # Keep false for hackathon

# Blockchain Configuration
ETHEREUM_RPC_URL=https://sepolia.infura.io/v3/YOUR_PROJECT_ID
PRIVATE_KEY=your_private_key_here

# Agent Configuration
AGENT_ID=hackathon_agent_001
MAX_POSITION_SIZE=0.1
RISK_LEVEL=medium
```

## Deployment

```bash
# Deploy to testnet
npm run deploy:testnet

# Deploy to mainnet (when ready)
npm run deploy:mainnet
```

## 🚀 Running Your Agent

### 🦀 **Rust Implementation**
```bash
# Start with Kraken CLI MCP integration
cargo run --bin kraken-agent -- start --strategy mean_reversion --symbols BTCUSD

# Paper trading only (safe for hackathon)
cargo run --bin kraken-agent -- start --paper --strategy momentum --symbols ETHUSD

# Test Kraken integration
cargo run --bin kraken-agent -- test-kraken

# Run comprehensive examples
cargo run --bin mcp-examples
```

### 📗 **TypeScript Implementation**
```bash
# Start with MCP integration
npm run agent -- --start --strategy mean_reversion --symbols BTCUSD

# Paper trading only
npm run agent -- --start --paper --strategy momentum --symbols ETHUSD

# Test integration
npm run agent -- --test-kraken

# Run examples
npm run examples
```

## 🌟 Key Features

### 🆕 **Kraken CLI Partnership Features**
- **AI-Native CLI**: Built specifically for AI agents (Claude, Cursor, Copilot compatible)
- **MCP Server**: Built-in Model Context Protocol for seamless integration
- **134 Commands**: Complete Kraken API coverage via CLI
- **50 Agent Skills**: Pre-built trading strategies and analysis tools
- **Paper Trading**: Safe simulation environment for testing
- **Real-time Data**: WebSocket streaming and live market data

### **ERC-8004 Trust Layer**
- ✅ On-chain identity registry for agent verification
- ✅ Reputation system for agent performance tracking
- ✅ Strategy validation registry for risk management
- ✅ Audit trail for all trading activities

### **Advanced Trading Features**
- ✅ Multi-asset support (crypto, stocks, forex, derivatives)
- ✅ Risk management and position sizing
- ✅ Real-time dashboard for monitoring
- ✅ Comprehensive test suite
- ✅ Production-ready deployment scripts

## 🎯 Hackathon Partnership Benefits

### **For Your Project**
- **Reduced Development**: Replace 360+ lines of custom API code with MCP integration
- **Professional Grade**: Use Kraken's official, battle-tested CLI
- **AI Agent Focus**: Built specifically for AI agents like yours
- **Safe Testing**: Paper trading environment for hackathon demos
- **50+ Skills**: Leverage pre-built trading strategies

### **For Kraken Partnership**
- **ERC-8004 Integration**: Your trust/reputation system adds value to their CLI
- **Hackathon Showcase**: Real-world use case of their new CLI
- **Agent Validation**: Your validation registry enhances agent reliability
- **Innovation**: First ERC-8004 + Kraken CLI integration

## 📚 Usage Examples

### **🦀 Rust Examples**
```rust
// Initialize Kraken integration
let config = AgentConfig::for_environment("development")?;
let agent = AgentFactory::create_agent(config).await?;

// Get market data
let market_data = agent.kraken_integration.get_market_data("BTCUSD").await?;
println!("BTC Price: ${}", extract_price(&market_data));

// Execute paper trade
let signal = TradingSignal {
    action: "buy".to_string(),
    pair: "BTCUSD".to_string(),
    confidence: 0.8,
    position_size: 0.1,
    price: None,
    reasoning: "RSI oversold condition".to_string(),
};

let result = agent.kraken_integration.execute_trade(signal, 0.1).await?;
println!("Trade Status: {}", result.status);
```

### **📗 TypeScript Examples**
```typescript
// Initialize Kraken integration
const configManager = ConfigManager.getInstance();
const config = configManager.getEnvironmentConfig('development');
const agent = await AgentFactory.createAgent(config);

// Get market data
const marketData = await agent.krakenIntegration.getMarketData('BTCUSD');
console.log('BTC Price: $' + extractPrice(marketData));

// Execute paper trade
const signal: TradingSignal = {
  action: 'buy',
  pair: 'BTCUSD',
  confidence: 0.8,
  positionSize: 0.1,
  reasoning: 'RSI oversold condition'
};

const result = await agent.krakenIntegration.executeTrade(signal, 0.1);
console.log('Trade Status:', result.status);
```

See `examples/` directory for comprehensive examples including:
- Multi-asset monitoring
- Risk management
- Portfolio tracking
- MCP tools discovery
- ERC-8004 integration

## 🚀 Next Steps for Hackathon

### **Phase 1: Setup & Testing (Day 1)**
1. ✅ Choose your language (Rust or TypeScript)
2. ✅ Install Kraken CLI and verify installation
3. ✅ Setup environment variables and configuration
4. ✅ Run example scripts to test integration
5. ✅ Deploy ERC-8004 contracts to Sepolia testnet

### **Phase 2: Strategy Development (Day 2)**
1. 🎯 Customize trading strategies for your chosen language
2. 🎯 Implement ERC-8004 validation logic
3. 🎯 Configure risk parameters and position sizing
4. 🎯 Test paper trading with different strategies

### **Phase 3: Integration & Polish (Day 3)**
1. 🔗 Integrate frontend dashboard with real-time data
2. 🔗 Add audit logging and monitoring
3. 🔗 Prepare demo with paper trading
4. 🔗 Document partnership benefits and language choice

### **Performance Comparison**
| Language | Performance | Memory Usage | Development Speed | Ecosystem |
|----------|-------------|--------------|------------------|-----------|
| Rust | ⚡ Fastest | 🔒 Lowest | 🐌 Moderate | 🛠️ Growing |
| TypeScript | 🚀 Fast | 📊 Moderate | ⚡ Fast | 🌐 Largest |

### **Production Deployment**
```bash
# Rust
cargo run --bin kraken-agent -- start --production

# TypeScript
npm run agent -- --start --production
```

## 🤝 Partnership Showcase

### **Demo Highlights**
- **Live Market Data**: Real-time price feeds via Kraken CLI
- **Paper Trading**: Safe demonstration of trading strategies
- **ERC-8004 Validation**: On-chain agent reputation and verification
- **Risk Management**: Position sizing based on agent reputation
- **Multi-Asset**: Support for crypto, stocks, forex, derivatives

### **Value Proposition**
- **For Hackathon**: Professional-grade trading infrastructure
- **For Kraken**: First ERC-8004 integration with trust layer
- **For Judges**: Innovation in DeFi + AI agent validation

## 📖 Documentation

- [Kraken CLI Documentation](https://github.com/krakenfx/kraken-cli)
- [MCP Protocol Specification](https://modelcontextprotocol.io/)
- [ERC-8004 Standard](https://eips.ethereum.org/EIPS/eip-8004)
- [Agent Skills Index](https://github.com/krakenfx/kraken-cli/blob/main/skills/INDEX.md)

## 🎉 Good luck with the hackathon! 🚀

**Remember**: This template gives you a production-ready foundation with official Kraken CLI partnership. Focus on your unique ERC-8004 value proposition and trading strategies!
