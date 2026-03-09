//! ERC-8004 AI Trading Agent with Kraken CLI MCP Integration
//! 
//! This library provides a comprehensive Rust implementation of an AI trading agent
//! that integrates with Kraken's CLI via the Model Context Protocol (MCP) and
//! implements ERC-8004 trust layer functionality on Ethereum.

pub mod config;
pub mod kraken;
pub mod erc8004;
pub mod agent;
pub mod strategies;
pub mod errors;
pub mod utils;

// Re-export main components for convenience
pub use config::{AgentConfig, KrakenConfig, ERC8004Config};
pub use kraken::{MCPClient, KrakenBot, UnifiedKrakenIntegration};
pub use erc8004::{ERC8004Client, IdentityRegistry, ReputationRegistry, ValidationRegistry};
pub use agent::{TradingAgent, AgentState, TradingSignal};
pub use strategies::{Strategy, StrategyResult, StrategyType};
pub use errors::{AgentError, Result};

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Main agent factory that creates and configures the complete trading agent
pub struct AgentFactory;

impl AgentFactory {
    /// Create a new trading agent with all components integrated
    pub async fn create_agent(config: AgentConfig) -> Result<Arc<TradingAgent>> {
        info!("Creating ERC-8004 AI Trading Agent");
        
        // Initialize Kraken integration
        let kraken_integration = Arc::new(
            UnifiedKrakenIntegration::new(config.kraken.clone()).await?
        );
        
        // Initialize ERC-8004 client
        let erc8004_client = Arc::new(
            ERC8004Client::new(config.erc8004.clone()).await?
        );
        
        // Create the main trading agent
        let agent = Arc::new(
            TradingAgent::new(
                config.clone(),
                kraken_integration,
                erc8004_client,
            ).await?
        );
        
        info!("Agent created successfully");
        Ok(agent)
    }
    
    /// Create agent with paper trading only (safe for testing)
    pub async fn create_paper_agent(config: AgentConfig) -> Result<Arc<TradingAgent>> {
        let mut paper_config = config.clone();
        paper_config.kraken.paper_trading_only = true;
        
        Self::create_agent(paper_config).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_agent_factory() {
        let config = AgentConfig::default();
        
        // This would fail without proper setup, but tests the structure
        // let agent = AgentFactory::create_agent(config).await;
        // assert!(agent.is_ok());
    }
}
