//! Kraken CLI MCP Integration
//! 
//! This module provides Rust implementations for interacting with Kraken's CLI
//! through the Model Context Protocol (MCP), offering a type-safe and performant
//! alternative to the Python implementation.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use tokio::process::{Command as AsyncCommand, Child};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Configuration for Kraken CLI MCP server
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
}

impl Default for KrakenConfig {
    fn default() -> Self {
        Self {
            kraken_cli_path: "kraken".to_string(),
            services: vec!["market".to_string(), "account".to_string(), "paper".to_string()],
            allow_dangerous: false,
            paper_trading_only: true,
            timeout_seconds: 30,
        }
    }
}

/// JSON-RPC request for MCP communication
#[derive(Debug, Serialize)]
struct MCPRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// JSON-RPC response from MCP server
#[derive(Debug, Deserialize)]
struct MCPResponse {
    jsonrpc: String,
    id: Option<Value>,
    result: Option<Value>,
    error: Option<MCPError>,
}

/// MCP error structure
#[derive(Debug, Deserialize)]
struct MCPError {
    code: i32,
    message: String,
    data: Option<Value>,
}

/// MCP client for Kraken CLI
pub struct MCPClient {
    config: KrakenConfig,
    child: Option<Child>,
    next_request_id: i32,
}

impl MCPClient {
    /// Create new MCP client
    pub fn new(config: KrakenConfig) -> Self {
        Self {
            config,
            child: None,
            next_request_id: 1,
        }
    }

    /// Start Kraken CLI MCP server
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Kraken CLI MCP server");

        let mut cmd = AsyncCommand::new(&self.config.kraken_cli_path);
        cmd.arg("mcp")
           .arg("-s")
           .arg(self.config.services.join(","))
           .stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        if self.config.allow_dangerous {
            cmd.arg("--allow-dangerous");
        }

        let mut child = cmd.spawn()?;
        
        // Initialize MCP session
        self.initialize_session(&mut child).await?;
        
        self.child = Some(child);
        info!("MCP server started successfully");
        Ok(())
    }

    /// Initialize MCP session with server
    async fn initialize_session(&mut self, child: &mut Child) -> Result<()> {
        let init_request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(self.next_request_id)),
            method: "initialize".to_string(),
            params: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "clientInfo": {
                    "name": "ERC-8004 AI Trading Agent",
                    "version": "1.0.0"
                }
            })),
        };

        let response = self.send_request(child, init_request).await?;
        
        if response.error.is_some() {
            return Err(anyhow::anyhow!("MCP initialization failed: {:?}", response.error));
        }

        // Send initialized notification
        let notification = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: "notifications/initialized".to_string(),
            params: None,
        };

        self.send_notification(child, notification).await?;
        
        self.next_request_id += 1;
        Ok(())
    }

    /// Send JSON-RPC request and wait for response
    async fn send_request(&mut self, child: &mut Child, request: MCPRequest) -> Result<MCPResponse> {
        let stdin = child.stdin.as_mut().ok_or_else(|| anyhow::anyhow!("No stdin available"))?;
        
        let request_json = serde_json::to_string(&request)?;
        stdin.write_all(request_json.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;

        // Read response
        let stdout = child.stdout.as_mut().ok_or_else(|| anyhow::anyhow!("No stdout available"))?;
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        
        reader.read_line(&mut line).await?;
        
        let response: MCPResponse = serde_json::from_str(&line.trim())?;
        Ok(response)
    }

    /// Send JSON-RPC notification (no response expected)
    async fn send_notification(&mut self, child: &mut Child, notification: MCPRequest) -> Result<()> {
        let stdin = child.stdin.as_mut().ok_or_else(|| anyhow::anyhow!("No stdin available"))?;
        
        let notification_json = serde_json::to_string(&notification)?;
        stdin.write_all(notification_json.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;
        
        Ok(())
    }

    /// List available tools from Kraken CLI
    pub async fn list_tools(&mut self) -> Result<Vec<Value>> {
        let child = self.child.as_mut().ok_or_else(|| anyhow::anyhow!("MCP server not started"))?;
        
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(self.next_request_id)),
            method: "tools/list".to_string(),
            params: None,
        };

        let response = self.send_request(child, request).await?;
        self.next_request_id += 1;

        if let Some(error) = response.error {
            return Err(anyhow::anyhow!("Failed to list tools: {:?}", error));
        }

        let tools = response.result
            .and_then(|r| r.get("tools").cloned())
            .and_then(|t| serde_json::from_value::<Vec<Value>>(t).ok())
            .unwrap_or_default();

        Ok(tools)
    }

    /// Call a specific tool on Kraken CLI
    pub async fn call_tool(&mut self, tool_name: &str, arguments: Option<Value>) -> Result<Value> {
        let child = self.child.as_mut().ok_or_else(|| anyhow::anyhow!("MCP server not started"))?;
        
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(self.next_request_id)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": tool_name,
                "arguments": arguments.unwrap_or(json!({}))
            })),
        };

        let response = self.send_request(child, request).await?;
        self.next_request_id += 1;

        if let Some(error) = response.error {
            return Err(anyhow::anyhow!("Tool call failed: {:?}", error));
        }

        response.result.ok_or_else(|| anyhow::anyhow!("No result in response"))
    }

    /// Get ticker information for a trading pair
    pub async fn get_ticker(&mut self, pair: &str) -> Result<Value> {
        self.call_tool("kraken_ticker", Some(json!({ "pair": [pair] }))).await
    }

    /// Get account balance
    pub async fn get_balance(&mut self) -> Result<Value> {
        self.call_tool("kraken_balance", None).await
    }

    /// Place a trading order
    pub async fn place_order(&mut self, pair: &str, side: &str, volume: f64, price: Option<f64>) -> Result<Value> {
        let mut args = json!({
            "pair": pair,
            "volume": volume.to_string()
        });

        if let Some(p) = price {
            args["price"] = json!(p.to_string());
        }

        let tool_name = format!("kraken_order_{}", side);
        self.call_tool(&tool_name, Some(args)).await
    }

    /// Get open orders
    pub async fn get_open_orders(&mut self) -> Result<Value> {
        self.call_tool("kraken_open_orders", None).await
    }

    /// Cancel an order
    pub async fn cancel_order(&mut self, txid: &str) -> Result<Value> {
        self.call_tool("kraken_order_cancel", Some(json!({ "txid": [txid] }))).await
    }

    /// Get OHLCV data
    pub async fn get_ohlcv(&mut self, pair: &str, interval: i32) -> Result<Value> {
        self.call_tool("kraken_ohlc", Some(json!({ 
            "pair": pair, 
            "interval": interval 
        }))).await
    }

    /// Get order book
    pub async fn get_orderbook(&mut self, pair: &str, count: i32) -> Result<Value> {
        self.call_tool("kraken_orderbook", Some(json!({ 
            "pair": pair, 
            "count": count 
        }))).await
    }

    /// Reset paper trading state
    pub async fn reset_paper_trading(&mut self) -> Result<Value> {
        self.call_tool("kraken_paper_reset", None).await
    }
}

impl Drop for MCPClient {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            debug!("Terminating MCP server");
            let _ = child.kill();
        }
    }
}

/// High-level Kraken trading bot using MCP
pub struct KrakenBot {
    mcp_client: Arc<RwLock<MCPClient>>,
    position_tracker: Arc<RwLock<HashMap<String, Position>>>,
}

/// Position tracking information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub pair: String,
    pub side: String,
    pub size: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub entry_price: Option<f64>,
    pub order_id: String,
}

impl KrakenBot {
    /// Create new Kraken bot
    pub async fn new(config: KrakenConfig) -> Result<Self> {
        let mut mcp_client = MCPClient::new(config);
        mcp_client.start().await?;

        Ok(Self {
            mcp_client: Arc::new(RwLock::new(mcp_client)),
            position_tracker: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get comprehensive market data
    pub async fn get_market_data(&self, pair: &str) -> Result<MarketData> {
        let mut client = self.mcp_client.write().await;
        
        let ticker = client.get_ticker(pair).await?;
        let orderbook = client.get_orderbook(pair, 100).await?;
        let ohlcv = client.get_ohlcv(pair, 60).await?;

        Ok(MarketData {
            pair: pair.to_string(),
            ticker,
            orderbook,
            ohlcv,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Execute trading signal
    pub async fn execute_trade(&self, signal: TradingSignal, max_position_size: f64) -> Result<TradeResult> {
        if signal.confidence < 0.7 {
            return Ok(TradeResult {
                status: "rejected".to_string(),
                reason: "Low confidence".to_string(),
                order_id: None,
            });
        }

        let actual_size = (signal.position_size * max_position_size).min(max_position_size);
        
        let mut client = self.mcp_client.write().await;
        let order_result = client.place_order(
            &signal.pair,
            &signal.action,
            actual_size,
            signal.price,
        ).await?;

        // Extract order ID
        let order_id = order_result
            .get("result")
            .and_then(|r| r.get("txid"))
            .and_then(|txid| txid.as_array())
            .and_then(|arr| arr.first())
            .and_then(|id| id.as_str())
            .unwrap_or("unknown")
            .to_string();

        // Track position
        let position = Position {
            pair: signal.pair.clone(),
            side: signal.action.clone(),
            size: actual_size,
            timestamp: chrono::Utc::now(),
            entry_price: signal.price,
            order_id: order_id.clone(),
        };

        let mut tracker = self.position_tracker.write().await;
        tracker.insert(order_id.clone(), position);

        Ok(TradeResult {
            status: "success".to_string(),
            reason: "Order placed".to_string(),
            order_id: Some(order_id),
        })
    }

    /// Get account status
    pub async fn get_account_status(&self) -> Result<AccountStatus> {
        let mut client = self.mcp_client.write().await;
        
        let balance = client.get_balance().await?;
        let open_orders = client.get_open_orders().await?;
        
        let tracker = self.position_tracker.read().await;
        let positions = tracker.values().cloned().collect();

        Ok(AccountStatus {
            balance,
            open_orders,
            positions,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Cancel all open orders
    pub async fn cancel_all_orders(&self) -> Result<Vec<String>> {
        let mut client = self.mcp_client.write().await;
        let account_status = self.get_account_status().await?;
        
        let mut cancelled_orders = Vec::new();
        
        if let Some(orders) = account_status.open_orders.get("result") {
            if let Some(open_orders) = orders.get("open") {
                if let Some(orders_map) = open_orders.as_object() {
                    for (txid, _order_info) in orders_map {
                        match client.cancel_order(txid).await {
                            Ok(_) => {
                                cancelled_orders.push(txid.clone());
                                info!("Cancelled order: {}", txid);
                            }
                            Err(e) => {
                                warn!("Failed to cancel order {}: {}", txid, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(cancelled_orders)
    }
}

/// Market data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub pair: String,
    pub ticker: Value,
    pub orderbook: Value,
    pub ohlcv: Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Trading signal structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSignal {
    pub action: String,  // "buy" or "sell"
    pub pair: String,
    pub confidence: f64,
    pub position_size: f64,
    pub price: Option<f64>,
    pub reasoning: String,
}

/// Trade result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    pub status: String,
    pub reason: String,
    pub order_id: Option<String>,
}

/// Account status structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStatus {
    pub balance: Value,
    pub open_orders: Value,
    pub positions: Vec<Position>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Unified Kraken integration supporting both MCP and direct API
pub struct UnifiedKrakenIntegration {
    mcp_bot: Option<KrakenBot>,
    config: KrakenConfig,
}

impl UnifiedKrakenIntegration {
    /// Create new unified integration
    pub async fn new(config: KrakenConfig) -> Result<Self> {
        let mcp_bot = KrakenBot::new(config.clone()).await?;
        
        Ok(Self {
            mcp_bot: Some(mcp_bot),
            config,
        })
    }

    /// Get market data
    pub async fn get_market_data(&self, pair: &str) -> Result<MarketData> {
        if let Some(bot) = &self.mcp_bot {
            bot.get_market_data(pair).await
        } else {
            Err(anyhow::anyhow!("No integration method available"))
        }
    }

    /// Execute trade
    pub async fn execute_trade(&self, signal: TradingSignal, max_position_size: f64) -> Result<TradeResult> {
        if let Some(bot) = &self.mcp_bot {
            bot.execute_trade(signal, max_position_size).await
        } else {
            Err(anyhow::anyhow!("No integration method available"))
        }
    }

    /// Get account status
    pub async fn get_account_status(&self) -> Result<AccountStatus> {
        if let Some(bot) = &self.mcp_bot {
            bot.get_account_status().await
        } else {
            Err(anyhow::anyhow!("No integration method available"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_client_creation() {
        let config = KrakenConfig::default();
        let client = MCPClient::new(config);
        assert_eq!(client.next_request_id, 1);
    }

    #[tokio::test]
    async fn test_trading_signal_serialization() {
        let signal = TradingSignal {
            action: "buy".to_string(),
            pair: "BTCUSD".to_string(),
            confidence: 0.8,
            position_size: 0.1,
            price: Some(50000.0),
            reasoning: "RSI oversold".to_string(),
        };

        let json = serde_json::to_string(&signal).unwrap();
        let parsed: TradingSignal = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.action, "buy");
        assert_eq!(parsed.pair, "BTCUSD");
        assert_eq!(parsed.confidence, 0.8);
    }
}
