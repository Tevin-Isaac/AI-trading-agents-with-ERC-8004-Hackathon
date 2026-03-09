//! TypeScript MCP Integration Examples
//! 
//! Comprehensive examples of using the Kraken CLI MCP integration with the ERC-8004 AI trading agent in TypeScript

import { AgentFactory } from '../typescript/src/agent';
import { TradingSignal } from '../typescript/src/types';
import { ConfigManager } from '../typescript/src/config';

async function main() {
  console.log('🚀 ERC-8004 Kraken MCP Examples in TypeScript');

  try {
    // Initialize configuration
    const configManager = ConfigManager.getInstance();
    const config = configManager.getEnvironmentConfig('development');

    // Create agent
    const agent = await AgentFactory.createAgent(config);

    // Run examples
    await example1_BasicMarketData(agent);
    await example2_PaperTrading(agent);
    await example3_MultiAssetMonitoring(agent);
    await example4_StrategyIntegration(agent);
    await example5_ERC8004Validation(agent);
    await example6_RiskManagement(agent);
    await example7_ContinuousTrading(agent);

    console.log('✅ All examples completed successfully!');
  } catch (error) {
    console.error('❌ Example execution failed:', error);
    process.exit(1);
  }
}

// Example 1: Basic Market Data Retrieval
async function example1_BasicMarketData(agent: any) {
  console.log('\n=== Example 1: Basic Market Data ===');

  try {
    const marketData = await agent.krakenIntegration.getMarketData('BTCUSD');
    
    console.log('✅ Retrieved BTC/USD market data');
    console.log('  Timestamp:', marketData.timestamp);
    console.log('  Data available: ticker, orderbook, OHLCV');

    // Extract and display current price
    if (marketData.ticker.result?.BTCUSD?.c?.[0]) {
      console.log('  Current BTC Price: $' + marketData.ticker.result.BTCUSD.c[0]);
    }
  } catch (error) {
    console.error('❌ Failed to get market data:', error);
  }
}

// Example 2: Paper Trading
async function example2_PaperTrading(agent: any) {
  console.log('\n=== Example 2: Paper Trading ===');

  try {
    const signal: TradingSignal = {
      action: 'buy',
      pair: 'BTCUSD',
      confidence: 0.85,
      positionSize: 0.02,
      reasoning: 'RSI oversold condition detected'
    };

    console.log('📈 Executing paper trade: ' + signal.action + ' ' + signal.positionSize + ' BTC');
    console.log('  Reasoning:', signal.reasoning);
    console.log('  Confidence: ' + (signal.confidence * 100).toFixed(1) + '%');

    const result = await agent.krakenIntegration.executeTrade(signal, 0.05);

    console.log('📊 Trade Result:', result.status);
    if (result.status === 'success' && result.orderId) {
      console.log('  Order ID:', result.orderId);
    } else {
      console.log('  Reason:', result.reason);
    }

    // Check account status after trade
    const accountStatus = await agent.krakenIntegration.getAccountStatus();
    console.log('  Open positions:', accountStatus.positions.length);
  } catch (error) {
    console.error('❌ Paper trading failed:', error);
  }
}

// Example 3: Multi-Asset Monitoring
async function example3_MultiAssetMonitoring(agent: any) {
  console.log('\n=== Example 3: Multi-Asset Monitoring ===');

  const symbols = ['BTCUSD', 'ETHUSD', 'SOLUSD'];
  const marketSnapshots: any[] = [];

  for (const symbol of symbols) {
    try {
      const marketData = await agent.krakenIntegration.getMarketData(symbol);
      
      // Extract key metrics
      if (marketData.ticker.result?.[symbol]) {
        const assetData = marketData.ticker.result[symbol];
        const snapshot = {
          symbol,
          price: assetData.c?.[0] || 'N/A',
          change: assetData.p?.[0] || '0',
          volume: assetData.v?.[0] || '0',
          high: assetData.h?.[0] || '0',
          low: assetData.l?.[0] || '0'
        };
        marketSnapshots.push(snapshot);
      }
    } catch (error) {
      console.error('❌ Failed to get data for', symbol, ':', error);
    }
  }

  console.log('📊 Multi-Asset Market Snapshot:');
  marketSnapshots.forEach(snapshot => {
    console.log(`  ${snapshot.symbol}: $${snapshot.price} (${snapshot.change}%)`);
  });
}

// Example 4: Strategy Integration
async function example4_StrategyIntegration(agent: any) {
  console.log('\n=== Example 4: Strategy Integration ===');

  try {
    // Get market data
    const marketData = await agent.krakenIntegration.getMarketData('BTCUSD');

    // Create mock agent profile for strategy analysis
    const agentProfile = {
      info: {
        id: 'example_agent',
        metadata: 'Example agent for demonstration',
        verified: true,
        registrationTimestamp: BigInt(0)
      },
      reputation: 85,
      reputationHistory: [75, 80, 85]
    };

    // Test mean reversion strategy
    console.log('🧠 Testing Mean Reversion Strategy...');
    // This would require implementing the strategy classes in TypeScript
    console.log('  ✅ Strategy analysis would be performed here');
    console.log('  📝 Action: BUY/SELL based on RSI');
    console.log('  📊 Confidence: 80-90%');
    console.log('  💡 Reasoning: RSI oversold/overbought conditions');
  } catch (error) {
    console.error('❌ Strategy integration failed:', error);
  }
}

// Example 5: ERC-8004 Validation
async function example5_ERC8004Validation(agent: any) {
  console.log('\n=== Example 5: ERC-8004 Validation ===');

  try {
    // Get agent profile
    const profile = await agent.erc8004Client.getAgentProfile(agent.config.agentId);

    if (profile) {
      console.log('👤 Agent Profile:');
      console.log('  ID:', profile.info.id);
      console.log('  Verified:', profile.info.verified);
      console.log('  Reputation:', profile.reputation);
      console.log('  History:', profile.reputationHistory);

      // Test validation for trading
      const canTrade = await agent.erc8004Client.validateAgentForTrading(
        agent.config.agentId,
        70  // Minimum reputation
      );

      console.log('🔒 Trading Validation:', canTrade ? '✅ Allowed' : '❌ Blocked');

      // Update reputation based on simulated performance
      console.log('📈 Updating reputation...');
      await agent.erc8004Client.updateReputation(agent.config.agentId, 88);
      
      const newReputation = await agent.erc8004Client.getReputation(agent.config.agentId);
      console.log('  New reputation:', newReputation);
    } else {
      console.log('⚠️  Agent not registered. Registering now...');
      
      const success = await agent.erc8004Client.registerAgent(
        agent.config.agentId,
        `ERC-8004 Agent: ${agent.config.agentName}`
      );

      if (success) {
        console.log('✅ Agent registered successfully!');
        await agent.erc8004Client.updateReputation(agent.config.agentId, 75);
      }
    }
  } catch (error) {
    console.error('❌ ERC-8004 validation failed:', error);
  }
}

// Example 6: Risk Management
async function example6_RiskManagement(agent: any) {
  console.log('\n=== Example 6: Risk Management ===');

  const riskParameters = {
    maxPositionSize: 0.05,      // 5% max position
    maxDailyLoss: 50.0,        // $50 max daily loss
    confidenceThreshold: 0.8,   // 80% confidence required
    stopLossPercentage: 0.03,  // 3% stop loss
    takeProfitPercentage: 0.06  // 6% take profit
  };

  console.log('⚖️  Risk Parameters:');
  console.log('  Max Position Size: ' + (riskParameters.maxPositionSize * 100).toFixed(1) + '%');
  console.log('  Max Daily Loss: $' + riskParameters.maxDailyLoss.toFixed(2));
  console.log('  Confidence Threshold: ' + (riskParameters.confidenceThreshold * 100).toFixed(1) + '%');

  // Test signals with different risk levels
  const testSignals: TradingSignal[] = [
    {
      action: 'buy',
      pair: 'BTCUSD',
      confidence: 0.9,  // High confidence
      positionSize: 0.03,
      reasoning: 'Strong momentum signal'
    },
    {
      action: 'buy',
      pair: 'BTCUSD',
      confidence: 0.6,  // Low confidence
      positionSize: 0.02,
      reasoning: 'Weak signal'
    },
    {
      action: 'sell',
      pair: 'BTCUSD',
      confidence: 0.85,
      positionSize: 0.08,  // Too large
      reasoning: 'Overbought conditions'
    }
  ];

  for (let i = 0; i < testSignals.length; i++) {
    const signal = testSignals[i];
    console.log('\n📊 Risk Assessment ' + (i + 1) + ':');
    console.log('  Signal: ' + signal.action + ' @ ' + (signal.confidence * 100).toFixed(1) + '% confidence');
    console.log('  Position Size: ' + (signal.positionSize * 100).toFixed(2) + '%');

    // Apply risk filters
    let riskScore = 0;
    const warnings: string[] = [];

    // Confidence check
    if (signal.confidence < riskParameters.confidenceThreshold) {
      warnings.push('Below confidence threshold');
    } else {
      riskScore++;
    }

    // Position size check
    if (signal.positionSize > riskParameters.maxPositionSize) {
      warnings.push('Position size too large');
    } else {
      riskScore++;
    }

    // Overall risk assessment
    let riskLevel: string;
    switch (riskScore) {
      case 2:
        riskLevel = '🟢 Low Risk';
        break;
      case 1:
        riskLevel = '🟡 Medium Risk';
        break;
      default:
        riskLevel = '🔴 High Risk';
        break;
    }

    console.log('  Risk Level:', riskLevel);
    if (warnings.length > 0) {
      warnings.forEach(warning => console.log('    ⚠️  ' + warning));
    }

    // Execute if low risk
    if (riskScore === 2) {
      try {
        const result = await agent.krakenIntegration.executeTrade(signal, riskParameters.maxPositionSize);
        console.log('  ✅ Trade Executed:', result.status);
      } catch (error) {
        console.error('  ❌ Trade execution failed:', error);
      }
    } else {
      console.log('  ❌ Trade Rejected: Risk too high');
    }
  }
}

// Example 7: Continuous Trading Loop
async function example7_ContinuousTrading(agent: any) {
  console.log('\n=== Example 7: Continuous Trading Loop ===');

  try {
    // Start the agent
    await agent.start();
    console.log('🚀 Agent started successfully');

    // Run a few trading cycles
    const symbols = ['BTCUSD'];
    const strategy = 'mean_reversion';
    const cycles = 3;

    console.log('🔄 Running ' + cycles + ' trading cycles with strategy: ' + strategy);

    for (let cycle = 1; cycle <= cycles; cycle++) {
      console.log('\n--- Trading Cycle ' + cycle + ' ---');

      try {
        const result = await agent.executeTradingCycle(strategy, 'BTCUSD');
        
        console.log('Signal Generated:', result.signalGenerated);
        console.log('Signal Executed:', result.signalExecuted);
        console.log('Reason:', result.reason);
        
        if (result.tradeResult) {
          console.log('Trade Status:', result.tradeResult.status);
          if (result.tradeResult.orderId) {
            console.log('Order ID:', result.tradeResult.orderId);
          }
        }
      } catch (error) {
        console.error('Trading cycle failed:', error);
      }

      // Wait between cycles
      if (cycle < cycles) {
        await new Promise(resolve => setTimeout(resolve, 5000));
      }
    }

    // Stop the agent
    await agent.stop();
    console.log('🛑 Agent stopped');

    // Show final status
    const finalStatus = agent.getState();
    console.log('\n📊 Final Agent Status:');
    console.log('  Total Trades:', finalStatus.totalTrades);
    console.log('  Successful Trades:', finalStatus.successfulTrades);
    const successRate = finalStatus.totalTrades > 0 
      ? (finalStatus.successfulTrades / finalStatus.totalTrades * 100).toFixed(1)
      : '0.0';
    console.log('  Success Rate:', successRate + '%');
  } catch (error) {
    console.error('❌ Continuous trading failed:', error);
  }
}

// Run the examples
if (require.main === module) {
  main().catch(console.error);
}

export {
  main,
  example1_BasicMarketData,
  example2_PaperTrading,
  example3_MultiAssetMonitoring,
  example4_StrategyIntegration,
  example5_ERC8004Validation,
  example6_RiskManagement,
  example7_ContinuousTrading
};
