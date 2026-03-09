#!/usr/bin/env python3
"""
Kraken CLI MCP Integration Examples
Demonstrates various use cases of the Kraken CLI MCP server with ERC-8004 AI agents
"""

import asyncio
import json
import logging
from agent.kraken_integration import UnifiedKrakenIntegration
from agent.mcp_client import MCPConfig, KrakenMCPBot

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

async def example_basic_market_data():
    """Example 1: Basic market data retrieval"""
    print("\n=== Example 1: Basic Market Data ===")
    
    integration = UnifiedKrakenIntegration(use_mcp=True)
    try:
        await integration.initialize()
        
        # Get market data for Bitcoin
        market_data = await integration.get_market_data("BTCUSD")
        
        print("BTC/USD Market Data:")
        print(f"Ticker: {json.dumps(market_data['ticker'], indent=2)}")
        print(f"Order Book Depth: {len(market_data['orderbook'].get('result', {}))}")
        print(f"OHLCV Data Points: {len(market_data['ohlcv'].get('result', {}))}")
        
    finally:
        await integration.shutdown()

async def example_portfolio_monitoring():
    """Example 2: Portfolio monitoring and balance checking"""
    print("\n=== Example 2: Portfolio Monitoring ===")
    
    integration = UnifiedKrakenIntegration(use_mcp=True)
    try:
        await integration.initialize()
        
        # Get account status
        account = await integration.get_account_status()
        
        print("Account Status:")
        print(f"Balance: {json.dumps(account['balance'], indent=2)}")
        print(f"Open Orders: {len(account['open_orders'].get('result', {}))}")
        print(f"Tracked Positions: {len(account['position_tracker'])}")
        
    finally:
        await integration.shutdown()

async def example_paper_trading():
    """Example 3: Paper trading simulation"""
    print("\n=== Example 3: Paper Trading Simulation ===")
    
    integration = UnifiedKrakenIntegration(use_mcp=True)
    try:
        await integration.initialize()
        
        # Reset paper trading state
        print("Resetting paper trading state...")
        # Note: This would be available via MCP if implemented
        
        # Simulate trading signals
        signals = [
            {
                'action': 'buy',
                'confidence': 0.85,
                'position_size': 0.1,
                'reasoning': 'RSI oversold condition'
            },
            {
                'action': 'sell',
                'confidence': 0.78,
                'position_size': 0.05,
                'reasoning': 'Take profit at resistance'
            }
        ]
        
        for i, signal in enumerate(signals, 1):
            print(f"\nExecuting Trade {i}: {signal['action']} {signal['position_size']} BTC")
            
            result = await integration.execute_trade(signal, "BTCUSD", 0.1)
            print(f"Result: {result['status']}")
            
            if result['status'] == 'success':
                print(f"Order ID: {result['order_id']}")
        
        # Check final account status
        final_status = await integration.get_account_status()
        print(f"\nFinal Open Orders: {len(final_status['open_orders'].get('result', {}))}")
        
    finally:
        await integration.shutdown()

async def example_multi_asset_monitoring():
    """Example 4: Multi-asset monitoring"""
    print("\n=== Example 4: Multi-Asset Monitoring ===")
    
    pairs = ["BTCUSD", "ETHUSD", "SOLUSD"]
    integration = UnifiedKrakenIntegration(use_mcp=True)
    
    try:
        await integration.initialize()
        
        # Monitor multiple assets
        market_snapshots = {}
        
        for pair in pairs:
            market_data = await integration.get_market_data(pair)
            ticker_data = market_data['ticker'].get('result', {})
            
            # Extract key metrics
            if ticker_data and pair in ticker_data:
                pair_data = ticker_data[pair]
                market_snapshots[pair] = {
                    'price': pair_data.get('c', [0])[0],  # Last trade price
                    'volume': pair_data.get('v', [0])[0],  # 24h volume
                    'change': pair_data.get('p', [0])[0],  # 24h price change
                    'high': pair_data.get('h', [0])[0],    # 24h high
                    'low': pair_data.get('l', [0])[0]     # 24h low
                }
        
        print("Multi-Asset Market Snapshot:")
        for pair, data in market_snapshots.items():
            print(f"\n{pair}:")
            print(f"  Price: ${data['price']:,.2f}")
            print(f"  24h Change: {data['change']:.2f}%")
            print(f"  Volume: ${data['volume']:,.0f}")
            print(f"  Range: ${data['low']:,.2f} - ${data['high']:,.2f}")
        
    finally:
        await integration.shutdown()

async def example_risk_management():
    """Example 5: Risk management with position sizing"""
    print("\n=== Example 5: Risk Management ===")
    
    integration = UnifiedKrakenIntegration(use_mcp=True)
    try:
        await integration.initialize()
        
        # Define risk parameters
        risk_params = {
            'max_position_size': 0.05,  # 5% max position
            'max_daily_loss': 100,      # $100 max daily loss
            'confidence_threshold': 0.75  # Minimum confidence
        }
        
        # Test signals with different risk levels
        test_signals = [
            {
                'action': 'buy',
                'confidence': 0.9,  # High confidence
                'position_size': 0.03,
                'reasoning': 'Strong momentum + volume spike'
            },
            {
                'action': 'buy',
                'confidence': 0.6,  # Low confidence - should be rejected
                'position_size': 0.02,
                'reasoning': 'Weak signal'
            },
            {
                'action': 'sell',
                'confidence': 0.8,  # Good confidence
                'position_size': 0.1,  # Too large - should be sized down
                'reasoning': 'Overbought conditions'
            }
        ]
        
        for i, signal in enumerate(test_signals, 1):
            print(f"\nRisk Assessment {i}:")
            print(f"Signal: {signal['action']} @ {signal['confidence']:.1%} confidence")
            
            # Apply risk filters
            if signal['confidence'] < risk_params['confidence_threshold']:
                print("❌ REJECTED: Confidence below threshold")
                continue
            
            # Adjust position size
            adjusted_size = min(signal['position_size'], risk_params['max_position_size'])
            if adjusted_size != signal['position_size']:
                print(f"⚠️  POSITION SIZED DOWN: {signal['position_size']:.2%} → {adjusted_size:.2%}")
            
            # Execute trade
            result = await integration.execute_trade(signal, "BTCUSD", adjusted_size)
            print(f"Execution: {result['status']}")
        
    finally:
        await integration.shutdown()

async def example_mcp_tools_discovery():
    """Example 6: Discover available MCP tools"""
    print("\n=== Example 6: MCP Tools Discovery ===")
    
    from agent.mcp_client import MCPClient, MCPConfig
    
    config = MCPConfig(services=["market", "account", "paper"])
    client = MCPClient(config)
    
    try:
        await client.start_server()
        
        # List all available tools
        tools = await client.list_tools()
        
        print(f"Available Kraken MCP Tools: {len(tools)}")
        print("\nTool Categories:")
        
        categories = {}
        for tool in tools:
            name = tool.get('name', '')
            if 'ticker' in name:
                categories.setdefault('Market Data', []).append(name)
            elif 'balance' in name or 'account' in name:
                categories.setdefault('Account', []).append(name)
            elif 'order' in name:
                categories.setdefault('Trading', []).append(name)
            elif 'paper' in name:
                categories.setdefault('Paper Trading', []).append(name)
            else:
                categories.setdefault('Other', []).append(name)
        
        for category, tool_names in categories.items():
            print(f"\n{category}:")
            for tool_name in tool_names[:5]:  # Show first 5 tools per category
                print(f"  - {tool_name}")
            if len(tool_names) > 5:
                print(f"  ... and {len(tool_names) - 5} more")
        
    finally:
        await client.close()

async def example_erc8004_integration():
    """Example 7: ERC-8004 validation + Kraken MCP integration"""
    print("\n=== Example 7: ERC-8004 + Kraken MCP Integration ===")
    
    # Simulate ERC-8004 agent validation
    class MockERC8004Validator:
        def validate_agent_identity(self, agent_id: str) -> bool:
            # Mock validation - in real implementation, check blockchain
            return agent_id == "hackathon_agent_001"
        
        def check_reputation(self, agent_id: str) -> int:
            # Mock reputation score (0-100)
            return 85 if agent_id == "hackathon_agent_001" else 50
        
        def validate_strategy(self, strategy: dict) -> bool:
            # Mock strategy validation
            required_fields = ['action', 'confidence', 'position_size', 'reasoning']
            return all(field in strategy for field in required_fields)
    
    validator = MockERC8004Validator()
    agent_id = "hackathon_agent_001"
    
    # Check agent credentials
    if not validator.validate_agent_identity(agent_id):
        print("❌ Agent identity validation failed")
        return
    
    reputation = validator.check_reputation(agent_id)
    print(f"✅ Agent Identity Valid - Reputation: {reputation}/100")
    
    # Initialize Kraken integration
    integration = UnifiedKrakenIntegration(use_mcp=True)
    try:
        await integration.initialize()
        
        # Create validated trading signal
        signal = {
            'action': 'buy',
            'confidence': 0.8,
            'position_size': 0.02,
            'reasoning': 'ETH breakout above resistance with volume confirmation',
            'agent_id': agent_id,
            'reputation_score': reputation
        }
        
        # Validate strategy through ERC-8004
        if not validator.validate_strategy(signal):
            print("❌ Strategy validation failed")
            return
        
        print("✅ Strategy Validated")
        
        # Execute trade with reputation-based position sizing
        max_position = 0.01 * (reputation / 100)  # Scale by reputation
        result = await integration.execute_trade(signal, "ETHUSD", max_position)
        
        print(f"Trade Execution: {result['status']}")
        if result['status'] == 'success':
            print(f"✅ Trade executed with ERC-8004 validation")
            print(f"Order ID: {result['order_id']}")
        
    finally:
        await integration.shutdown()

async def main():
    """Run all examples"""
    print("🚀 Kraken CLI MCP Integration Examples")
    print("=" * 50)
    
    examples = [
        example_basic_market_data,
        example_portfolio_monitoring,
        example_paper_trading,
        example_multi_asset_monitoring,
        example_risk_management,
        example_mcp_tools_discovery,
        example_erc8004_integration
    ]
    
    for example in examples:
        try:
            await example()
        except Exception as e:
            logger.error(f"Example failed: {e}")
        
        print("\n" + "-" * 50)
    
    print("\n✅ All examples completed!")

if __name__ == "__main__":
    asyncio.run(main())
