//! Main Trading Agent Implementation
//! 
//! This module provides the core trading agent that combines ERC-8004 trust layer
//! with Kraken CLI MCP integration for safe and validated automated trading.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::config::AgentConfig;
use crate::kraken::{KrakenBot, MarketData, TradingSignal, TradeResult, UnifiedKrakenIntegration};
use crate::erc8004::{ERC8004Client, AgentProfile};

/// Main trading agent state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub agent_id: String,
    pub is_running: bool,
    pub current_positions: Vec<String>,
    pub last_trade_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub total_trades: u64,
    pub successful_trades: u64,
    pub current_balance: Option<f64>,
}

impl Default for AgentState {
    fn default() -> Self {
        Self {
            agent_id: "erc8004_agent".to_string(),
            is_running: false,
            current_positions: Vec::new(),
            last_trade_timestamp: None,
            total_trades: 0,
            successful_trades: 0,
            current_balance: None,
        }
    }
}

/// Core trading agent
pub struct TradingAgent {
    config: AgentConfig,
    kraken_integration: Arc<UnifiedKrakenIntegration>,
    erc8004_client: Arc<ERC8004Client>,
    state: Arc<RwLock<AgentState>>,
    strategy_registry: Arc<RwLock<HashMap<String, Box<dyn Strategy>>>>,
}

/// Strategy trait for implementing trading strategies
#[async_trait::async_trait]
pub trait Strategy: Send + Sync {
    /// Strategy name
    fn name(&self) -> &str;
    
    /// Strategy type
    fn strategy_type(&self) -> StrategyType;
    
    /// Analyze market data and generate trading signal
    async fn analyze(&self, market_data: &MarketData, agent_profile: &AgentProfile) -> Result<TradingSignal>;
    
    /// Validate signal before execution
    fn validate_signal(&self, signal: &TradingSignal) -> bool;
    
    /// Get risk parameters
    fn risk_parameters(&self) -> RiskParameters;
}

/// Strategy types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyType {
    Conservative,
    Balanced,
    Aggressive,
    Arbitrage,
    MarketMaking,
    TrendFollowing,
    MeanReversion,
}

/// Risk parameters for strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskParameters {
    pub max_position_size: f64,
    pub max_daily_loss: f64,
    pub confidence_threshold: f64,
    pub stop_loss_percentage: f64,
    pub take_profit_percentage: f64,
}

impl Default for RiskParameters {
    fn default() -> Self {
        Self {
            max_position_size: 0.1,      // 10% max position
            max_daily_loss: 100.0,      // $100 max daily loss
            confidence_threshold: 0.75, // 75% confidence required
            stop_loss_percentage: 0.05,  // 5% stop loss
            take_profit_percentage: 0.10, // 10% take profit
        }
    }
}

impl TradingAgent {
    /// Create new trading agent
    pub async fn new(
        config: AgentConfig,
        kraken_integration: Arc<UnifiedKrakenIntegration>,
        erc8004_client: Arc<ERC8004Client>,
    ) -> Result<Self> {
        info!("Creating trading agent with ID: {}", config.agent_id);

        let state = Arc::new(RwLock::new(AgentState {
            agent_id: config.agent_id.clone(),
            ..Default::default()
        }));

        let strategy_registry = Arc::new(RwLock::new(HashMap::new()));

        let agent = Self {
            config,
            kraken_integration,
            erc8004_client,
            state,
            strategy_registry,
        };

        // Register default strategies
        agent.register_default_strategies().await?;

        info!("Trading agent created successfully");
        Ok(agent)
    }

    /// Register default trading strategies
    async fn register_default_strategies(&self) -> Result<()> {
        let mut registry = self.strategy_registry.write().await;
        
        // Register mean reversion strategy
        registry.insert(
            "mean_reversion".to_string(),
            Box::new(crate::strategies::MeanReversionStrategy::new()),
        );

        // Register momentum strategy
        registry.insert(
            "momentum".to_string(),
            Box::new(crate::strategies::MomentumStrategy::new()),
        );

        info!("Default strategies registered");
        Ok(())
    }

    /// Start the trading agent
    pub async fn start(&self) -> Result<()> {
        info!("Starting trading agent");

        // Validate agent registration
        let agent_info = self.erc8004_client.get_agent_info(&self.config.agent_id).await?;
        if agent_info.is_none() || !agent_info.unwrap().verified {
            return Err(anyhow::anyhow!("Agent not registered or not verified"));
        }

        // Check reputation
        let reputation = self.erc8004_client.get_reputation(&self.config.agent_id).await?;
        if reputation < self.config.min_reputation_threshold {
            return Err(anyhow::anyhow!(
                "Agent reputation {} below threshold {}",
                reputation,
                self.config.min_reputation_threshold
            ));
        }

        // Update state
        {
            let mut state = self.state.write().await;
            state.is_running = true;
        }

        info!("Trading agent started successfully");
        Ok(())
    }

    /// Stop the trading agent
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping trading agent");

        // Cancel all open orders
        if let Some(bot) = &self.kraken_integration.mcp_bot {
            let cancelled = bot.cancel_all_orders().await?;
            info!("Cancelled {} open orders", cancelled.len());
        }

        // Update state
        {
            let mut state = self.state.write().await;
            state.is_running = false;
        }

        info!("Trading agent stopped");
        Ok(())
    }

    /// Execute trading cycle
    pub async fn execute_trading_cycle(&self, strategy_name: &str, symbol: &str) -> Result<TradingCycleResult> {
        debug!("Executing trading cycle for strategy: {} on symbol: {}", strategy_name, symbol);

        // Validate agent is running
        {
            let state = self.state.read().await;
            if !state.is_running {
                return Err(anyhow::anyhow!("Agent is not running"));
            }
        }

        // Get strategy
        let strategy = {
            let registry = self.strategy_registry.read().await;
            registry.get(strategy_name).cloned()
                .ok_or_else(|| anyhow::anyhow!("Strategy '{}' not found", strategy_name))?
        };

        // Get agent profile
        let agent_profile = self.erc8004_client.get_agent_profile(&self.config.agent_id).await?
            .ok_or_else(|| anyhow::anyhow!("Agent profile not found"))?;

        // Get market data
        let market_data = self.kraken_integration.get_market_data(symbol).await?;

        // Generate trading signal
        let signal = strategy.analyze(&market_data, &agent_profile).await?;

        // Validate signal
        if !strategy.validate_signal(&signal) {
            warn!("Trading signal validation failed");
            return Ok(TradingCycleResult {
                signal_generated: true,
                signal_executed: false,
                signal,
                trade_result: None,
                reason: "Signal validation failed".to_string(),
            });
        }

        // Execute trade
        let trade_result = self.kraken_integration.execute_trade(
            signal.clone(),
            strategy.risk_parameters().max_position_size,
        ).await;

        // Update state
        {
            let mut state = self.state.write().await;
            state.total_trades += 1;
            if trade_result.as_ref().map_or(false, |r| r.status == "success") {
                state.successful_trades += 1;
                state.last_trade_timestamp = Some(chrono::Utc::now());
            }
        }

        let success = trade_result.as_ref().map_or(false, |r| r.status == "success");

        Ok(TradingCycleResult {
            signal_generated: true,
            signal_executed: success,
            signal,
            trade_result: trade_result.ok(),
            reason: if success { "Trade executed successfully".to_string() } else { "Trade execution failed".to_string() },
        })
    }

    /// Get current agent state
    pub async fn get_state(&self) -> AgentState {
        self.state.read().await.clone()
    }

    /// Get account status
    pub async fn get_account_status(&self) -> Result<AccountStatus> {
        let kraken_status = self.kraken_integration.get_account_status().await?;
        
        let mut state = self.state.write().await;
        if let Some(balance) = kraken_status.balance.get("result") {
            // Extract USD balance (simplified)
            if let Some(usd_balance) = balance.get("ZUSD") {
                if let Some(balance_str) = usd_balance.as_str() {
                    if let Ok(balance_f64) = balance_str.parse::<f64>() {
                        state.current_balance = Some(balance_f64);
                    }
                }
            }
        }

        AccountStatus {
            agent_state: state.clone(),
            kraken_status,
        }
    }

    /// Register a new strategy
    pub async fn register_strategy(&self, name: String, strategy: Box<dyn Strategy>) -> Result<()> {
        let mut registry = self.strategy_registry.write().await;
        registry.insert(name, strategy);
        info!("Strategy registered successfully");
        Ok(())
    }

    /// Update agent reputation based on performance
    pub async fn update_performance_reputation(&self, profit_loss: f64) -> Result<()> {
        let current_reputation = self.erc8004_client.get_reputation(&self.config.agent_id).await?;
        
        // Simple reputation calculation based on P&L
        let reputation_change = if profit_loss > 0.0 {
            (profit_log10(profit_loss) * 10.0) as i64
        } else {
            -(profit_log10(-profit_loss) * 5.0) as i64
        };

        let new_reputation = (current_reputation + reputation_change).max(-100).min(100);
        
        self.erc8004_client.update_reputation(&self.config.agent_id, new_reputation).await?;
        
        info!("Updated reputation: {} -> {} (P&L: ${:.2})", current_reputation, new_reputation, profit_loss);
        Ok(())
    }

    /// Run continuous trading loop
    pub async fn run_trading_loop(&self, strategy_name: &str, symbols: Vec<String>, interval_seconds: u64) -> Result<()> {
        info!("Starting trading loop for strategy: {}", strategy_name);
        
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_seconds));
        
        loop {
            // Check if agent should continue running
            {
                let state = self.state.read().await;
                if !state.is_running {
                    info!("Trading loop stopped - agent not running");
                    break;
                }
            }

            interval.tick().await;

            for symbol in &symbols {
                match self.execute_trading_cycle(strategy_name, symbol).await {
                    Ok(result) => {
                        debug!("Trading cycle completed for {}: {}", symbol, result.reason);
                    }
                    Err(e) => {
                        error!("Trading cycle failed for {}: {}", symbol, e);
                    }
                }
            }
        }

        Ok(())
    }
}

/// Trading cycle result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingCycleResult {
    pub signal_generated: bool,
    pub signal_executed: bool,
    pub signal: TradingSignal,
    pub trade_result: Option<TradeResult>,
    pub reason: String,
}

/// Account status combining agent and Kraken data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStatus {
    pub agent_state: AgentState,
    pub kraken_status: crate::kraken::AccountStatus,
}

/// Helper function for log10 calculation
fn profit_log10(x: f64) -> f64 {
    if x <= 1.0 {
        0.0
    } else {
        x.log10()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_state_default() {
        let state = AgentState::default();
        assert_eq!(state.agent_id, "erc8004_agent");
        assert!(!state.is_running);
        assert_eq!(state.total_trades, 0);
    }

    #[test]
    fn test_risk_parameters_default() {
        let params = RiskParameters::default();
        assert_eq!(params.max_position_size, 0.1);
        assert_eq!(params.confidence_threshold, 0.75);
    }

    #[test]
    fn test_profit_log10() {
        assert_eq!(profit_log10(0.5), 0.0);
        assert_eq!(profit_log10(10.0), 1.0);
        assert_eq!(profit_log10(100.0), 2.0);
    }
}
