//! Configuration Management
//! 
//! This module provides configuration structures for the ERC-8004 AI trading agent,
//! including Kraken CLI settings, blockchain configuration, and agent parameters.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;

/// Main agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent identification
    pub agent_id: String,
    pub agent_name: String,
    
    /// Kraken integration configuration
    pub kraken: KrakenConfig,
    
    /// ERC-8004 blockchain configuration
    pub erc8004: ERC8004Config,
    
    /// Agent behavior settings
    pub min_reputation_threshold: i64,
    pub max_daily_trades: u32,
    pub emergency_stop: bool,
    
    /// Logging configuration
    pub log_level: String,
    pub log_file: Option<String>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            agent_id: "erc8004_agent_001".to_string(),
            agent_name: "ERC-8004 Trading Bot".to_string(),
            kraken: KrakenConfig::default(),
            erc8004: ERC8004Config::default(),
            min_reputation_threshold: 70,
            max_daily_trades: 50,
            emergency_stop: false,
            log_level: "info".to_string(),
            log_file: Some("logs/agent.log".to_string()),
        }
    }
}

/// Kraken configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrakenConfig {
    /// Path to kraken CLI executable
    pub kraken_cli_path: String,
    /// MCP services to enable
    pub services: Vec<String>,
    /// Allow dangerous operations
    pub allow_dangerous: bool,
    /// Paper trading only
    pub paper_trading_only: bool,
    /// Timeout for operations
    pub timeout_seconds: u64,
    /// Maximum position size
    pub max_position_size: f64,
    /// Risk level
    pub risk_level: String,
}

impl Default for KrakenConfig {
    fn default() -> Self {
        Self {
            kraken_cli_path: "kraken".to_string(),
            services: vec![
                "market".to_string(),
                "account".to_string(), 
                "paper".to_string()
            ],
            allow_dangerous: false,
            paper_trading_only: true,
            timeout_seconds: 30,
            max_position_size: 0.1,
            risk_level: "medium".to_string(),
        }
    }
}

/// ERC-8004 blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERC8004Config {
    /// Ethereum RPC URL
    pub rpc_url: String,
    /// Private key for signing transactions
    pub private_key: String,
    /// Chain ID
    pub chain_id: u64,
    /// Contract addresses
    pub identity_registry_address: String,
    pub reputation_registry_address: String,
    pub validation_registry_address: String,
}

impl Default for ERC8004Config {
    fn default() -> Self {
        Self {
            rpc_url: "https://sepolia.infura.io/v3/YOUR_PROJECT_ID".to_string(),
            private_key: "your_private_key_here".to_string(),
            chain_id: 11155111,  // Sepolia testnet
            identity_registry_address: "0x0000000000000000000000000000000000000000".to_string(),
            reputation_registry_address: "0x0000000000000000000000000000000000000000".to_string(),
            validation_registry_address: "0x0000000000000000000000000000000000000000".to_string(),
        }
    }
}

impl AgentConfig {
    /// Load configuration from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: AgentConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Load from environment variables
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // Agent settings
        if let Ok(agent_id) = std::env::var("AGENT_ID") {
            config.agent_id = agent_id;
        }
        
        if let Ok(agent_name) = std::env::var("AGENT_NAME") {
            config.agent_name = agent_name;
        }

        // Kraken settings
        if let Ok(kraken_path) = std::env::var("KRAKEN_CLI_PATH") {
            config.kraken.kraken_cli_path = kraken_path;
        }
        
        if let Ok(services) = std::env::var("KRAKEN_MCP_SERVICES") {
            config.kraken.services = services.split(',').map(|s| s.trim().to_string()).collect();
        }
        
        if let Ok(allow_dangerous) = std::env::var("KRAKEN_ALLOW_DANGEROUS") {
            config.kraken.allow_dangerous = allow_dangerous.parse().unwrap_or(false);
        }
        
        if let Ok(paper_only) = std::env::var("PAPER_TRADING_ONLY") {
            config.kraken.paper_trading_only = paper_only.parse().unwrap_or(true);
        }
        
        if let Ok(max_position) = std::env::var("MAX_POSITION_SIZE") {
            config.kraken.max_position_size = max_position.parse().unwrap_or(0.1);
        }
        
        if let Ok(risk_level) = std::env::var("RISK_LEVEL") {
            config.kraken.risk_level = risk_level;
        }

        // ERC-8004 settings
        if let Ok(rpc_url) = std::env::var("ETHEREUM_RPC_URL") {
            config.erc8004.rpc_url = rpc_url;
        }
        
        if let Ok(private_key) = std::env::var("PRIVATE_KEY") {
            config.erc8004.private_key = private_key;
        }
        
        if let Ok(chain_id) = std::env::var("NETWORK_ID") {
            config.erc8004.chain_id = chain_id.parse().unwrap_or(11155111);
        }
        
        if let Ok(identity_addr) = std::env::var("IDENTITY_REGISTRY_ADDRESS") {
            config.erc8004.identity_registry_address = identity_addr;
        }
        
        if let Ok(reputation_addr) = std::env::var("REPUTATION_REGISTRY_ADDRESS") {
            config.erc8004.reputation_registry_address = reputation_addr;
        }
        
        if let Ok(validation_addr) = std::env::var("VALIDATION_REGISTRY_ADDRESS") {
            config.erc8004.validation_registry_address = validation_addr;
        }

        // Agent behavior
        if let Ok(min_rep) = std::env::var("REPUTATION_THRESHOLD") {
            config.min_reputation_threshold = min_rep.parse().unwrap_or(70);
        }
        
        if let Ok(max_trades) = std::env::var("MAX_DAILY_TRADES") {
            config.max_daily_trades = max_trades.parse().unwrap_or(50);
        }

        // Logging
        if let Ok(log_level) = std::env::var("LOG_LEVEL") {
            config.log_level = log_level;
        }
        
        if let Ok(log_file) = std::env::var("LOG_FILE") {
            config.log_file = Some(log_file);
        }

        Ok(config)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate Kraken CLI path (basic check)
        if self.kraken.kraken_cli_path.is_empty() {
            return Err(anyhow::anyhow!("Kraken CLI path cannot be empty"));
        }

        // Validate RPC URL
        if !self.erc8004.rpc_url.starts_with("http") {
            return Err(anyhow::anyhow!("Invalid RPC URL format"));
        }

        // Validate private key (basic format check)
        if self.erc8004.private_key.len() != 64 {
            return Err(anyhow::anyhow!("Invalid private key format"));
        }

        // Validate contract addresses
        let addresses = [
            &self.erc8004.identity_registry_address,
            &self.erc8004.reputation_registry_address,
            &self.erc8004.validation_registry_address,
        ];

        for addr in addresses {
            if !addr.starts_with("0x") || addr.len() != 42 {
                return Err(anyhow::anyhow!("Invalid contract address format: {}", addr));
            }
        }

        // Validate agent settings
        if self.agent_id.is_empty() {
            return Err(anyhow::anyhow!("Agent ID cannot be empty"));
        }

        if self.min_reputation_threshold < 0 || self.min_reputation_threshold > 100 {
            return Err(anyhow::anyhow!("Reputation threshold must be between 0 and 100"));
        }

        if self.kraken.max_position_size <= 0.0 || self.kraken.max_position_size > 1.0 {
            return Err(anyhow::anyhow!("Max position size must be between 0 and 1"));
        }

        Ok(())
    }

    /// Get configuration for specific environment
    pub fn for_environment(env: &str) -> Result<Self> {
        match env {
            "development" => {
                let mut config = Self::default();
                config.kraken.paper_trading_only = true;
                config.kraken.allow_dangerous = false;
                config.log_level = "debug".to_string();
                Ok(config)
            }
            "testing" => {
                let mut config = Self::default();
                config.kraken.paper_trading_only = true;
                config.kraken.allow_dangerous = false;
                config.erc8004.chain_id = 31337;  // Local hardhat
                config.log_level = "info".to_string();
                Ok(config)
            }
            "production" => {
                let mut config = Self::default();
                config.kraken.paper_trading_only = false;
                config.kraken.allow_dangerous = true;
                config.erc8004.chain_id = 1;  // Ethereum mainnet
                config.log_level = "warn".to_string();
                Ok(config)
            }
            _ => Err(anyhow::anyhow!("Unknown environment: {}", env))
        }
    }

    /// Merge with another configuration (env vars override file config)
    pub fn merge_with_env(mut self) -> Result<Self> {
        let env_config = Self::from_env()?;
        
        // Override current config with environment values
        if env_config.agent_id != Self::default().agent_id {
            self.agent_id = env_config.agent_id;
        }
        
        if env_config.kraken.kraken_cli_path != Self::default().kraken.kraken_cli_path {
            self.kraken.kraken_cli_path = env_config.kraken.kraken_cli_path;
        }
        
        // ... continue for other fields
        
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = AgentConfig::default();
        assert_eq!(config.agent_id, "erc8004_agent_001");
        assert_eq!(config.kraken.services.len(), 3);
        assert!(config.kraken.paper_trading_only);
    }

    #[test]
    fn test_config_validation() {
        let mut config = AgentConfig::default();
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Invalid private key should fail
        config.erc8004.private_key = "invalid".to_string();
        assert!(config.validate().is_err());
        
        // Invalid address should fail
        config.erc8004.private_key = "0".repeat(64);  // Reset to valid
        config.erc8004.identity_registry_address = "invalid".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_environment_configs() {
        let dev_config = AgentConfig::for_environment("development").unwrap();
        assert!(dev_config.kraken.paper_trading_only);
        assert_eq!(dev_config.log_level, "debug");
        
        let prod_config = AgentConfig::for_environment("production").unwrap();
        assert!(!prod_config.kraken.paper_trading_only);
        assert_eq!(prod_config.log_level, "warn");
    }

    #[test]
    fn test_from_env() {
        // Set some environment variables
        env::set_var("AGENT_ID", "test_agent_123");
        env::set_var("KRAKEN_ALLOW_DANGEROUS", "true");
        env::set_var("REPUTATION_THRESHOLD", "80");
        
        let config = AgentConfig::from_env().unwrap();
        assert_eq!(config.agent_id, "test_agent_123");
        assert!(config.kraken.allow_dangerous);
        assert_eq!(config.min_reputation_threshold, 80);
        
        // Clean up
        env::remove_var("AGENT_ID");
        env::remove_var("KRAKEN_ALLOW_DANGEROUS");
        env::remove_var("REPUTATION_THRESHOLD");
    }
}
