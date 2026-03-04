"""
AI Trading Strategy Implementation
Supports multiple strategies and risk management
"""

import numpy as np
import pandas as pd
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass
from enum import Enum
import logging

class StrategyType(Enum):
    MEAN_REVERSION = "mean_reversion"
    MOMENTUM = "momentum"
    ARBITRAGE = "arbitrage"
    GRID_TRADING = "grid_trading"

@dataclass
class Signal:
    action: str  # 'buy', 'sell', 'hold'
    confidence: float  # 0.0 to 1.0
    position_size: float  # 0.0 to 1.0 (fraction of capital)
    stop_loss: Optional[float] = None
    take_profit: Optional[float] = None
    reasoning: str = ""

@dataclass
class RiskParams:
    max_position_size: float = 0.1  # 10% of capital
    max_leverage: float = 2.0
    max_daily_loss: float = 0.05  # 5% daily loss limit
    max_drawdown: float = 0.15  # 15% max drawdown
    min_confidence: float = 0.7

class TradingStrategy:
    def __init__(self, strategy_type: StrategyType, risk_params: RiskParams):
        self.strategy_type = strategy_type
        self.risk_params = risk_params
        self.logger = logging.getLogger(__name__)
        
        # Strategy-specific parameters
        if strategy_type == StrategyType.MEAN_REVERSION:
            self.lookback_period = 20
            self.entry_threshold = 2.0  # Standard deviations
        elif strategy_type == StrategyType.MOMENTUM:
            self.momentum_period = 14
            self.rsi_period = 14
        elif strategy_type == StrategyType.ARBITRAGE:
            self.min_profit_threshold = 0.005  # 0.5%
        elif strategy_type == StrategyType.GRID_TRADING:
            self.grid_size = 0.01  # 1% grid
            self.grid_levels = 10
    
    def generate_signal(self, market_data: pd.DataFrame) -> Signal:
        """Generate trading signal based on strategy type"""
        
        if self.strategy_type == StrategyType.MEAN_REVERSION:
            return self._mean_reversion_signal(market_data)
        elif self.strategy_type == StrategyType.MOMENTUM:
            return self._momentum_signal(market_data)
        elif self.strategy_type == StrategyType.ARBITRAGE:
            return self._arbitrage_signal(market_data)
        elif self.strategy_type == StrategyType.GRID_TRADING:
            return self._grid_trading_signal(market_data)
        else:
            return Signal("hold", 0.0, 0.0, reasoning="Unknown strategy")
    
    def _mean_reversion_signal(self, data: pd.DataFrame) -> Signal:
        """Mean reversion strategy using Bollinger Bands"""
        
        # Calculate moving average and standard deviation
        data['sma'] = data['close'].rolling(window=self.lookback_period).mean()
        data['std'] = data['close'].rolling(window=self.lookback_period).std()
        data['upper_band'] = data['sma'] + (data['std'] * self.entry_threshold)
        data['lower_band'] = data['sma'] - (data['std'] * self.entry_threshold)
        
        current_price = data['close'].iloc[-1]
        upper_band = data['upper_band'].iloc[-1]
        lower_band = data['lower_band'].iloc[-1]
        sma = data['sma'].iloc[-1]
        
        # Generate signal
        if current_price <= lower_band:
            # Price is oversold - buy signal
            confidence = min((lower_band - current_price) / lower_band, 1.0)
            position_size = confidence * self.risk_params.max_position_size
            
            return Signal(
                action="buy",
                confidence=confidence,
                position_size=position_size,
                stop_loss=current_price * 0.95,  # 5% stop loss
                take_profit=sma,
                reasoning=f"Price {current_price} below lower band {lower_band}"
            )
        
        elif current_price >= upper_band:
            # Price is overbought - sell signal
            confidence = min((current_price - upper_band) / upper_band, 1.0)
            position_size = confidence * self.risk_params.max_position_size
            
            return Signal(
                action="sell",
                confidence=confidence,
                position_size=position_size,
                stop_loss=current_price * 1.05,  # 5% stop loss
                take_profit=sma,
                reasoning=f"Price {current_price} above upper band {upper_band}"
            )
        
        else:
            return Signal("hold", 0.0, 0.0, reasoning="Price within bands")
    
    def _momentum_signal(self, data: pd.DataFrame) -> Signal:
        """Momentum strategy using RSI and price momentum"""
        
        # Calculate RSI
        delta = data['close'].diff()
        gain = (delta.where(delta > 0, 0)).rolling(window=self.rsi_period).mean()
        loss = (-delta.where(delta < 0, 0)).rolling(window=self.rsi_period).mean()
        rs = gain / loss
        data['rsi'] = 100 - (100 / (1 + rs))
        
        # Calculate momentum
        data['momentum'] = data['close'].pct_change(periods=self.momentum_period)
        
        current_rsi = data['rsi'].iloc[-1]
        current_momentum = data['momentum'].iloc[-1]
        
        # Generate signal
        if current_rsi < 30 and current_momentum > 0:
            # Oversold with positive momentum - buy
            confidence = min((30 - current_rsi) / 30, 1.0)
            position_size = confidence * self.risk_params.max_position_size
            
            return Signal(
                action="buy",
                confidence=confidence,
                position_size=position_size,
                reasoning=f"RSI {current_rsi:.1f} oversold, momentum {current_momentum:.3f}"
            )
        
        elif current_rsi > 70 and current_momentum < 0:
            # Overbought with negative momentum - sell
            confidence = min((current_rsi - 70) / 30, 1.0)
            position_size = confidence * self.risk_params.max_position_size
            
            return Signal(
                action="sell",
                confidence=confidence,
                position_size=position_size,
                reasoning=f"RSI {current_rsi:.1f} overbought, momentum {current_momentum:.3f}"
            )
        
        else:
            return Signal("hold", 0.0, 0.0, reasoning=f"RSI {current_rsi:.1f}, momentum neutral")
    
    def _arbitrage_signal(self, data: pd.DataFrame) -> Signal:
        """Arbitrage strategy between exchanges"""
        
        if 'exchange1_price' not in data.columns or 'exchange2_price' not in data.columns:
            return Signal("hold", 0.0, 0.0, reasoning="No arbitrage data available")
        
        price_diff = abs(data['exchange1_price'].iloc[-1] - data['exchange2_price'].iloc[-1])
        avg_price = (data['exchange1_price'].iloc[-1] + data['exchange2_price'].iloc[-1]) / 2
        profit_potential = price_diff / avg_price
        
        if profit_potential > self.min_profit_threshold:
            confidence = min(profit_potential / (self.min_profit_threshold * 2), 1.0)
            
            # Determine which exchange to buy/sell on
            if data['exchange1_price'].iloc[-1] < data['exchange2_price'].iloc[-1]:
                reasoning = f"Buy on exchange1, sell on exchange2 - {profit_potential:.3f} profit"
            else:
                reasoning = f"Buy on exchange2, sell on exchange1 - {profit_potential:.3f} profit"
            
            return Signal(
                action="arbitrage",
                confidence=confidence,
                position_size=self.risk_params.max_position_size,
                reasoning=reasoning
            )
        
        return Signal("hold", 0.0, 0.0, reasoning=f"Profit potential {profit_potential:.3f} below threshold")
    
    def _grid_trading_signal(self, data: pd.DataFrame) -> Signal:
        """Grid trading strategy"""
        
        current_price = data['close'].iloc[-1]
        
        # Simple grid implementation - in production, this would track open positions
        grid_levels = []
        for i in range(-self.grid_levels, self.grid_levels + 1):
            grid_levels.append(current_price * (1 + i * self.grid_size))
        
        # Find nearest grid levels
        lower_grid = max([level for level in grid_levels if level < current_price], default=0)
        upper_grid = min([level for level in grid_levels if level > current_price], default=float('inf'))
        
        # Generate signal based on position in grid
        if current_price <= lower_grid * 1.001:  # Near lower grid
            return Signal(
                action="buy",
                confidence=0.8,
                position_size=self.risk_params.max_position_size / self.grid_levels,
                reasoning=f"Price {current_price} near lower grid {lower_grid}"
            )
        elif current_price >= upper_grid * 0.999:  # Near upper grid
            return Signal(
                action="sell",
                confidence=0.8,
                position_size=self.risk_params.max_position_size / self.grid_levels,
                reasoning=f"Price {current_price} near upper grid {upper_grid}"
            )
        
        return Signal("hold", 0.0, 0.0, reasoning="Price in middle of grid")
    
    def validate_signal(self, signal: Signal, current_capital: float) -> bool:
        """Validate signal against risk parameters"""
        
        # Check confidence threshold
        if signal.confidence < self.risk_params.min_confidence:
            self.logger.info(f"Signal rejected: confidence {signal.confidence} below threshold {self.risk_params.min_confidence}")
            return False
        
        # Check position size
        if signal.position_size > self.risk_params.max_position_size:
            self.logger.info(f"Signal rejected: position size {signal.position_size} exceeds maximum {self.risk_params.max_position_size}")
            return False
        
        # Additional risk checks can be added here
        return True
    
    def calculate_position_size(self, signal: Signal, current_capital: float) -> float:
        """Calculate actual position size based on signal and risk parameters"""
        
        base_size = current_capital * signal.position_size
        
        # Apply leverage if configured
        if self.risk_params.max_leverage > 1.0:
            base_size *= min(self.risk_params.max_leverage, 2.0)  # Cap at 2x for safety
        
        return base_size
