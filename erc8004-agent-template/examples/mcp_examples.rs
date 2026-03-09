//! Rust MCP Integration Examples
//! 
//! This file contains comprehensive examples of using the Kraken CLI MCP integration
//! with the ERC-8004 AI trading agent in Rust.

use anyhow::Result;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

use erc8004_kraken::{
    AgentConfig, AgentFactory, TradingSignal, UnifiedKrakenIntegration,
    KrakenConfig, ERC8004Config
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 ERC-8004 Kraken MCP Examples in Rust");

    // Run all examples
    example_1_basic_market_data().await?;
    example_2_paper_trading().await?;
    example_3_multi_asset_monitoring().await?;
    example_4_strategy_integration().await?;
    example_5_erc8004_validation().await?;
    example_6_risk_management().await?;
    example_7_continuous_trading().await?;

    info!("✅ All examples completed successfully!");
    Ok(())
}

/// Example 1: Basic Market Data Retrieval
async fn example_1_basic_market_data() -> Result<()> {
    info!("\n=== Example 1: Basic Market Data ===");

    let config = AgentConfig::for_environment("development")?;
    let agent = AgentFactory::create_agent(config).await?;

    // Get market data for Bitcoin
    let market_data = agent.kraken_integration.get_market_data("BTCUSD").await?;
    
    info!("✅ Retrieved BTC/USD market data");
    info!("  Timestamp: {}", market_data.timestamp);
    info!("  Data available: ticker, orderbook, OHLCV");

    // Extract and display current price
    if let Some(ticker_result) = market_data.ticker.get("result") {
        if let Some(btc_data) = ticker_result.get("BTCUSD") {
            if let Some(prices) = btc_data.get("c").and_then(|c| c.as_array()) {
                if let Some(current_price) = prices.first().and_then(|p| p.as_str()) {
                    info!("  Current BTC Price: ${}", current_price);
                }
            }
        }
    }

    Ok(())
}

/// Example 2: Paper Trading
async fn example_2_paper_trading() -> Result<()> {
    info!("\n=== Example 2: Paper Trading ===");

    let config = AgentConfig::for_environment("development")?;
    let agent = AgentFactory::create_agent(config).await?;

    // Create a trading signal
    let signal = TradingSignal {
        action: "buy".to_string(),
        pair: "BTCUSD".to_string(),
        confidence: 0.85,
        position_size: 0.02,
        price: None,  // Market order
        reasoning: "RSI oversold condition detected".to_string(),
    };

    info!("📈 Executing paper trade: {} {} BTC", signal.action, signal.position_size);
    info!("  Reasoning: {}", signal.reasoning);
    info!("  Confidence: {:.1%}", signal.confidence);

    // Execute the trade
    let result = agent.kraken_integration.execute_trade(signal, 0.05).await?;

    info!("📊 Trade Result: {}", result.status);
    if result.status == "success" {
        if let Some(order_id) = &result.order_id {
            info!("  Order ID: {}", order_id);
        }
    } else {
        warn!("  Reason: {}", result.reason);
    }

    // Check account status after trade
    let account_status = agent.kraken_integration.get_account_status().await?;
    info!("  Open positions: {}", account_status.positions.len());

    Ok(())
}

/// Example 3: Multi-Asset Monitoring
async fn example_3_multi_asset_monitoring() -> Result<()> {
    info!("\n=== Example 3: Multi-Asset Monitoring ===");

    let config = AgentConfig::for_environment("development")?;
    let agent = AgentFactory::create_agent(config).await?;

    let symbols = vec!["BTCUSD", "ETHUSD", "SOLUSD"];
    let mut market_snapshots = Vec::new();

    for symbol in &symbols {
        let market_data = agent.kraken_integration.get_market_data(symbol).await?;
        
        // Extract key metrics
        if let Some(ticker_result) = market_data.ticker.get("result") {
            if let Some(asset_data) = ticker_result.get(symbol) {
                let snapshot = json!({
                    "symbol": symbol,
                    "price": asset_data.get("c").and_then(|c| c.as_array()).and_then(|arr| arr.first()).and_then(|p| p.as_str()).unwrap_or("N/A"),
                    "change": asset_data.get("p").and_then(|p| p.as_array()).and_then(|arr| arr.first()).and_then(|p| p.as_str()).unwrap_or("0"),
                    "volume": asset_data.get("v").and_then(|v| v.as_array()).and_then(|arr| arr.first()).and_then(|v| v.as_str()).unwrap_or("0"),
                    "high": asset_data.get("h").and_then(|h| h.as_array()).and_then(|arr| arr.first()).and_then(|h| h.as_str()).unwrap_or("0"),
                    "low": asset_data.get("l").and_then(|l| l.as_array()).and_then(|arr| arr.first()).and_then(|l| l.as_str()).unwrap_or("0"),
                });
                market_snapshots.push(snapshot);
            }
        }
    }

    info!("📊 Multi-Asset Market Snapshot:");
    for snapshot in &market_snapshots {
        info!("  {}: ${} ({})", 
            snapshot["symbol"], 
            snapshot["price"], 
            snapshot["change"]
        );
    }

    Ok(())
}

/// Example 4: Strategy Integration
async fn example_4_strategy_integration() -> Result<()> {
    info!("\n=== Example 4: Strategy Integration ===");

    let config = AgentConfig::for_environment("development")?;
    let agent = AgentFactory::create_agent(config).await?;

    // Get market data
    let market_data = agent.kraken_integration.get_market_data("BTCUSD").await?;

    // Create mock agent profile for strategy analysis
    let agent_profile = erc8004_kraken::erc8004::AgentProfile {
        info: erc8004_kraken::erc8004::AgentInfo {
            id: "example_agent".to_string(),
            metadata: "Example agent for demonstration".to_string(),
            verified: true,
            registration_timestamp: ethers::types::U256::zero(),
        },
        reputation: 85,
        reputation_history: vec![75, 80, 85],
    };

    // Test mean reversion strategy
    info!("🧠 Testing Mean Reversion Strategy...");
    let mean_reversion = erc8004_kraken::strategies::MeanReversionStrategy::new();
    
    match mean_reversion.analyze(&market_data, &agent_profile).await {
        Ok(signal) => {
            info!("  ✅ Signal generated:");
            info!("    Action: {}", signal.action);
            info!("    Confidence: {:.1%}", signal.confidence);
            info!("    Position Size: {:.2%}", signal.position_size);
            info!("    Reasoning: {}", signal.reasoning);
        }
        Err(e) => {
            info!("  ⚠️  No signal: {}", e);
        }
    }

    // Test momentum strategy
    info!("🚀 Testing Momentum Strategy...");
    let momentum = erc8004_kraken::strategies::MomentumStrategy::new();
    
    match momentum.analyze(&market_data, &agent_profile).await {
        Ok(signal) => {
            info!("  ✅ Signal generated:");
            info!("    Action: {}", signal.action);
            info!("    Confidence: {:.1%}", signal.confidence);
            info!("    Position Size: {:.2%}", signal.position_size);
            info!("    Reasoning: {}", signal.reasoning);
        }
        Err(e) => {
            info!("  ⚠️  No signal: {}", e);
        }
    }

    Ok(())
}

/// Example 5: ERC-8004 Validation
async fn example_5_erc8004_validation() -> Result<()> {
    info!("\n=== Example 5: ERC-8004 Validation ===");

    let config = AgentConfig::for_environment("development")?;
    let agent = AgentFactory::create_agent(config).await?;

    // Get agent profile
    let profile = agent.erc8004_client.get_agent_profile(&agent.config.agent_id).await?;

    if let Some(profile) = profile {
        info!("👤 Agent Profile:");
        info!("  ID: {}", profile.info.id);
        info!("  Verified: {}", profile.info.verified);
        info!("  Reputation: {}", profile.reputation);
        info!("  History: {:?}", profile.reputation_history);

        // Test validation for trading
        let can_trade = agent.erc8004_client.validate_agent_for_trading(
            &agent.config.agent_id, 
            70  // Minimum reputation
        ).await?;

        info!("🔒 Trading Validation: {}", if can_trade { "✅ Allowed" } else { "❌ Blocked" });

        // Update reputation based on simulated performance
        info!("📈 Updating reputation...");
        agent.erc8004_client.update_reputation(&agent.config.agent_id, 88).await?;
        
        let new_reputation = agent.erc8004_client.get_reputation(&agent.config.agent_id).await?;
        info!("  New reputation: {}", new_reputation);
    } else {
        warn!("⚠️  Agent not registered. Registering now...");
        
        let success = agent.erc8004_client.register_agent(
            &agent.config.agent_id,
            &format!("ERC-8004 Agent: {}", agent.config.agent_name),
        ).await?;

        if success {
            info!("✅ Agent registered successfully!");
            agent.erc8004_client.update_reputation(&agent.config.agent_id, 75).await?;
        }
    }

    Ok(())
}

/// Example 6: Risk Management
async fn example_6_risk_management() -> Result<()> {
    info!("\n=== Example 6: Risk Management ===");

    let config = AgentConfig::for_environment("development")?;
    let agent = AgentFactory::create_agent(config).await?;

    // Define risk parameters
    let risk_params = erc8004_kraken::agent::RiskParameters {
        max_position_size: 0.05,      // 5% max position
        max_daily_loss: 50.0,        // $50 max daily loss
        confidence_threshold: 0.8,   // 80% confidence required
        stop_loss_percentage: 0.03,  // 3% stop loss
        take_profit_percentage: 0.06, // 6% take profit
    };

    info!("⚖️  Risk Parameters:");
    info!("  Max Position Size: {:.1%}", risk_params.max_position_size);
    info!("  Max Daily Loss: ${:.2}", risk_params.max_daily_loss);
    info!("  Confidence Threshold: {:.1%}", risk_params.confidence_threshold);

    // Test signals with different risk levels
    let test_signals = vec![
        TradingSignal {
            action: "buy".to_string(),
            pair: "BTCUSD".to_string(),
            confidence: 0.9,  // High confidence
            position_size: 0.03,
            price: None,
            reasoning: "Strong momentum signal".to_string(),
        },
        TradingSignal {
            action: "buy".to_string(),
            pair: "BTCUSD".to_string(),
            confidence: 0.6,  // Low confidence
            position_size: 0.02,
            price: None,
            reasoning: "Weak signal".to_string(),
        },
        TradingSignal {
            action: "sell".to_string(),
            pair: "BTCUSD".to_string(),
            confidence: 0.85,
            position_size: 0.08,  // Too large
            price: None,
            reasoning: "Overbought conditions".to_string(),
        },
    ];

    for (i, signal) in test_signals.iter().enumerate() {
        info!("\n📊 Risk Assessment {}:", i + 1);
        info!("  Signal: {} @ {:.1%} confidence", signal.action, signal.confidence);
        info!("  Position Size: {:.2%}", signal.position_size);

        // Apply risk filters
        let mut risk_score = 0;
        let mut warnings = Vec::new();

        // Confidence check
        if signal.confidence < risk_params.confidence_threshold {
            warnings.push("Below confidence threshold");
        } else {
            risk_score += 1;
        }

        // Position size check
        if signal.position_size > risk_params.max_position_size {
            warnings.push("Position size too large");
        } else {
            risk_score += 1;
        }

        // Overall risk assessment
        let risk_level = match risk_score {
            2 => "🟢 Low Risk",
            1 => "🟡 Medium Risk",
            0 => "🔴 High Risk",
            _ => "❓ Unknown",
        };

        info!("  Risk Level: {}", risk_level);
        if !warnings.is_empty() {
            for warning in warnings {
                warn!("    ⚠️  {}", warning);
            }
        }

        // Execute if low risk
        if risk_score == 2 {
            let result = agent.kraken_integration.execute_trade(signal.clone(), risk_params.max_position_size).await?;
            info!("  ✅ Trade Executed: {}", result.status);
        } else {
            info!("  ❌ Trade Rejected: Risk too high");
        }
    }

    Ok(())
}

/// Example 7: Continuous Trading Loop
async fn example_7_continuous_trading() -> Result<()> {
    info!("\n=== Example 7: Continuous Trading Loop ===");

    let config = AgentConfig::for_environment("development")?;
    let agent = AgentFactory::create_agent(config).await?;

    // Start the agent
    agent.start().await?;
    info!("🚀 Agent started successfully");

    // Run a few trading cycles
    let symbols = vec!["BTCUSD".to_string()];
    let strategy = "mean_reversion";
    let cycles = 3;

    info!("🔄 Running {} trading cycles with strategy: {}", cycles, strategy);

    for cycle in 1..=cycles {
        info!("\n--- Trading Cycle {} ---", cycle);

        match agent.execute_trading_cycle(strategy, "BTCUSD").await {
            Ok(result) => {
                info!("Signal Generated: {}", result.signal_generated);
                info!("Signal Executed: {}", result.signal_executed);
                info!("Reason: {}", result.reason);
                
                if let Some(trade_result) = &result.trade_result {
                    info!("Trade Status: {}", trade_result.status);
                    if let Some(order_id) = &trade_result.order_id {
                        info!("Order ID: {}", order_id);
                    }
                }
            }
            Err(e) => {
                warn!("Trading cycle failed: {}", e);
            }
        }

        // Wait between cycles
        if cycle < cycles {
            sleep(Duration::from_secs(5)).await;
        }
    }

    // Stop the agent
    agent.stop().await?;
    info!("🛑 Agent stopped");

    // Show final status
    let final_status = agent.get_state();
    info!("\n📊 Final Agent Status:");
    info!("  Total Trades: {}", final_status.total_trades);
    info!("  Successful Trades: {}", final_status.successful_trades);
    info!("  Success Rate: {:.1%}", 
        if final_status.total_trades > 0 {
            final_status.successful_trades as f64 / final_status.total_trades as f64
        } else {
            0.0
        }
    );

    Ok(())
}
