//! ERC-8004 Trust Layer Implementation
//! 
//! This module provides Rust implementations for ERC-8004 standard contracts
//! including identity registry, reputation system, and validation registry.

use anyhow::Result;
use ethers::prelude::*;
use ethers::contract::abigen;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

// Generate contract bindings from ABIs
abigen!(
    IdentityRegistry,
    r#"[
        function registerAgent(string memory agentId, string memory metadata) external returns (bool)
        function verifyAgent(string memory agentId) external view returns (bool)
        function getAgentMetadata(string memory agentId) external view returns (string memory)
        function getRegisteredAgents() external view returns (string[] memory)
        event AgentRegistered(string indexed agentId, string metadata, uint256 timestamp)
        event AgentVerified(string indexed agentId, bool verified, uint256 timestamp)
    ]"#;

    ReputationRegistry,
    r#"[
        function updateReputation(string memory agentId, int256 score) external returns (bool)
        function getReputation(string memory agentId) external view returns (int256)
        function getReputationHistory(string memory agentId) external view returns (int256[] memory)
        function batchUpdateReputation(string[] memory agentIds, int256[] memory scores) external returns (bool)
        event ReputationUpdated(string indexed agentId, int256 newScore, uint256 timestamp)
        event ReputationBatchUpdated(string[] agentIds, int256[] scores, uint256 timestamp)
    ]"#;

    ValidationRegistry,
    r#"[
        function validateStrategy(string memory strategyId, string memory metadata) external returns (bool)
        function isValidStrategy(string memory strategyId) external view returns (bool)
        function getStrategyMetadata(string memory strategyId) external view returns (string memory)
        function getValidStrategies() external view returns (string[] memory)
        event StrategyValidated(string indexed strategyId, bool valid, uint256 timestamp)
    ]"#;
);

/// Configuration for ERC-8004 contracts
#[derive(Debug, Clone)]
pub struct ERC8004Config {
    /// Ethereum RPC URL
    pub rpc_url: String,
    /// Private key for signing transactions
    pub private_key: String,
    /// Chain ID
    pub chain_id: u64,
    /// Contract addresses
    pub identity_registry_address: Address,
    pub reputation_registry_address: Address,
    pub validation_registry_address: Address,
}

/// ERC-8004 client managing all three registries
pub struct ERC8004Client {
    provider: Arc<Provider<Http>>,
    wallet: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    identity_registry: IdentityRegistry<SignerMiddleware<Provider<Http>, LocalWallet>>,
    reputation_registry: ReputationRegistry<SignerMiddleware<Provider<Http>, LocalWallet>>,
    validation_registry: ValidationRegistry<SignerMiddleware<Provider<Http>, LocalWallet>>,
    
    // Local cache for performance
    agent_cache: Arc<RwLock<HashMap<String, AgentInfo>>>,
    reputation_cache: Arc<RwLock<HashMap<String, i64>>>,
    strategy_cache: Arc<RwLock<HashMap<String, StrategyInfo>>>,
}

/// Agent information from identity registry
#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub id: String,
    pub metadata: String,
    pub verified: bool,
    pub registration_timestamp: U256,
}

/// Strategy information from validation registry
#[derive(Debug, Clone)]
pub struct StrategyInfo {
    pub id: String,
    pub metadata: String,
    pub valid: bool,
    pub validation_timestamp: U256,
}

impl ERC8004Client {
    /// Create new ERC-8004 client
    pub async fn new(config: ERC8004Config) -> Result<Self> {
        info!("Initializing ERC-8004 client");

        // Setup provider and wallet
        let provider = Provider::<Http>::try_from(&config.rpc_url)?;
        let provider = Arc::new(provider);
        
        let chain_id = config.chain_id;
        let wallet: LocalWallet = config.private_key.parse::<LocalWallet>()?
            .with_chain_id(chain_id);
        let wallet = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

        // Initialize contract instances
        let identity_registry = IdentityRegistry::new(
            config.identity_registry_address,
            wallet.clone(),
        );
        
        let reputation_registry = ReputationRegistry::new(
            config.reputation_registry_address,
            wallet.clone(),
        );
        
        let validation_registry = ValidationRegistry::new(
            config.validation_registry_address,
            wallet.clone(),
        );

        info!("ERC-8004 client initialized successfully");

        Ok(Self {
            provider,
            wallet,
            identity_registry,
            reputation_registry,
            validation_registry,
            agent_cache: Arc::new(RwLock::new(HashMap::new())),
            reputation_cache: Arc::new(RwLock::new(HashMap::new())),
            strategy_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Register a new agent
    pub async fn register_agent(&self, agent_id: &str, metadata: &str) -> Result<bool> {
        info!("Registering agent: {}", agent_id);

        let call = self.identity_registry.register_agent(
            agent_id.to_string(),
            metadata.to_string(),
        );

        let receipt = call.send().await?.await?;
        let success = receipt.status == Some(U64::from(1));

        if success {
            info!("Agent {} registered successfully", agent_id);
            
            // Update cache
            let mut cache = self.agent_cache.write().await;
            cache.insert(agent_id.to_string(), AgentInfo {
                id: agent_id.to_string(),
                metadata: metadata.to_string(),
                verified: false,
                registration_timestamp: receipt.block_number.unwrap_or_default(),
            });
        } else {
            error!("Failed to register agent: {}", agent_id);
        }

        Ok(success)
    }

    /// Verify an agent
    pub async fn verify_agent(&self, agent_id: &str) -> Result<bool> {
        info!("Verifying agent: {}", agent_id);

        let call = self.identity_registry.verify_agent(agent_id.to_string());
        let verified = call.call().await?;

        // Update cache
        let mut cache = self.agent_cache.write().await;
        if let Some(agent_info) = cache.get_mut(agent_id) {
            agent_info.verified = verified;
        }

        Ok(verified)
    }

    /// Get agent information
    pub async fn get_agent_info(&self, agent_id: &str) -> Result<Option<AgentInfo>> {
        // Check cache first
        {
            let cache = self.agent_cache.read().await;
            if let Some(info) = cache.get(agent_id) {
                return Ok(Some(info.clone()));
            }
        }

        // Fetch from blockchain
        let metadata = self.identity_registry.get_agent_metadata(agent_id.to_string()).call().await?;
        let verified = self.identity_registry.verify_agent(agent_id.to_string()).call().await?;

        let agent_info = AgentInfo {
            id: agent_id.to_string(),
            metadata,
            verified,
            registration_timestamp: U256::zero(), // Would need to fetch from events
        };

        // Update cache
        let mut cache = self.agent_cache.write().await;
        cache.insert(agent_id.to_string(), agent_info.clone());

        Ok(Some(agent_info))
    }

    /// Update agent reputation
    pub async fn update_reputation(&self, agent_id: &str, score: i64) -> Result<bool> {
        info!("Updating reputation for {}: {}", agent_id, score);

        let call = self.reputation_registry.update_reputation(
            agent_id.to_string(),
            score.into(),
        );

        let receipt = call.send().await?.await?;
        let success = receipt.status == Some(U64::from(1));

        if success {
            // Update cache
            let mut cache = self.reputation_cache.write().await;
            cache.insert(agent_id.to_string(), score);
        }

        Ok(success)
    }

    /// Get agent reputation
    pub async fn get_reputation(&self, agent_id: &str) -> Result<i64> {
        // Check cache first
        {
            let cache = self.reputation_cache.read().await;
            if let Some(&score) = cache.get(agent_id) {
                return Ok(score);
            }
        }

        // Fetch from blockchain
        let score = self.reputation_registry.get_reputation(agent_id.to_string()).call().await?;
        let score_i64 = score.as_i64();

        // Update cache
        let mut cache = self.reputation_cache.write().await;
        cache.insert(agent_id.to_string(), score_i64);

        Ok(score_i64)
    }

    /// Batch update reputation for multiple agents
    pub async fn batch_update_reputation(&self, updates: Vec<(String, i64)>) -> Result<bool> {
        if updates.is_empty() {
            return Ok(true);
        }

        info!("Batch updating reputation for {} agents", updates.len());

        let (agent_ids, scores): (Vec<_>, Vec<_>) = updates.into_iter().unzip();
        let scores_u256: Vec<U256> = scores.into_iter().map(|s| s.into()).collect();

        let call = self.reputation_registry.batch_update_reputation(
            agent_ids,
            scores_u256,
        );

        let receipt = call.send().await?.await?;
        let success = receipt.status == Some(U64::from(1));

        if success {
            // Update cache
            let mut cache = self.reputation_cache.write().await;
            for (agent_id, score) in agent_ids.into_iter().zip(scores) {
                cache.insert(agent_id, score);
            }
        }

        Ok(success)
    }

    /// Validate a trading strategy
    pub async fn validate_strategy(&self, strategy_id: &str, metadata: &str) -> Result<bool> {
        info!("Validating strategy: {}", strategy_id);

        let call = self.validation_registry.validate_strategy(
            strategy_id.to_string(),
            metadata.to_string(),
        );

        let receipt = call.send().await?.await?;
        let success = receipt.status == Some(U64::from(1));

        if success {
            // Update cache
            let mut cache = self.strategy_cache.write().await;
            cache.insert(strategy_id.to_string(), StrategyInfo {
                id: strategy_id.to_string(),
                metadata: metadata.to_string(),
                valid: true,
                validation_timestamp: receipt.block_number.unwrap_or_default(),
            });
        }

        Ok(success)
    }

    /// Check if strategy is valid
    pub async fn is_valid_strategy(&self, strategy_id: &str) -> Result<bool> {
        // Check cache first
        {
            let cache = self.strategy_cache.read().await;
            if let Some(strategy_info) = cache.get(strategy_id) {
                return Ok(strategy_info.valid);
            }
        }

        // Fetch from blockchain
        let valid = self.validation_registry.is_valid_strategy(strategy_id.to_string()).call().await?;

        // Update cache
        let mut cache = self.strategy_cache.write().await;
        cache.insert(strategy_id.to_string(), StrategyInfo {
            id: strategy_id.to_string(),
            metadata: String::new(),
            valid,
            validation_timestamp: U256::zero(),
        });

        Ok(valid)
    }

    /// Get strategy information
    pub async fn get_strategy_info(&self, strategy_id: &str) -> Result<Option<StrategyInfo>> {
        // Check cache first
        {
            let cache = self.strategy_cache.read().await;
            if let Some(info) = cache.get(strategy_id) {
                return Ok(Some(info.clone()));
            }
        }

        // Fetch from blockchain
        let metadata = self.validation_registry.get_strategy_metadata(strategy_id.to_string()).call().await?;
        let valid = self.validation_registry.is_valid_strategy(strategy_id.to_string()).call().await?;

        let strategy_info = StrategyInfo {
            id: strategy_id.to_string(),
            metadata,
            valid,
            validation_timestamp: U256::zero(),
        };

        // Update cache
        let mut cache = self.strategy_cache.write().await;
        cache.insert(strategy_id.to_string(), strategy_info.clone());

        Ok(Some(strategy_info))
    }

    /// Validate agent before allowing trade
    pub async fn validate_agent_for_trading(&self, agent_id: &str, min_reputation: i64) -> Result<bool> {
        debug!("Validating agent {} for trading", agent_id);

        // Check if agent exists and is verified
        let agent_info = self.get_agent_info(agent_id).await?;
        if agent_info.is_none() || !agent_info.unwrap().verified {
            warn!("Agent {} not found or not verified", agent_id);
            return Ok(false);
        }

        // Check reputation
        let reputation = self.get_reputation(agent_id).await?;
        if reputation < min_reputation {
            warn!("Agent {} reputation {} below threshold {}", agent_id, reputation, min_reputation);
            return Ok(false);
        }

        debug!("Agent {} validated for trading", agent_id);
        Ok(true)
    }

    /// Get comprehensive agent profile
    pub async fn get_agent_profile(&self, agent_id: &str) -> Result<Option<AgentProfile>> {
        let agent_info = match self.get_agent_info(agent_id).await? {
            Some(info) => info,
            None => return Ok(None),
        };

        let reputation = self.get_reputation(agent_id).await?;
        let reputation_history = self.reputation_registry
            .get_reputation_history(agent_id.to_string())
            .call()
            .await?
            .into_iter()
            .map(|s| s.as_i64())
            .collect();

        Ok(Some(AgentProfile {
            info: agent_info,
            reputation,
            reputation_history,
        }))
    }

    /// Clear all caches (useful for testing or reset)
    pub async fn clear_caches(&self) {
        self.agent_cache.write().await.clear();
        self.reputation_cache.write().await.clear();
        self.strategy_cache.write().await.clear();
        info!("ERC-8004 caches cleared");
    }
}

/// Comprehensive agent profile
#[derive(Debug, Clone)]
pub struct AgentProfile {
    pub info: AgentInfo,
    pub reputation: i64,
    pub reputation_history: Vec<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_agent_info_serialization() {
        let info = AgentInfo {
            id: "test_agent".to_string(),
            metadata: "Test metadata".to_string(),
            verified: true,
            registration_timestamp: U256::from(12345),
        };

        // Test that the struct can be created and accessed
        assert_eq!(info.id, "test_agent");
        assert_eq!(info.verified, true);
    }

    #[test]
    fn test_address_parsing() {
        let addr_str = "0x1234567890123456789012345678901234567890";
        let addr = Address::from_str(addr_str).unwrap();
        assert_eq!(addr.to_string(), addr_str);
    }
}
