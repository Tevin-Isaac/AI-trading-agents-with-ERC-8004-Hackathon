//! ERC-8004 AI Trading Agent - Main Entry Point
//! 
//! This is the main entry point for the ERC-8004 AI trading agent that integrates
//! with Kraken CLI via MCP and provides a comprehensive trading solution.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use erc8004_kraken::{AgentFactory, AgentConfig};

#[derive(Parser)]
#[command(name = "kraken-agent")]
#[command(about = "ERC-8004 AI Trading Agent with Kraken CLI MCP Integration")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Environment (development, testing, production)
    #[arg(short, long, default_value = "development")]
    env: String,
    
    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the trading agent
    Start {
        /// Strategy to use
        #[arg(short, long, default_value = "mean_reversion")]
        strategy: String,
        
        /// Trading symbols
        #[arg(short, long, value_delimiter = ',')]
        symbols: Vec<String>,
        
        /// Trading interval in seconds
        #[arg(short, long, default_value = "60")]
        interval: u64,
        
        /// Paper trading only
        #[arg(short, long)]
        paper: bool,
    },
    
    /// Register agent with ERC-8004
    Register {
        /// Agent metadata
        #[arg(short, long)]
        metadata: String,
    },
    
    /// Show agent status
    Status,
    
    /// Test Kraken CLI integration
    TestKraken,
    
    /// Run examples
    Examples {
        /// Example to run
        #[arg(short, long)]
        example: Option<String>,
    },
    
    /// Initialize configuration
    InitConfig,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(&cli.log_level)?;

    // Load configuration
    let config = load_config(&cli.config, &cli.env)?;
    config.validate()?;

    info!("Starting ERC-8004 AI Trading Agent");
    info!("Agent ID: {}", config.agent_id);
    info!("Environment: {}", cli.env);

    match cli.command {
        Commands::Start { strategy, symbols, interval, paper } => {
            start_agent(config, strategy, symbols, interval, paper).await?;
        }
        Commands::Register { metadata } => {
            register_agent(config, metadata).await?;
        }
        Commands::Status => {
            show_status(config).await?;
        }
        Commands::TestKraken => {
            test_kraken_integration(config).await?;
        }
        Commands::Examples { example } => {
            run_examples(config, example).await?;
        }
        Commands::InitConfig => {
            init_config(&cli.config)?;
        }
    }

    Ok(())
}

/// Initialize logging
fn init_logging(level: &str) -> Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}

/// Load configuration
fn load_config(config_path: &str, env: &str) -> Result<AgentConfig> {
    let config = if std::path::Path::new(config_path).exists() {
        let mut config = AgentConfig::from_file(config_path)?;
        config.merge_with_env()
    } else {
        let config = AgentConfig::for_environment(env)?;
        config.save_to_file(config_path)?;
        warn!("Created new config file at: {}", config_path);
        config
    }?;

    Ok(config)
}

/// Start the trading agent
async fn start_agent(
    config: AgentConfig,
    strategy: String,
    symbols: Vec<String>,
    interval: u64,
    paper: bool,
) -> Result<()> {
    info!("Starting trading agent with strategy: {}", strategy);
    info!("Trading symbols: {:?}", symbols);
    info!("Interval: {} seconds", interval);

    // Create agent
    let agent_config = if paper {
        let mut paper_config = config.clone();
        paper_config.kraken.paper_trading_only = true;
        paper_config
    } else {
        config
    };

    let agent = AgentFactory::create_agent(agent_config).await?;

    // Register agent if not already registered
    let agent_info = agent.erc8004_client.get_agent_info(&agent.config.agent_id).await?;
    if agent_info.is_none() {
        info!("Registering new agent...");
        agent.erc8004_client.register_agent(
            &agent.config.agent_id,
            &format!("ERC-8004 Agent: {}", agent.config.agent_name),
        ).await?;
    }

    // Start agent
    agent.start().await?;

    // Run trading loop
    info!("Starting trading loop...");
    agent.run_trading_loop(&strategy, symbols, interval).await?;

    Ok(())
}

/// Register agent with ERC-8004
async fn register_agent(config: AgentConfig, metadata: String) -> Result<()> {
    info!("Registering agent with ERC-8004...");

    let agent = AgentFactory::create_agent(config).await?;
    
    let success = agent.erc8004_client.register_agent(
        &agent.config.agent_id,
        &metadata,
    ).await?;

    if success {
        info!("Agent registered successfully!");
        
        // Set initial reputation
        agent.erc8004_client.update_reputation(&agent.config.agent_id, 75).await?;
        info!("Initial reputation set to 75");
    } else {
        error!("Failed to register agent");
    }

    Ok(())
}

/// Show agent status
async fn show_status(config: AgentConfig) -> Result<()> {
    let agent = AgentFactory::create_agent(config).await?;
    
    // Get agent profile
    let profile = agent.erc8004_client.get_agent_profile(&agent.config.agent_id).await?;
    
    if let Some(profile) = profile {
        println!("=== Agent Status ===");
        println!("ID: {}", profile.info.id);
        println!("Name: {}", profile.info.metadata);
        println!("Verified: {}", profile.info.verified);
        println!("Reputation: {}", profile.reputation);
        println!("Reputation History: {:?}", profile.reputation_history);
    } else {
        println!("Agent not found or not registered");
    }

    // Get account status
    let account_status = agent.get_account_status().await?;
    println!("\n=== Account Status ===");
    println!("Total Trades: {}", account_status.agent_state.total_trades);
    println!("Successful Trades: {}", account_status.agent_state.successful_trades);
    println!("Current Balance: ${:.2}", account_status.agent_state.current_balance.unwrap_or(0.0));
    println!("Open Positions: {}", account_status.kraken_status.positions.len());

    Ok(())
}

/// Test Kraken CLI integration
async fn test_kraken_integration(config: AgentConfig) -> Result<()> {
    info!("Testing Kraken CLI integration...");

    let agent = AgentFactory::create_agent(config).await?;
    
    // Test market data retrieval
    let market_data = agent.kraken_integration.get_market_data("BTCUSD").await?;
    println!("✅ Market data retrieved for BTCUSD");
    
    // Test account status
    let account_status = agent.kraken_integration.get_account_status().await?;
    println!("✅ Account status retrieved");
    
    // Test paper trading (if enabled)
    if agent.config.kraken.paper_trading_only {
        let signal = erc8004_kraken::TradingSignal {
            action: "buy".to_string(),
            pair: "BTCUSD".to_string(),
            confidence: 0.8,
            position_size: 0.01,
            price: None,
            reasoning: "Test signal".to_string(),
        };
        
        let result = agent.kraken_integration.execute_trade(signal, 0.01).await?;
        println!("✅ Paper trade executed: {}", result.status);
    }

    println!("✅ All Kraken integration tests passed!");
    Ok(())
}

/// Run examples
async fn run_examples(config: AgentConfig, example: Option<String>) -> Result<()> {
    info!("Running examples...");

    match example.as_deref() {
        Some("market_data") => {
            let agent = AgentFactory::create_agent(config).await?;
            let market_data = agent.kraken_integration.get_market_data("BTCUSD").await?;
            println!("Market Data: {}", serde_json::to_string_pretty(&market_data)?);
        }
        Some("paper_trade") => {
            let agent = AgentFactory::create_agent(config).await?;
            
            let signal = erc8004_kraken::TradingSignal {
                action: "buy".to_string(),
                pair: "BTCUSD".to_string(),
                confidence: 0.8,
                position_size: 0.01,
                price: None,
                reasoning: "Example paper trade".to_string(),
            };
            
            let result = agent.kraken_integration.execute_trade(signal, 0.01).await?;
            println!("Paper Trade Result: {}", serde_json::to_string_pretty(&result)?);
        }
        Some("erc8004") => {
            let agent = AgentFactory::create_agent(config).await?;
            let profile = agent.erc8004_client.get_agent_profile(&agent.config.agent_id).await?;
            
            if let Some(profile) = profile {
                println!("Agent Profile: {}", serde_json::to_string_pretty(&profile)?);
            } else {
                println!("Agent not registered");
            }
        }
        Some("strategies") => {
            let agent = AgentFactory::create_agent(config).await?;
            
            let market_data = agent.kraken_integration.get_market_data("BTCUSD").await?;
            let agent_profile = erc8004_kraken::erc8004::AgentProfile {
                info: erc8004_kraken::erc8004::AgentInfo {
                    id: "test".to_string(),
                    metadata: "test".to_string(),
                    verified: true,
                    registration_timestamp: ethers::types::U256::zero(),
                },
                reputation: 80,
                reputation_history: vec![80],
            };
            
            // Test mean reversion strategy
            let strategy = erc8004_kraken::strategies::MeanReversionStrategy::new();
            match strategy.analyze(&market_data, &agent_profile).await {
                Ok(signal) => {
                    println!("Mean Reversion Signal: {}", serde_json::to_string_pretty(&signal)?);
                }
                Err(e) => {
                    println!("No signal: {}", e);
                }
            }
        }
        _ => {
            println!("Available examples:");
            println!("  market_data    - Get market data for BTCUSD");
            println!("  paper_trade    - Execute a paper trade");
            println!("  erc8004        - Show ERC-8004 agent profile");
            println!("  strategies     - Test trading strategies");
        }
    }

    Ok(())
}

/// Initialize configuration file
fn init_config(config_path: &str) -> Result<()> {
    let config = AgentConfig::default();
    config.save_to_file(config_path)?;
    
    println!("Configuration initialized at: {}", config_path);
    println!("Please edit the file and update the following values:");
    println!("  - erc8004.rpc_url");
    println!("  - erc8004.private_key");
    println!("  - Contract addresses");
    println!("  - Agent settings");
    
    Ok(())
}
