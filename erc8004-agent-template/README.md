# ERC-8004 AI Trading Agent Template

## Quick Start Template for Hackathon Participants

This template provides a solid foundation for building AI trading agents with ERC-8004 trust layer and optional Kraken integration.

## Project Structure

```
erc8004-agent-template/
├── contracts/                 # Solidity smart contracts
│   ├── IdentityRegistry.sol
│   ├── ReputationRegistry.sol
│   ├── ValidationRegistry.sol
│   └── TradingAgent.sol
├── agent/                     # Python AI agent code
│   ├── strategy.py
│   ├── kraken_integration.py
│   ├── validation.py
│   └── main.py
├── frontend/                  # Dashboard UI (React)
├── scripts/                   # Deployment and utility scripts
└── tests/                     # Test suites
```

## Installation

```bash
# Clone and setup
git clone <template-url>
cd erc8004-agent-template

# Install dependencies
npm install
pip install -r requirements.txt

# Setup environment
cp .env.example .env
# Edit .env with your API keys and configuration
```

## Deployment

```bash
# Deploy to testnet
npm run deploy:testnet

# Deploy to mainnet (when ready)
npm run deploy:mainnet
```

## Running Your Agent

```bash
# Start the AI agent
python agent/main.py --network sepolia --kraken-enabled

# Run with specific strategy
python agent/main.py --strategy mean_reversion --risk-level medium
```

## Key Features

- ✅ ERC-8004 compliant identity, reputation, and validation registries
- ✅ Risk management and position sizing
- ✅ Optional Kraken API integration
- ✅ Real-time dashboard for monitoring
- ✅ Comprehensive test suite
- ✅ Documentation and examples

## Next Steps

1. Customize the trading strategy in `agent/strategy.py`
2. Configure your risk parameters
3. Set up Kraken API integration (optional)
4. Deploy your agent to the testnet
5. Test thoroughly before mainnet deployment

Good luck! 🚀
