# Bankr CLI Commands Cheat Sheet

## Installation & Setup
```bash
# Install CLI
npm install -g @bankr/cli --prefix ~/.local
export PATH="$HOME/.local/bin:$PATH"

# Login and setup
bankr login
bankr whoami  # Verify setup
```

## Core Commands

### Prompt & Trading
```bash
# Basic queries
bankr prompt "what is the price of ETH?"
bankr prompt "what are my balances?"
bankr prompt "show my portfolio"

# Trading commands
bankr prompt "buy $5 of BNKR on base"
bankr prompt "swap $50 USDC to ETH"
bankr prompt "sell all my BONK for SOL"
bankr prompt "swap 50% of my USDC to ETH"

# Chain-specific
bankr prompt "buy $10 of BONK on solana"
bankr prompt "swap $50 USDC from polygon to ETH on base"
```

### Automations
```bash
# Limit orders
bankr prompt "buy 100 BNKR if it drops 10%"
bankr prompt "sell my BNKR when it rises 20%"

# DCA & TWAP
bankr prompt "DCA $100 USDC into BNKR every day at 9am"
bankr prompt "sell 1000 BNKR over the next 4 hours"

# Management
bankr prompt "cancel my limit order"
bankr prompt "cancel all my automations"
```

### Token Launching
```bash
# Basic deployment
bankr prompt "deploy a token called MyAgent with symbol AGENT on base"
bankr prompt "launch a token called CoolBot on solana"

# With vaulting/vesting
bankr prompt "deploy a token with 30% vaulted for 30 days on base"
bankr prompt "deploy TeamToken with 20% vault, 7 day lockup, vesting over 30 days on base"

# Fee management
bankr prompt "how much fees have I earned?"
bankr prompt "claim my fees for TokenName"
```

### Advanced Features
```bash
# Leveraged trading
bankr prompt "long BTC/USD with 10x leverage"
bankr prompt "short $25 of ETH/USD"
bankr prompt "buy $50 BTC/USD with 5x leverage, 5% stop loss, 200% take profit"

# Polymarket
bankr prompt "what are the odds the eagles win?"
bankr prompt "bet $5 on eagles to win tonight"
bankr prompt "show my Polymarket positions"

# NFTs
bankr prompt "show my NFTs"
bankr prompt "buy this NFT: [opensea link]"
bankr prompt "mint from [manifold link]"

# Transfers
bankr prompt "send 100 USDC to 0x1234..."
bankr prompt "send 0.1 ETH to vitalik.eth"
bankr prompt "send $5 of DEGEN to @username"
```

## Job Management
```bash
# Check job status
bankr status <jobId>

# Cancel pending jobs
bankr cancel <jobId>

# Sign transactions (advanced)
bankr sign

# Submit signed transactions
bankr submit
```

## Configuration
```bash
# View config
bankr config get [key]

# Set config
bankr config set <key> <value>

# List available skills
bankr skills

# LLM management
bankr llm models
bankr llm setup <target>
bankr llm claude [args...]

# Logout
bankr logout
```

## API Integration Examples

### Direct API Calls
```bash
# Submit prompt
curl -X POST https://api.bankr.bot/agent/prompt \
  -H "Content-Type: application/json" \
  -H "X-API-Key: YOUR_API_KEY" \
  -d '{"prompt": "what is the price of ETH?"}'

# Poll for results
curl https://api.bankr.bot/agent/job/JOB_ID \
  -H "X-API-Key: YOUR_API_KEY"
```

### SDK Usage
```javascript
import { BankrClient } from '@bankr/sdk';

const client = new BankrClient({ 
  privateKey: '0x...',
  baseUrl: 'https://api.bankr.bot'
});

// Simple prompt
const result = await client.promptAndWait({ 
  prompt: 'what is the price of ETH?' 
});

// With options
const result = await client.promptAndWait({ 
  prompt: 'buy $10 of BNKR',
  chain: 'base',
  timeout: 300000
});
```

## Quick Test Commands for Interview
```bash
# Test basic connectivity
bankr whoami

# Test data queries
bankr prompt "price of ETH"
bankr prompt "what are my balances?"

# Test trading (small amounts)
bankr prompt "buy $1 of USDC with ETH"

# Test token launching (testnet first)
bankr prompt "deploy a token called TestToken with symbol TEST on base"

# Test automations
bankr prompt "buy 100 BNKR if it drops 10%"
bankr prompt "show my automations"
```

## Security Commands
```bash
# Check access controls
bankr whoami

# Verify API key permissions
bankr prompt "what are my permissions?"

# Test with dedicated wallet
bankr prompt "show my wallet address"
```
