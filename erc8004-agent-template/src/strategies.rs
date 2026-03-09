//! Trading Strategies Implementation
//! 
//! This module provides various trading strategies that can be used with the
//! ERC-8004 AI trading agent, including mean reversion, momentum, and arbitrage.

use anyhow::Result;
use async_trait::async_trait;

use crate::agent::{Strategy, StrategyType, RiskParameters, TradingSignal};
use crate::kraken::{MarketData};
use crate::erc8004::{AgentProfile};

/// Mean reversion trading strategy
pub struct MeanReversionStrategy {
    name: String,
    risk_parameters: RiskParameters,
}

impl MeanReversionStrategy {
    pub fn new() -> Self {
        Self {
            name: "Mean Reversion".to_string(),
            risk_parameters: RiskParameters {
                max_position_size: 0.05,      // Conservative
                max_daily_loss: 50.0,
                confidence_threshold: 0.8,
                stop_loss_percentage: 0.03,
                take_profit_percentage: 0.06,
            },
        }
    }
}

#[async_trait]
impl Strategy for MeanReversionStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn strategy_type(&self) -> StrategyType {
        StrategyType::MeanReversion
    }

    async fn analyze(&self, market_data: &MarketData, _agent_profile: &AgentProfile) -> Result<TradingSignal> {
        // Extract price data from ticker
        let ticker_data = market_data.ticker.get("result")
            .and_then(|r| r.get(&market_data.pair))
            .ok_or_else(|| anyhow::anyhow!("No ticker data available"))?;

        let current_price = ticker_data.get("c")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|price| price.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| anyhow::anyhow!("No current price available"))?;

        let volume = ticker_data.get("v")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|vol| vol.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        // Extract OHLCV data for RSI calculation
        let ohlcv_data = market_data.ohlcv.get("result")
            .and_then(|r| r.get(&market_data.pair))
            .and_then(|data| data.as_array())
            .ok_or_else(|| anyhow::anyhow!("No OHLCV data available"))?;

        // Calculate RSI (simplified)
        let rsi = calculate_rsi(ohlcv_data, 14)?;

        // Generate signal based on RSI
        let (action, confidence, reasoning) = if rsi < 30.0 {
            // Oversold - buy signal
            ("buy", 0.85, format!("RSI oversold at {:.2}", rsi))
        } else if rsi > 70.0 {
            // Overbought - sell signal
            ("sell", 0.85, format!("RSI overbought at {:.2}", rsi))
        } else if rsi < 40.0 {
            // Approaching oversold - weak buy
            ("buy", 0.6, format!("RSI approaching oversold at {:.2}", rsi))
        } else if rsi > 60.0 {
            // Approaching overbought - weak sell
            ("sell", 0.6, format!("RSI approaching overbought at {:.2}", rsi))
        } else {
            // Neutral - no signal
            return Err(anyhow::anyhow!("No clear mean reversion signal"));
        };

        // Position sizing based on RSI extremity
        let position_size = match rsi {
            r if r < 20.0 || r > 80.0 => 0.03,  // Strong signal
            r if r < 30.0 || r > 70.0 => 0.02,  // Medium signal
            _ => 0.01,  // Weak signal
        };

        Ok(TradingSignal {
            action: action.to_string(),
            pair: market_data.pair.clone(),
            confidence,
            position_size,
            price: Some(current_price),
            reasoning,
        })
    }

    fn validate_signal(&self, signal: &TradingSignal) -> bool {
        signal.confidence >= self.risk_parameters.confidence_threshold &&
        signal.position_size <= self.risk_parameters.max_position_size
    }

    fn risk_parameters(&self) -> RiskParameters {
        self.risk_parameters.clone()
    }
}

/// Momentum trading strategy
pub struct MomentumStrategy {
    name: String,
    risk_parameters: RiskParameters,
}

impl MomentumStrategy {
    pub fn new() -> Self {
        Self {
            name: "Momentum".to_string(),
            risk_parameters: RiskParameters {
                max_position_size: 0.08,      // More aggressive
                max_daily_loss: 75.0,
                confidence_threshold: 0.7,
                stop_loss_percentage: 0.04,
                take_profit_percentage: 0.12,
            },
        }
    }
}

#[async_trait]
impl Strategy for MomentumStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn strategy_type(&self) -> StrategyType {
        StrategyType::TrendFollowing
    }

    async fn analyze(&self, market_data: &MarketData, _agent_profile: &AgentProfile) -> Result<TradingSignal> {
        // Extract price and volume data
        let ticker_data = market_data.ticker.get("result")
            .and_then(|r| r.get(&market_data.pair))
            .ok_or_else(|| anyhow::anyhow!("No ticker data available"))?;

        let current_price = ticker_data.get("c")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|price| price.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| anyhow::anyhow!("No current price available"))?;

        let price_change = ticker_data.get("p")
            .and_then(|p| p.as_array())
            .and_then(|arr| arr.first())
            .and_then(|change| change.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        let volume = ticker_data.get("v")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|vol| vol.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        // Calculate moving averages from OHLCV data
        let ohlcv_data = market_data.ohlcv.get("result")
            .and_then(|r| r.get(&market_data.pair))
            .and_then(|data| data.as_array())
            .ok_or_else(|| anyhow::anyhow!("No OHLCV data available"))?;

        let (sma_20, sma_50) = calculate_moving_averages(ohlcv_data)?;

        // Generate momentum signal
        let (action, confidence, reasoning) = if price_change > 2.0 && current_price > sma_20 && sma_20 > sma_50 {
            // Strong upward momentum
            ("buy", 0.9, format!("Strong momentum: +{:.2}% price, above moving averages", price_change))
        } else if price_change > 1.0 && current_price > sma_20 {
            // Moderate upward momentum
            ("buy", 0.75, format!("Moderate momentum: +{:.2}% price", price_change))
        } else if price_change < -2.0 && current_price < sma_20 && sma_20 < sma_50 {
            // Strong downward momentum
            ("sell", 0.9, format!("Strong downward momentum: {:.2}% price, below moving averages", price_change))
        } else if price_change < -1.0 && current_price < sma_20 {
            // Moderate downward momentum
            ("sell", 0.75, format!("Moderate downward momentum: {:.2}% price", price_change))
        } else {
            // No clear momentum
            return Err(anyhow::anyhow!("No clear momentum signal"));
        };

        // Position sizing based on momentum strength
        let position_size = match price_change.abs() {
            p if p > 3.0 => 0.06,  // Very strong momentum
            p if p > 2.0 => 0.04,  // Strong momentum
            p if p > 1.0 => 0.02,  // Moderate momentum
            _ => 0.01,  // Weak momentum
        };

        Ok(TradingSignal {
            action: action.to_string(),
            pair: market_data.pair.clone(),
            confidence,
            position_size,
            price: Some(current_price),
            reasoning,
        })
    }

    fn validate_signal(&self, signal: &TradingSignal) -> bool {
        signal.confidence >= self.risk_parameters.confidence_threshold &&
        signal.position_size <= self.risk_parameters.max_position_size
    }

    fn risk_parameters(&self) -> RiskParameters {
        self.risk_parameters.clone()
    }
}

/// Simple arbitrage strategy (placeholder)
pub struct ArbitrageStrategy {
    name: String,
    risk_parameters: RiskParameters,
}

impl ArbitrageStrategy {
    pub fn new() -> Self {
        Self {
            name: "Arbitrage".to_string(),
            risk_parameters: RiskParameters {
                max_position_size: 0.15,      // Higher for arbitrage
                max_daily_loss: 25.0,        // Lower loss tolerance
                confidence_threshold: 0.9,   // High confidence required
                stop_loss_percentage: 0.02,
                take_profit_percentage: 0.05,
            },
        }
    }
}

#[async_trait]
impl Strategy for ArbitrageStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn strategy_type(&self) -> StrategyType {
        StrategyType::Arbitrage
    }

    async fn analyze(&self, market_data: &MarketData, _agent_profile: &AgentProfile) -> Result<TradingSignal> {
        // This is a simplified arbitrage detection
        // In reality, you'd compare prices across different exchanges
        
        // For demo purposes, we'll look for order book imbalances
        let orderbook = market_data.orderbook.get("result")
            .and_then(|r| r.get(&market_data.pair))
            .ok_or_else(|| anyhow::anyhow!("No orderbook data available"))?;

        let bids = orderbook.get("bids")
            .and_then(|b| b.as_array())
            .ok_or_else(|| anyhow::anyhow!("No bids available"))?;

        let asks = orderbook.get("asks")
            .and_then(|a| a.as_array())
            .ok_or_else(|| anyhow::anyhow!("No asks available"))?;

        if bids.is_empty() || asks.is_empty() {
            return Err(anyhow::anyhow!("Empty order book"));
        }

        // Calculate spread
        let best_bid = bids[0].get(0)
            .and_then(|price| price.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| anyhow::anyhow!("Invalid best bid"))?;

        let best_ask = asks[0].get(0)
            .and_then(|price| price.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| anyhow::anyhow!("Invalid best ask"))?;

        let spread = best_ask - best_bid;
        let spread_percentage = (spread / best_bid) * 100.0;

        // Look for arbitrage opportunities (wide spreads)
        if spread_percentage > 0.1 {  // More than 0.1% spread
            let confidence = (spread_percentage / 0.5).min(0.95);  // Scale confidence with spread
            
            Ok(TradingSignal {
                action: "buy".to_string(),  // Buy at bid, sell at ask
                pair: market_data.pair.clone(),
                confidence,
                position_size: 0.02,
                price: Some(best_bid),
                reasoning: format!("Arbitrage opportunity: {:.3}% spread", spread_percentage),
            })
        } else {
            Err(anyhow::anyhow!("No arbitrage opportunity"))
        }
    }

    fn validate_signal(&self, signal: &TradingSignal) -> bool {
        signal.confidence >= self.risk_parameters.confidence_threshold &&
        signal.position_size <= self.risk_parameters.max_position_size
    }

    fn risk_parameters(&self) -> RiskParameters {
        self.risk_parameters.clone()
    }
}

/// Calculate RSI (Relative Strength Index)
fn calculate_rsi(ohlcv_data: &[serde_json::Value], period: usize) -> Result<f64> {
    if ohlcv_data.len() < period + 1 {
        return Ok(50.0);  // Default RSI when insufficient data
    }

    let mut gains = Vec::new();
    let mut losses = Vec::new();

    // Calculate price changes
    for i in 1..ohlcv_data.len().min(period + 1) {
        let prev_close = ohlcv_data[i - 1].get(4)  // Close price is at index 4
            .and_then(|c| c.as_f64())
            .unwrap_or(0.0);
        
        let current_close = ohlcv_data[i].get(4)
            .and_then(|c| c.as_f64())
            .unwrap_or(0.0);

        let change = current_close - prev_close;
        
        if change > 0.0 {
            gains.push(change);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-change);
        }
    }

    let avg_gain = gains.iter().sum::<f64>() / period as f64;
    let avg_loss = losses.iter().sum::<f64>() / period as f64;

    if avg_loss == 0.0 {
        return Ok(100.0);
    }

    let rs = avg_gain / avg_loss;
    let rsi = 100.0 - (100.0 / (1.0 + rs));

    Ok(rsi)
}

/// Calculate moving averages
fn calculate_moving_averages(ohlcv_data: &[serde_json::Value]) -> Result<(f64, f64)> {
    if ohlcv_data.len() < 50 {
        return Ok((0.0, 0.0));
    }

    let sma_20: f64 = ohlcv_data.iter()
        .rev()
        .take(20)
        .map(|candle| candle.get(4).and_then(|c| c.as_f64()).unwrap_or(0.0))
        .sum::<f64>() / 20.0;

    let sma_50: f64 = ohlcv_data.iter()
        .rev()
        .take(50)
        .map(|candle| candle.get(4).and_then(|c| c.as_f64()).unwrap_or(0.0))
        .sum::<f64>() / 50.0;

    Ok((sma_20, sma_50))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_mean_reversion_strategy() {
        let strategy = MeanReversionStrategy::new();
        assert_eq!(strategy.name(), "Mean Reversion");
        assert_eq!(strategy.strategy_type(), StrategyType::MeanReversion);
    }

    #[test]
    fn test_rsi_calculation() {
        // Create mock OHLCV data
        let ohlcv_data = vec![
            json!([1234567890, 100.0, 105.0, 95.0, 102.0, 1000.0]),  // [time, open, high, low, close, volume]
            json!([1234567891, 102.0, 108.0, 98.0, 105.0, 1100.0]),
            json!([1234567892, 105.0, 110.0, 100.0, 108.0, 1200.0]),
        ];

        let rsi = calculate_rsi(&ohlcv_data, 2).unwrap();
        assert!(rsi >= 0.0 && rsi <= 100.0);
    }

    #[test]
    fn test_moving_averages() {
        let ohlcv_data = vec![
            json!([1234567890, 100.0, 105.0, 95.0, 102.0, 1000.0]),
            json!([1234567891, 102.0, 108.0, 98.0, 105.0, 1100.0]),
            json!([1234567892, 105.0, 110.0, 100.0, 108.0, 1200.0]),
        ];

        let (sma_20, sma_50) = calculate_moving_averages(&ohlcv_data).unwrap();
        assert_eq!(sma_20, 105.0);  // Average of 3 prices
        assert_eq!(sma_50, 105.0);  // Same data, so same average
    }
}
