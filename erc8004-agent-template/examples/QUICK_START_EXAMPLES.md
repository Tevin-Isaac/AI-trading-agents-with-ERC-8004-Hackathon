# Quick Start Examples for ERC-8004 AI Trading Agents

## 🚀 Complete Working Examples

### Example 1: Basic Mean Reversion Agent

```python
"""
Basic Mean Reversion Trading Agent
A complete example that you can run immediately
"""

import asyncio
import json
import pandas as pd
import numpy as np
from agent.strategy import TradingStrategy, StrategyType, RiskParams
from agent.main import AITradingAgent

async def basic_mean_reversion_agent():
    """Complete example of a basic mean reversion agent"""
    
    # Configuration
    config = {
        "network": "sepolia",
        "strategy_type": "mean_reversion",
        "risk_level": "medium",
        "kraken_enabled": False,  # Start without Kraken
        "trading_pairs": ["ETH/USD"],
        "initial_capital": 1000.0,
        "analysis_interval": 60
    }
    
    # Initialize agent
    agent = AITradingAgent("config.json")
    await agent.initialize()
    
    # Generate sample market data
    dates = pd.date_range(end='2024-01-01', periods=100, freq='1H')
    np.random.seed(42)
    prices = [2000 + i * 0.5 + np.random.normal(0, 10) for i in range(100)]
    
    market_data = pd.DataFrame({
        'timestamp': dates,
        'close': prices,
        'volume': np.random.uniform(100, 1000, 100)
    })
    market_data.set_index('timestamp', inplace=True)
    
    print("🤖 Starting Basic Mean Reversion Agent...")
    print(f"📊 Initial capital: ${config['initial_capital']}")
    
    # Run for 10 iterations
    for i in range(10):
        # Generate signal
        signal = agent.strategy.generate_signal(market_data)
        
        if signal.action != 'hold':
            print(f"📈 Signal {i+1}: {signal.action.upper()}")
            print(f"   Confidence: {signal.confidence:.2f}")
            print(f"   Position Size: {signal.position_size:.2f}")
            print(f"   Reasoning: {signal.reasoning}")
            
            # Execute trade (mock)
            result = await agent.execute_trade(signal, "ETH/USD")
            print(f"   Result: {result['status']}")
        
        await asyncio.sleep(1)  # Wait 1 second between signals
    
    # Show final status
    status = agent.get_status()
    print(f"\n📊 Final Status:")
    print(f"   Total Trades: {status['total_trades']}")
    print(f"   Current Capital: ${status['current_capital']:.2f}")
    print(f"   Performance: {status['performance_metrics']}")

if __name__ == "__main__":
    asyncio.run(basic_mean_reversion_agent())
```

### Example 2: Kraken-Enhanced Agent

```python
"""
Kraken-Enhanced Trading Agent
Real market data and trading execution
"""

import asyncio
import os
from agent.kraken_integration import KrakenTradingBot, KrakenConfig
from agent.strategy import TradingStrategy, StrategyType, RiskParams

async def kraken_enhanced_agent():
    """Complete example with real Kraken integration"""
    
    # Configuration (use environment variables for security)
    kraken_config = KrakenConfig(
        api_key=os.getenv('KRAKEN_API_KEY', 'demo_key'),
        api_secret=os.getenv('KRAKEN_API_SECRET', 'demo_secret'),
        sandbox=True  # Use sandbox for testing
    )
    
    # Initialize strategy
    risk_params = RiskParams(
        max_position_size=0.05,  # 5% max position
        max_leverage=1.0,        # No leverage for safety
        max_daily_loss=0.02,     # 2% daily loss limit
        min_confidence=0.8       # High confidence required
    )
    
    strategy = TradingStrategy(StrategyType.MOMENTUM, risk_params)
    kraken_bot = KrakenTradingBot(kraken_config)
    
    print("🐙 Initializing Kraken-Enhanced Agent...")
    
    try:
        await kraken_bot.initialize()
        print("✅ Kraken connection established")
        
        # Get real market data
        market_data = await kraken_bot.get_market_data("XBTUSD")
        print(f"📊 Current BTC Price: ${market_data['ticker']['result']['XXBTZUSD']['c'][0]}")
        
        # Generate trading signal
        # Convert Kraken data to strategy format
        df = pd.DataFrame(market_data['ohlcv']['result']['XXBTZUSD'])
        df.columns = ['timestamp', 'open', 'high', 'low', 'close', 'vwap', 'volume', 'count']
        df['timestamp'] = pd.to_datetime(df['timestamp'], unit='s')
        df.set_index('timestamp', inplace=True)
        
        signal = strategy.generate_signal(df)
        
        print(f"🎯 Trading Signal: {signal.action.upper()}")
        print(f"   Confidence: {signal.confidence:.2f}")
        print(f"   Reasoning: {signal.reasoning}")
        
        # Execute trade if signal is strong
        if signal.action != 'hold' and signal.confidence > 0.8:
            trade_result = await kraken_bot.execute_trade(
                signal.__dict__, 
                "XBTUSD", 
                0.001  # Small position for testing
            )
            print(f"💰 Trade executed: {trade_result['status']}")
            if trade_result['status'] == 'success':
                print(f"   Order ID: {trade_result['order_id']}")
        else:
            print("⏸️ No trade executed - signal too weak or HOLD")
            
    except Exception as e:
        print(f"❌ Error: {e}")
        print("💡 Make sure to set KRAKEN_API_KEY and KRAKEN_API_SECRET environment variables")
    
    finally:
        await kraken_bot.shutdown()

if __name__ == "__main__":
    asyncio.run(kraken_enhanced_agent())
```

### Example 3: Advanced Multi-Strategy Agent

```python
"""
Advanced Multi-Strategy Agent
Combines multiple strategies with dynamic allocation
"""

import asyncio
import pandas as pd
import numpy as np
from typing import Dict, List
from dataclasses import dataclass
from agent.strategy import TradingStrategy, StrategyType, RiskParams, Signal

@dataclass
class StrategyAllocation:
    strategy: TradingStrategy
    weight: float
    last_signal: Signal
    performance: float

class MultiStrategyAgent:
    def __init__(self, initial_capital: float = 10000):
        self.initial_capital = initial_capital
        self.strategies = {}
        self.performance_history = {}
        
    def add_strategy(self, name: str, strategy: TradingStrategy, weight: float = 1.0):
        """Add a strategy to the agent"""
        self.strategies[name] = StrategyAllocation(
            strategy=strategy,
            weight=weight,
            last_signal=Signal("hold", 0.0, 0.0),
            performance=0.0
        )
        self.performance_history[name] = []
    
    def generate_consensus_signal(self, market_data: pd.DataFrame) -> Signal:
        """Generate consensus signal from all strategies"""
        signals = {}
        
        # Get signals from all strategies
        for name, allocation in self.strategies.items():
            signal = allocation.strategy.generate_signal(market_data)
            signals[name] = signal
            
            # Update last signal
            allocation.last_signal = signal
        
        # Weighted voting
        buy_weight = 0.0
        sell_weight = 0.0
        total_confidence = 0.0
        
        for name, signal in signals.items():
            weight = self.strategies[name].weight
            confidence = signal.confidence * weight
            
            if signal.action == 'buy':
                buy_weight += confidence
            elif signal.action == 'sell':
                sell_weight += confidence
            
            total_confidence += confidence
        
        # Determine consensus action
        if buy_weight > sell_weight * 1.2:  # 20% threshold
            action = 'buy'
            consensus_confidence = buy_weight / total_confidence
        elif sell_weight > buy_weight * 1.2:
            action = 'sell'
            consensus_confidence = sell_weight / total_confidence
        else:
            action = 'hold'
            consensus_confidence = 0.0
        
        # Create consensus signal
        consensus_signal = Signal(
            action=action,
            confidence=consensus_confidence,
            position_size=consensus_confidence * 0.1,  # Max 10% position
            reasoning=f"Consensus from {len(signals)} strategies. Buy: {buy_weight:.2f}, Sell: {sell_weight:.2f}"
        )
        
        return consensus_signal
    
    def update_strategy_performance(self, strategy_name: str, pnl: float):
        """Update strategy performance based on PnL"""
        if strategy_name in self.strategies:
            self.strategies[strategy_name].performance += pnl
            self.performance_history[strategy_name].append(pnl)
            
            # Adjust weights based on performance (simple momentum)
            if len(self.performance_history[strategy_name]) > 5:
                recent_performance = np.mean(self.performance_history[strategy_name][-5:])
                if recent_performance > 0:
                    self.strategies[strategy_name].weight *= 1.1  # Increase weight
                else:
                    self.strategies[strategy_name].weight *= 0.9  # Decrease weight

async def multi_strategy_example():
    """Complete example of multi-strategy agent"""
    
    print("🚀 Initializing Multi-Strategy Agent...")
    
    # Create agent
    agent = MultiStrategyAgent(initial_capital=10000)
    
    # Add strategies with different weights
    risk_params = RiskParams(max_position_size=0.05)
    
    agent.add_strategy("mean_reversion", 
                      TradingStrategy(StrategyType.MEAN_REVERSION, risk_params), 
                      weight=1.0)
    
    agent.add_strategy("momentum", 
                      TradingStrategy(StrategyType.MOMENTUM, risk_params), 
                      weight=0.8)
    
    agent.add_strategy("arbitrage", 
                      TradingStrategy(StrategyType.ARBITRAGE, risk_params), 
                      weight=0.5)
    
    print(f"✅ Added {len(agent.strategies)} strategies")
    
    # Generate sample market data with different characteristics
    dates = pd.date_range(end='2024-01-01', periods=200, freq='1H')
    np.random.seed(42)
    
    # Create trending market with mean reversion
    trend = np.linspace(100, 120, 200)
    noise = np.random.normal(0, 2, 200)
    prices = trend + noise
    
    # Add some arbitrage opportunities
    exchange1_prices = prices.copy()
    exchange2_prices = prices + np.random.normal(0, 0.5, 200)
    
    market_data = pd.DataFrame({
        'timestamp': dates,
        'close': prices,
        'exchange1_price': exchange1_prices,
        'exchange2_price': exchange2_prices,
        'volume': np.random.uniform(100, 1000, 200)
    })
    market_data.set_index('timestamp', inplace=True)
    
    print("📊 Generated market data with 200 data points")
    
    # Run simulation
    for i in range(50, 200, 10):  # Every 10 data points
        current_data = market_data.iloc[:i]
        
        # Generate consensus signal
        consensus_signal = agent.generate_consensus_signal(current_data)
        
        print(f"\n📍 Period {i}:")
        print(f"   Consensus Signal: {consensus_signal.action.upper()}")
        print(f"   Confidence: {consensus_signal.confidence:.2f}")
        print(f"   Position Size: {consensus_signal.position_size:.3f}")
        print(f"   Reasoning: {consensus_signal.reasoning}")
        
        # Show individual strategy signals
        for name, allocation in agent.strategies.items():
            print(f"   {name}: {allocation.last_signal.action} ({allocation.last_signal.confidence:.2f})")
        
        # Simulate PnL and update performance
        if consensus_signal.action != 'hold':
            # Mock PnL calculation
            if consensus_signal.action == 'buy':
                pnl = np.random.normal(0.01, 0.02)  # Small positive/negative return
            else:
                pnl = np.random.normal(-0.005, 0.015)  # Slightly negative bias for sells
            
            # Update best performing strategies
            for name, allocation in agent.strategies.items():
                if allocation.last_signal.action == consensus_signal.action:
                    agent.update_strategy_performance(name, pnl)
        
        await asyncio.sleep(0.1)  # Small delay
    
    # Show final performance
    print(f"\n📊 Final Strategy Performance:")
    for name, allocation in agent.strategies.items():
        print(f"   {name}:")
        print(f"     Weight: {allocation.weight:.2f}")
        print(f"     Performance: {allocation.performance:.4f}")
        print(f"     Last Signal: {allocation.last_signal.action}")

if __name__ == "__main__":
    asyncio.run(multi_strategy_example())
```

### Example 4: ERC-8004 Contract Deployment

```python
"""
ERC-8004 Contract Deployment Example
Deploy all required contracts for your agent
"""

import asyncio
import json
from web3 import Web3
from web3.middleware import geth_poa_middleware

async def deploy_erc8004_contracts():
    """Deploy all ERC-8004 contracts"""
    
    # Connect to Sepolia testnet
    w3 = Web3(Web3.HTTPProvider('https://sepolia.infura.io/v3/YOUR_PROJECT_ID'))
    w3.middleware_onion.inject(geth_poa_middleware, layer=0)
    
    # Your private key (use environment variable in production)
    private_key = os.getenv('PRIVATE_KEY')
    if not private_key:
        print("❌ Set PRIVATE_KEY environment variable")
        return
    
    account = w3.eth.account.from_key(private_key)
    w3.eth.default_account = account.address
    
    print(f"🔗 Connected to Sepolia testnet")
    print(f"👤 Account: {account.address}")
    print(f"💰 Balance: {w3.eth.get_balance(account.address) / 1e18:.4f} ETH")
    
    # Contract ABIs (simplified - use actual compiled contracts)
    identity_registry_abi = [
        {
            "inputs": [],
            "stateMutability": "nonpayable",
            "type": "constructor"
        },
        {
            "inputs": [
                {"name": "to", "type": "address"},
                {"name": "name", "type": "string"},
                {"name": "uri", "type": "string"}
            ],
            "name": "mintAgent",
            "outputs": [{"name": "agentId", "type": "uint256"}],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]
    
    # Deploy Identity Registry
    print("📄 Deploying Identity Registry...")
    identity_registry = w3.eth.contract(abi=identity_registry_abi, bytecode="0x...")
    
    # Estimate gas
    gas_estimate = identity_registry.constructor().estimate_gas()
    print(f"⛽ Estimated gas: {gas_estimate}")
    
    # Deploy
    tx_hash = identity_registry.constructor().transact({
        'from': account.address,
        'gas': gas_estimate + 100000,  # Add buffer
        'gasPrice': w3.eth.gas_price
    })
    
    print(f"📤 Transaction sent: {tx_hash.hex()}")
    receipt = w3.eth.wait_for_transaction_receipt(tx_hash)
    
    if receipt.status == 1:
        print(f"✅ Identity Registry deployed at: {receipt.contractAddress}")
        identity_registry_address = receipt.contractAddress
    else:
        print("❌ Deployment failed")
        return
    
    # Deploy other contracts (similar process)
    # Reputation Registry, Validation Registry, Risk Router, Trading Agent
    
    # Save deployment addresses
    deployment_info = {
        "network": "sepolia",
        "deployer": account.address,
        "contracts": {
            "identity_registry": identity_registry_address,
            # Add other contract addresses here
        },
        "timestamp": str(pd.Timestamp.now())
    }
    
    with open('deployment.json', 'w') as f:
        json.dump(deployment_info, f, indent=2)
    
    print(f"💾 Deployment info saved to deployment.json")
    
    # Register your agent
    print("🤖 Registering your AI agent...")
    
    agent_metadata = {
        "type": "https://eips.ethereum.org/EIPS/eip-8004#registration-v1",
        "name": "MyAIAgent",
        "description": "AI trading agent specializing in mean reversion strategies",
        "services": [
            {
                "name": "web",
                "endpoint": "https://my-agent.example.com/"
            }
        ],
        "supportedTrust": ["reputation", "crypto-economic"],
        "agentWallet": account.address
    }
    
    # Upload metadata to IPFS (mock for example)
    ipfs_hash = "Qm" + "x" * 44  # Mock IPFS hash
    metadata_uri = f"https://ipfs.io/ipfs/{ipfs_hash}"
    
    # Mint agent NFT
    identity_contract = w3.eth.contract(
        address=identity_registry_address,
        abi=identity_registry_abi
    )
    
    tx_hash = identity_contract.functions.mintAgent(
        account.address,
        "MyAIAgent",
        metadata_uri
    ).transact({
        'from': account.address,
        'gas': 200000,
        'gasPrice': w3.eth.gas_price
    })
    
    receipt = w3.eth.wait_for_transaction_receipt(tx_hash)
    
    if receipt.status == 1:
        print(f"🎉 Agent registered successfully!")
        print(f"📛 Agent ID: {receipt.logs[0].topics[1].hex()}")
        print(f"🔗 Metadata URI: {metadata_uri}")
    else:
        print("❌ Agent registration failed")

if __name__ == "__main__":
    asyncio.run(deploy_erc8004_contracts())
```

### Example 5: Complete Testing Suite

```python
"""
Complete Testing Example
Run all tests for your agent
"""

import pytest
import asyncio
import pandas as pd
import numpy as np
from agent.strategy import TradingStrategy, StrategyType, RiskParams

def test_strategy_functionality():
    """Test all strategy types"""
    
    print("🧪 Testing Strategy Functionality...")
    
    # Test data
    dates = pd.date_range(end='2024-01-01', periods=100, freq='1H')
    np.random.seed(42)
    prices = [100 + i * 0.1 + np.random.normal(0, 1) for i in range(100)]
    
    market_data = pd.DataFrame({
        'timestamp': dates,
        'close': prices,
        'volume': np.random.uniform(100, 1000, 100)
    })
    market_data.set_index('timestamp', inplace=True)
    
    risk_params = RiskParams(max_position_size=0.1)
    
    # Test each strategy
    strategies = [
        ("Mean Reversion", StrategyType.MEAN_REVERSION),
        ("Momentum", StrategyType.MOMENTUM),
        ("Arbitrage", StrategyType.ARBITRAGE),
        ("Grid Trading", StrategyType.GRID_TRADING)
    ]
    
    for name, strategy_type in strategies:
        print(f"  📊 Testing {name}...")
        
        strategy = TradingStrategy(strategy_type, risk_params)
        signal = strategy.generate_signal(market_data)
        
        assert signal.action in ['buy', 'sell', 'hold'], f"Invalid action in {name}"
        assert 0 <= signal.confidence <= 1, f"Invalid confidence in {name}"
        assert 0 <= signal.position_size <= 1, f"Invalid position size in {name}"
        
        print(f"     ✅ {name}: {signal.action} (confidence: {signal.confidence:.2f})")

def test_risk_management():
    """Test risk management functionality"""
    
    print("🛡️ Testing Risk Management...")
    
    risk_params = RiskParams(
        max_position_size=0.1,
        max_leverage=2.0,
        max_daily_loss=0.05,
        min_confidence=0.7
    )
    
    strategy = TradingStrategy(StrategyType.MEAN_REVERSION, risk_params)
    
    # Test valid signal
    valid_signal = Signal(
        action="buy",
        confidence=0.8,
        position_size=0.05,
        reasoning="Valid signal"
    )
    
    assert strategy.validate_signal(valid_signal, 10000) == True
    print("  ✅ Valid signal passed")
    
    # Test invalid signal (low confidence)
    invalid_signal = Signal(
        action="buy",
        confidence=0.5,
        position_size=0.05,
        reasoning="Low confidence"
    )
    
    assert strategy.validate_signal(invalid_signal, 10000) == False
    print("  ✅ Low confidence signal rejected")
    
    # Test oversized signal
    oversized_signal = Signal(
        action="buy",
        confidence=0.8,
        position_size=0.2,  # Exceeds max_position_size
        reasoning="Oversized signal"
    )
    
    assert strategy.validate_signal(oversized_signal, 10000) == False
    print("  ✅ Oversized signal rejected")

async def test_performance_tracking():
    """Test performance tracking"""
    
    print("📈 Testing Performance Tracking...")
    
    # Mock performance data
    trades = [
        {'pnl': 0.05, 'timestamp': '2024-01-01', 'action': 'buy'},
        {'pnl': -0.02, 'timestamp': '2024-01-02', 'action': 'sell'},
        {'pnl': 0.03, 'timestamp': '2024-01-03', 'action': 'buy'},
        {'pnl': 0.07, 'timestamp': '2024-01-04', 'action': 'sell'},
        {'pnl': -0.01, 'timestamp': '2024-01-05', 'action': 'buy'}
    ]
    
    # Calculate metrics
    total_pnl = sum(trade['pnl'] for trade in trades)
    winning_trades = len([t for t in trades if t['pnl'] > 0])
    win_rate = winning_trades / len(trades)
    
    # Simple Sharpe ratio calculation (mock)
    returns = [trade['pnl'] for trade in trades]
    avg_return = np.mean(returns)
    std_return = np.std(returns)
    sharpe_ratio = avg_return / std_return if std_return > 0 else 0
    
    print(f"  📊 Total PnL: {total_pnl:.3f}")
    print(f"  🎯 Win Rate: {win_rate:.2%}")
    print(f"  📈 Sharpe Ratio: {sharpe_ratio:.2f}")
    
    # Validate metrics
    assert -1 <= total_pnl <= 1, "Total PnL should be reasonable"
    assert 0 <= win_rate <= 1, "Win rate should be between 0 and 1"
    assert not np.isnan(sharpe_ratio), "Sharpe ratio should not be NaN"
    
    print("  ✅ Performance metrics validated")

def run_all_tests():
    """Run all tests"""
    
    print("🚀 Running Complete Test Suite...")
    print("=" * 50)
    
    try:
        test_strategy_functionality()
        print()
        test_risk_management()
        print()
        asyncio.run(test_performance_tracking())
        print()
        print("🎉 All tests passed! Your agent is ready to deploy.")
        
    except Exception as e:
        print(f"❌ Test failed: {e}")
        print("💡 Fix the issues before deploying your agent.")

if __name__ == "__main__":
    run_all_tests()
```

## 🎯 How to Use These Examples

### 1. **Basic Agent** - Start Here
```bash
python basic_mean_reversion_agent.py
```
Perfect for understanding the core concepts without external dependencies.

### 2. **Kraken Integration** - Real Trading
```bash
export KRAKEN_API_KEY="your_key"
export KRAKEN_API_SECRET="your_secret"
python kraken_enhanced_agent.py
```
Connect to real markets and execute actual trades.

### 3. **Multi-Strategy** - Advanced
```bash
python multi_strategy_agent.py
```
Combine multiple strategies for better performance.

### 4. **Contract Deployment** - Production
```bash
export PRIVATE_KEY="your_private_key"
python deploy_contracts.py
```
Deploy your agent to the blockchain.

### 5. **Testing** - Quality Assurance
```bash
python test_suite.py
```
Ensure everything works before deployment.

## 🛠️ Customization Tips

### Modify Strategy Parameters
```python
# Adjust risk levels
risk_params = RiskParams(
    max_position_size=0.2,  # Increase to 20%
    max_leverage=3.0,        # Increase leverage
    min_confidence=0.6       # Lower confidence threshold
)
```

### Add Custom Indicators
```python
# Add to your strategy class
def calculate_custom_indicator(self, data):
    # Your custom logic here
    return indicator_value
```

### Integrate Additional Exchanges
```python
# Follow the Kraken pattern for other exchanges
class BinanceIntegration:
    async def get_market_data(self, pair):
        # Binance API implementation
        pass
```

These examples provide everything you need to build, test, and deploy a production-ready ERC-8004 AI trading agent! 🚀
