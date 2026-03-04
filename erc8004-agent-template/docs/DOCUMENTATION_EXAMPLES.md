# Documentation Examples for ERC-8004 AI Trading Agents

## 📚 Complete Documentation Templates

### 1. Project README Template

```markdown
# [Your Agent Name] - AI Trading Agent

## 🎯 Overview
[Your agent description - what makes it unique, strategy focus, etc.]

## 🏗️ Architecture
- **ERC-8004 Identity**: Agent registered as NFT #12345
- **Strategy Type**: Mean reversion with Bollinger Bands
- **Risk Management**: Dynamic position sizing with 5% max drawdown
- **Validation**: TEE attestations + stake-secured verification

## 🚀 Quick Start

### Prerequisites
- Node.js 18+
- Python 3.9+
- MetaMask or compatible wallet
- Testnet ETH for gas fees

### Installation
```bash
git clone https://github.com/yourusername/your-agent
cd your-agent
npm install
pip install -r requirements.txt
```

### Configuration
```bash
cp .env.example .env
# Edit .env with your configuration
```

### Deployment
```bash
# Deploy contracts
npm run deploy:testnet

# Register agent
python scripts/register_agent.py --network sepolia

# Start trading
python agent/main.py --network sepolia --config config.json
```

## 📊 Performance
- **Sharpe Ratio**: 1.85
- **Max Drawdown**: 8.2%
- **Win Rate**: 67.3%
- **Validation Score**: 92%

## 🛠️ API Documentation
See [API_DOCS.md](./API_DOCS.md) for detailed API reference.

## 🧪 Testing
```bash
# Run all tests
npm test && python -m pytest

# Run specific test suites
npm run test:contracts
python -m pytest tests/test_strategy.py
```

## 📝 License
MIT License - see [LICENSE](LICENSE) file for details.
```

### 2. API Documentation Template

```markdown
# API Documentation - [Your Agent Name]

## 🔐 Authentication
All API requests require EIP-712 signed messages for security.

## 📡 Endpoints

### Agent Information
```http
GET /api/v1/agent/info
```

**Response:**
```json
{
  "agentId": "12345",
  "name": "MeanReversionAgent",
  "strategy": "mean_reversion",
  "status": "active",
  "performance": {
    "sharpeRatio": 1.85,
    "maxDrawdown": 0.082,
    "winRate": 0.673
  },
  "registration": {
    "identityRegistry": "0x...",
    "reputationScore": 92,
    "validationCount": 156
  }
}
```

### Trading Signals
```http
POST /api/v1/signals/generate
```

**Request:**
```json
{
  "pair": "ETH/USD",
  "timeframe": "1h",
  "riskLevel": "medium"
}
```

**Response:**
```json
{
  "signal": {
    "action": "buy",
    "confidence": 0.78,
    "positionSize": 0.15,
    "stopLoss": 2850.0,
    "takeProfit": 3200.0,
    "reasoning": "Price below lower Bollinger Band with RSI oversold"
  },
  "marketData": {
    "price": 2950.0,
    "volume": 15420.5,
    "timestamp": "2024-03-04T10:30:00Z"
  }
}
```

### Performance History
```http
GET /api/v1/performance/history?period=30d
```

**Response:**
```json
{
  "period": "30d",
  "totalReturn": 0.127,
  "sharpeRatio": 1.85,
  "maxDrawdown": 0.082,
  "winRate": 0.673,
  "dailyReturns": [
    {"date": "2024-02-03", "return": 0.0042},
    {"date": "2024-02-04", "return": -0.0018}
  ],
  "trades": [
    {
      "id": "trade_123",
      "pair": "ETH/USD",
      "action": "buy",
      "entryPrice": 2950.0,
      "exitPrice": 3120.0,
      "pnl": 0.058,
      "timestamp": "2024-03-01T14:30:00Z"
    }
  ]
}
```

## 🔗 Webhooks
Configure webhooks to receive real-time updates:

### Trade Executed
```json
{
  "event": "trade.executed",
  "data": {
    "tradeId": "trade_123",
    "agentId": "12345",
    "pair": "ETH/USD",
    "action": "buy",
    "amount": 0.5,
    "price": 2950.0,
    "timestamp": "2024-03-04T10:30:00Z"
  }
}
```

### Validation Completed
```json
{
  "event": "validation.completed",
  "data": {
    "agentId": "12345",
    "validationId": "val_456",
    "score": 92,
    "validator": "0x...",
    "evidence": "https://ipfs.io/ipfs/Qm...",
    "timestamp": "2024-03-04T10:35:00Z"
  }
}
```
```

### 3. Smart Contract Documentation Template

```solidity
/**
 * @title TradingAgent
 * @dev ERC-8004 compliant AI trading agent
 * 
 * This contract implements a trustless AI trading agent that:
 * 1. Registers identity on ERC-8004 Identity Registry
 * 2. Accumulates reputation through performance tracking
 * 3. Validates actions through cryptographic proofs
 * 
 * @author Your Name
 * @notice Use this contract to deploy your AI trading agent
 * @dev All functions require proper EIP-712 signatures
 */
contract TradingAgent is ERC721, Ownable {
    
    // ========================================
    // STATE VARIABLES
    // ========================================
    
    /// @notice ERC-8004 Identity Registry contract
    IIdentityRegistry public immutable identityRegistry;
    
    /// @notice Reputation Registry for performance tracking
    IReputationRegistry public immutable reputationRegistry;
    
    /// @notice Validation Registry for cryptographic verification
    IValidationRegistry public immutable validationRegistry;
    
    /// @notice Risk Router for position management
    IRiskRouter public immutable riskRouter;
    
    /// @notice Unique identifier for this agent
    uint256 public agentId;
    
    /// @notice Wallet that controls the agent
    address public agentWallet;
    
    /// @notice Track executed trades to prevent duplicates
    mapping(bytes32 => bool) public executedTrades;
    
    // ========================================
    // EVENTS
    // ========================================
    
    /**
     * @notice Emitted when a trade is executed
     * @param tradeHash Unique hash of the trade
     * @param tokenIn Input token address
     * @param tokenOut Output token address
     * @param amountIn Input amount
     * @param amountOut Output amount
     * @param timestamp Execution timestamp
     */
    event TradeExecuted(
        bytes32 indexed tradeHash,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        uint256 timestamp
    );
    
    /**
     * @notice Emitted when strategy is validated
     * @param strategyHash Hash of the strategy
     * @param validator Address of the validator
     * @param score Validation score (0-100)
     */
    event StrategyValidated(
        bytes32 indexed strategyHash,
        address indexed validator,
        uint8 score
    );
    
    // ========================================
    // STRUCTS
    // ========================================
    
    /**
     * @notice Trade intent structure
     * @dev All trade parameters must be signed by agent owner
     */
    struct TradeIntent {
        address tokenIn;           // Input token address
        address tokenOut;          // Output token address
        uint256 amountIn;         // Input amount
        uint256 minAmountOut;      // Minimum output amount (slippage protection)
        uint256 deadline;          // Trade execution deadline
        uint256 maxSlippage;       // Maximum slippage in basis points
        bytes32 strategyHash;      // Hash of the strategy being used
    }
    
    // ========================================
    // CONSTRUCTOR
    // ========================================
    
    /**
     * @notice Deploy a new trading agent
     * @param _identityRegistry Address of the Identity Registry
     * @param _reputationRegistry Address of the Reputation Registry
     * @param _validationRegistry Address of the Validation Registry
     * @param _riskRouter Address of the Risk Router
     * @param agentName Human-readable name for the agent
     * @param agentURI IPFS URI with agent metadata
     * @param _agentWallet Wallet that controls the agent
     */
    constructor(
        address _identityRegistry,
        address _reputationRegistry,
        address _validationRegistry,
        address _riskRouter,
        string memory agentName,
        string memory agentURI,
        address _agentWallet
    ) ERC721("TradingAgent", "TRADE") {
        // Validate inputs
        require(_identityRegistry != address(0), "Invalid identity registry");
        require(_reputationRegistry != address(0), "Invalid reputation registry");
        require(_validationRegistry != address(0), "Invalid validation registry");
        require(_riskRouter != address(0), "Invalid risk router");
        require(_agentWallet != address(0), "Invalid agent wallet");
        
        // Set immutable variables
        identityRegistry = IIdentityRegistry(_identityRegistry);
        reputationRegistry = IReputationRegistry(_reputationRegistry);
        validationRegistry = IValidationRegistry(_validationRegistry);
        riskRouter = IRiskRouter(_riskRouter);
        agentWallet = _agentWallet;
        
        // Register agent identity
        agentId = identityRegistry.mintAgent(agentName, agentURI);
        _mint(msg.sender, agentId);
    }
    
    // ========================================
    // CORE FUNCTIONS
    // ========================================
    
    /**
     * @notice Execute a signed trade intent
     * @param intent The trade intent to execute
     * @param signature EIP-712 signature from agent owner
     * @return amountOut Actual amount received from the trade
     * 
     * @dev Requirements:
     * - Caller must be agent wallet
     * - Trade must not be expired
     * - Trade must not have been executed before
     * - Signature must be valid
     */
    function executeSignedTrade(
        TradeIntent calldata intent,
        bytes calldata signature
    ) external returns (uint256 amountOut) {
        // Validate caller
        require(msg.sender == agentWallet, "Only agent wallet can execute");
        
        // Validate trade parameters
        require(block.timestamp <= intent.deadline, "Trade expired");
        
        // Prevent duplicate execution
        bytes32 tradeHash = keccak256(abi.encode(intent));
        require(!executedTrades[tradeHash], "Trade already executed");
        
        // Verify signature
        address signer = ECDSA.recover(
            MessageHashUtils.toEthSignedMessageHash(tradeHash),
            signature
        );
        require(signer == owner(), "Invalid signature");
        
        // Execute trade through risk router
        amountOut = riskRouter.executeTrade(address(this), intent, signature);
        
        // Mark trade as executed
        executedTrades[tradeHash] = true;
        
        // Emit event
        emit TradeExecuted(
            tradeHash,
            intent.tokenIn,
            intent.tokenOut,
            intent.amountIn,
            amountOut,
            block.timestamp
        );
        
        return amountOut;
    }
    
    /**
     * @notice Request validation for a strategy
     * @param validator Address of the validator contract
     * @param strategyHash Hash of the strategy to validate
     * @param evidenceURI IPFS URI with validation evidence
     */
    function requestValidation(
        address validator,
        bytes32 strategyHash,
        string calldata evidenceURI
    ) external onlyOwner {
        bytes32 requestHash = keccak256(abi.encode(strategyHash, evidenceURI));
        
        validationRegistry.validationRequest(
            validator,
            agentId,
            evidenceURI,
            requestHash
        );
    }
    
    /**
     * @notice Record performance metrics
     * @param pnl Profit and loss amount
     * @param decimals Number of decimals for pnl
     * @param period Time period for the metrics
     */
    function recordPerformance(
        int128 pnl,
        uint8 decimals,
        string calldata period
    ) external onlyOwner {
        reputationRegistry.giveFeedback(
            agentId,
            pnl,
            decimals,
            "tradingYield",
            period,
            "",
            "",
            bytes32(0)
        );
    }
}
```

### 4. User Guide Template

```markdown
# User Guide - [Your Agent Name]

## 🎯 Getting Started

### 1. Wallet Setup
1. Install MetaMask browser extension
2. Create a new wallet or import existing
3. Switch to Sepolia testnet
4. Get test ETH from [sepoliafaucet.com](https://sepoliafaucet.com)

### 2. Agent Registration
1. Visit the agent registration page
2. Connect your wallet
3. Click "Register New Agent"
4. Pay gas fee (~0.001 ETH)
5. Save your Agent ID

### 3. Configuration
```json
{
  "agentId": "12345",
  "strategy": "mean_reversion",
  "riskLevel": "medium",
  "tradingPairs": ["ETH/USD", "BTC/USD"],
  "maxPositionSize": 0.1,
  "stopLoss": 0.05,
  "takeProfit": 0.1
}
```

## 📊 Monitoring Your Agent

### Dashboard Features
- **Real-time PnL**: Track profit and loss
- **Performance Metrics**: Sharpe ratio, drawdown, win rate
- **Trade History**: Complete audit trail
- **Risk Alerts**: Notifications for unusual activity

### Key Metrics to Watch
- **Sharpe Ratio**: Above 1.0 is good
- **Max Drawdown**: Below 15% is acceptable
- **Win Rate**: Above 60% is excellent
- **Validation Score**: Above 85% for trust

## 🔧 Advanced Configuration

### Strategy Parameters
```python
# Mean reversion strategy example
strategy_config = {
    "lookback_period": 20,      # Moving average period
    "entry_threshold": 2.0,    # Standard deviations
    "exit_threshold": 0.5,     # Exit signal threshold
    "position_sizing": "kelly", # Position sizing method
    "risk_management": {
        "max_position_size": 0.1,
        "stop_loss": 0.05,
        "take_profit": 0.1
    }
}
```

### Risk Management
- **Position Sizing**: Never risk more than 10% per trade
- **Stop Loss**: Automatically exit losing positions
- **Take Profit**: Lock in gains at predefined levels
- **Drawdown Protection**: Stop trading if losses exceed threshold

## 🚨 Troubleshooting

### Common Issues

#### Agent Not Responding
1. Check wallet connection
2. Verify gas fees are paid
3. Check network status
4. Review agent logs

#### Trade Execution Failed
1. Verify sufficient balance
2. Check slippage settings
3. Confirm market hours
4. Review risk limits

#### Validation Score Low
1. Improve strategy consistency
2. Reduce drawdown
3. Increase win rate
4. Add more validation evidence

### Getting Help
- **Documentation**: [link to docs]
- **Discord Community**: [discord link]
- **Support Email**: support@youragent.com
- **GitHub Issues**: [issues link]

## 📈 Performance Optimization

### Best Practices
1. **Start Small**: Begin with minimum position sizes
2. **Monitor Closely**: Check performance daily
3. **Adjust Parameters**: Optimize based on results
4. **Stay Updated**: Keep software current

### Advanced Tips
- Use multiple timeframes for analysis
- Implement correlation analysis
- Add sentiment data signals
- Consider market regime changes
```

### 5. Deployment Guide Template

```markdown
# Deployment Guide - [Your Agent Name]

## 🚀 Production Deployment

### 1. Pre-Deployment Checklist
- [ ] All tests passing (100% coverage)
- [ ] Security audit completed
- [ ] Gas optimization done
- [ ] Documentation updated
- [ ] Monitoring configured

### 2. Contract Deployment

#### Sepolia Testnet
```bash
# Deploy to testnet
npx hardhat run scripts/deploy.js --network sepolia

# Verify contracts
npx hardhat verify --network sepolia <contract-address>
```

#### Mainnet Deployment
```bash
# Deploy to mainnet (CAUTION!)
npx hardhat run scripts/deploy.js --network mainnet

# Verify contracts
npx hardhat verify --network mainnet <contract-address>
```

### 3. Infrastructure Setup

#### Server Requirements
- **CPU**: 4+ cores
- **RAM**: 8GB+ 
- **Storage**: 100GB+ SSD
- **Network**: 1Gbps+ connection

#### Environment Variables
```bash
# Blockchain
ETHEREUM_RPC_URL=https://mainnet.infura.io/v3/YOUR_PROJECT_ID
PRIVATE_KEY=your_private_key_here
CONTRACT_ADDRESS=0x...

# API Keys
KRAKEN_API_KEY=your_kraken_api_key
KRAKEN_API_SECRET=your_kraken_secret

# Monitoring
DISCORD_WEBHOOK_URL=your_discord_webhook
SLACK_WEBHOOK_URL=your_slack_webhook

# Database
DATABASE_URL=postgresql://user:pass@localhost/agent_db
```

### 4. Monitoring & Alerting

#### Prometheus Metrics
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'trading-agent'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/metrics'
```

#### Grafana Dashboard
- Agent performance metrics
- Trade execution statistics
- Error rates and alerts
- System health monitoring

### 5. Security Considerations

#### Smart Contract Security
- Use OpenZeppelin contracts
- Implement access controls
- Add emergency pause functions
- Regular security audits

#### API Security
- Rate limiting
- Input validation
- Authentication/Authorization
- HTTPS only

#### Operational Security
- Multi-signature wallets
- Hardware security modules
- Regular key rotation
- Incident response plan

### 6. Maintenance

#### Regular Tasks
- Update dependencies
- Monitor gas costs
- Backup databases
- Review performance metrics

#### Emergency Procedures
1. **Pause Trading**: Use emergency stop function
2. **Notify Users**: Send alerts via all channels
3. **Investigate**: Check logs and metrics
4. **Recover**: Restore from backup if needed
5. **Communicate**: Post-mortem and updates
```

These documentation templates provide comprehensive coverage for all aspects of building and deploying ERC-8004 AI trading agents, making it easy for developers to understand and implement the requirements.
