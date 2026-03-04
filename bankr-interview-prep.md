# Bankr.bot Interview Preparation Guide

## Platform Overview

Bankr is a Web3 AI companion platform that enables AI agents to execute blockchain transactions through natural language commands. The platform handles security, routing, and execution while developers focus on building.

### Core Value Proposition
- **AI Agents That Fund Themselves**: Build an AI agent, launch a token, trading fees pay for compute costs
- **No ongoing costs**: Self-sustaining through trading fees
- **Infrastructure layer**: Handles wallets, trading, security so you can focus on building

## Key Platform Capabilities

### 1. Natural Language Trading
- Send plain English commands that get executed as blockchain transactions
- Examples:
  - "swap $50 of ETH to USDC on base"
  - "buy $10 of BONK on solana"
  - "set a limit order to buy BNKR if it drops 10%"

### 2. Multi-Chain Support
**5 Supported Networks:**
- **Base** (Primary): Gas sponsorship, full feature support
- **Ethereum**: High-value operations, no gas sponsorship
- **Polygon**: Low-cost, Polymarket support, gas sponsorship
- **Unichain**: Uniswap's native L2, gas sponsorship
- **Solana**: High-speed SVM, limited gas sponsorship (1/day standard, 10/day Bankr Club)

### 3. Security First
- Sentinel security system checks for:
  - Malicious contract addresses
  - Phishing attempts
  - Unusual transaction patterns
  - Prompt injection attacks

### 4. Token Launching
- Deploy tokens on Base or Solana
- Built-in liquidity pool creation
- Fee sharing arrangements
- Vaulting and vesting options
- Social deployment via X (Twitter) tagging @bankrbot

## Integration Options

### 1. OpenClaw Skills (Fastest)
```bash
# Tell your OpenClaw agent:
install the bankr skill from https://github.com/BankrBot/openclaw-skills
```

### 2. Agent API
```bash
# Get API key from bankr.bot/api
curl -X POST https://api.bankr.bot/agent/prompt \
  -H "Content-Type: application/json" \
  -H "X-API-Key: YOUR_API_KEY" \
  -d '{"prompt": "what is the price of ETH?"}'
```

### 3. Bankr CLI
```bash
npm install -g @bankr/cli
bankr login
bankr prompt "what is the price of ETH?"
```

### 4. TypeScript SDK (Advanced)
```javascript
import { BankrClient } from '@bankr/sdk';
const client = new BankrClient({ privateKey: '0x...' });
const result = await client.promptAndWait({ 
  prompt: 'what is the price of ETH?' 
});
```

## Key Features & Commands

### Trading
- Swaps: "swap $50 of ETH to USDC"
- Buying: "buy $10 of BNKR on base"
- Selling: "sell all my BONK for SOL"
- Percentage trades: "sell half my BNKR"
- Multi-swaps: "swap 10 USDC to BNKR and 5 USDC to DEGEN"

### Automations
- Limit orders: "buy 100 BNKR if it drops 10%"
- Stop-loss: "sell all my DEGEN if it drops 20%"
- DCA: "DCA $100 USDC into BNKR every day at 9am"
- TWAP: "sell 1000 BNKR over the next 4 hours"

### Token Launching
- Basic: "deploy a token called MyAgent with symbol AGENT on base"
- With vaulting: "deploy a token with 30% vaulted for 30 days on base"
- Fee destination: "deploy a token with fees going to 0x1234... on base"
- Fee management: "claim my fees for TokenName"

### Advanced Features
- **Leveraged Trading (Avantis)**: "long BTC/USD with 10x leverage"
- **Polymarket**: "bet $5 on eagles to win tonight"
- **NFTs**: "buy this NFT: [opensea link]"
- **Transfers**: "send 100 USDC to 0x1234..."
- **Staking**: "stake 1000 BANKR"

## LLM Gateway
- Unified interface for multiple LLMs (Claude, GPT, Gemini, Kimi, Qwen)
- Pay with launch fees or wallet balance
- No need for separate API keys

## Security Best Practices
1. **Use dedicated agent wallets** - Don't use personal API keys
2. **Configure access controls** - IP allowlists, read-only mode
3. **API key security** - Revoke immediately if compromised
4. **Start with small amounts** - Only fund what's needed for testing

## Technical Architecture
- **Agent API**: Submit prompts, poll for results, sign/submit transactions
- **SDK**: For apps managing their own wallets (uses x402 micropayments)
- **CLI**: Built on Agent API, handles auth and polling automatically
- **OpenClaw Skills**: Plugin system for agent capabilities

## Business Model
- **Gas sponsorship**: Within limits for Base, Polygon, Unichain
- **Token launch fees**: Generate ongoing revenue for agents
- **LLM Gateway**: Pay-as-you-go or use launch fees
- **No ongoing costs**: Self-sustaining through trading fees

## Interview Talking Points

### Technical Depth
- Multi-chain architecture and routing
- Security system (Sentinel) implementation
- Natural language to transaction parsing
- Gas sponsorship mechanics
- Cross-chain swap aggregation

### Business Understanding
- Self-sustaining agent economics
- Token launchpad as revenue model
- Developer experience focus
- Integration with existing agent frameworks

### Use Cases
- Trading bots that fund themselves
- Social media deployed tokens
- Automated portfolio management
- Cross-chain arbitrage agents
- Gaming and NFT automation

## Setup Commands (Already Completed)
```bash
# CLI Installation
npm install -g @bankr/cli --prefix ~/.local
export PATH="$HOME/.local/bin:$PATH"
bankr --version  # 0.1.0-beta.6
```

## Next Steps for Interview
1. Create account at bankr.bot/api
2. Generate API key with Agent API access
3. Configure CLI: `bankr login`
4. Test basic commands: `bankr prompt "what are my balances?"`
5. Review API documentation for integration examples

## Key Differentiators
- **Natural language interface** vs traditional DeFi SDKs
- **Self-funding model** vs subscription-based
- **Multi-chain unified** vs chain-specific solutions
- **Built-in security** vs manual security implementation
- **Gas sponsorship** vs pay-per-transaction
