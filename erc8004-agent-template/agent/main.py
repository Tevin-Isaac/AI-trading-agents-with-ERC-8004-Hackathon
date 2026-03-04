"""
Main AI Trading Agent Application
Integrates strategy, validation, and Kraken API
"""

import asyncio
import json
import logging
import os
import sys
from pathlib import Path
from typing import Dict, Any, Optional
import argparse
from datetime import datetime

# Add parent directory to path for imports
sys.path.append(str(Path(__file__).parent.parent))

from web3 import Web3
from web3.middleware import geth_poa_middleware
import pandas as pd
import numpy as np

from strategy import TradingStrategy, StrategyType, RiskParams, Signal
from kraken_integration import KrakenTradingBot, KrakenConfig
from validation import ValidationManager

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class AITradingAgent:
    def __init__(self, config_path: str):
        self.config = self._load_config(config_path)
        self.web3 = self._setup_web3()
        self.strategy = None
        self.kraken_bot = None
        self.validation_manager = None
        self.is_running = False
        
        # Agent state
        self.current_capital = self.config['initial_capital']
        self.positions = {}
        self.trade_history = []
        self.performance_metrics = {
            'total_pnl': 0.0,
            'win_rate': 0.0,
            'sharpe_ratio': 0.0,
            'max_drawdown': 0.0,
            'total_trades': 0
        }
    
    def _load_config(self, config_path: str) -> Dict:
        """Load configuration from file"""
        with open(config_path, 'r') as f:
            config = json.load(f)
        
        # Set default values
        defaults = {
            'initial_capital': 10000.0,
            'strategy_type': 'mean_reversion',
            'risk_level': 'medium',
            'kraken_enabled': False,
            'validation_enabled': True,
            'network': 'sepolia'
        }
        
        for key, value in defaults.items():
            if key not in config:
                config[key] = value
        
        return config
    
    def _setup_web3(self) -> Web3:
        """Setup Web3 connection"""
        network = self.config['network']
        
        if network == 'sepolia':
            w3 = Web3(Web3.HTTPProvider('https://sepolia.infura.io/v3/YOUR_PROJECT_ID'))
        elif network == 'mainnet':
            w3 = Web3(Web3.HTTPProvider('https://mainnet.infura.io/v3/YOUR_PROJECT_ID'))
        else:
            w3 = Web3(Web3.HTTPProvider(f'http://localhost:8545'))
        
        # Add POA middleware if needed
        if network in ['sepolia', 'goerli']:
            w3.middleware_onion.inject(geth_poa_middleware, layer=0)
        
        return w3
    
    async def initialize(self):
        """Initialize all components"""
        logger.info("Initializing AI Trading Agent...")
        
        # Setup strategy
        strategy_type = StrategyType(self.config['strategy_type'])
        risk_params = self._get_risk_params()
        self.strategy = TradingStrategy(strategy_type, risk_params)
        
        # Setup Kraken integration if enabled
        if self.config['kraken_enabled']:
            kraken_config = KrakenConfig(
                api_key=os.getenv('KRAKEN_API_KEY'),
                api_secret=os.getenv('KRAKEN_API_SECRET'),
                sandbox=self.config.get('sandbox', True)
            )
            self.kraken_bot = KrakenTradingBot(kraken_config)
            await self.kraken_bot.initialize()
            logger.info("Kraken integration enabled")
        
        # Setup validation manager
        if self.config['validation_enabled']:
            self.validation_manager = ValidationManager(
                web3=self.web3,
                contract_address=self.config['validation_registry_address']
            )
            logger.info("Validation manager enabled")
        
        logger.info("AI Trading Agent initialized successfully")
    
    def _get_risk_params(self) -> RiskParams:
        """Get risk parameters based on risk level"""
        risk_level = self.config['risk_level']
        
        if risk_level == 'low':
            return RiskParams(
                max_position_size=0.05,
                max_leverage=1.0,
                max_daily_loss=0.02,
                max_drawdown=0.10,
                min_confidence=0.8
            )
        elif risk_level == 'high':
            return RiskParams(
                max_position_size=0.2,
                max_leverage=3.0,
                max_daily_loss=0.08,
                max_drawdown=0.25,
                min_confidence=0.6
            )
        else:  # medium
            return RiskParams(
                max_position_size=0.1,
                max_leverage=2.0,
                max_daily_loss=0.05,
                max_drawdown=0.15,
                min_confidence=0.7
            )
    
    async def get_market_data(self, pair: str) -> pd.DataFrame:
        """Get market data for analysis"""
        if self.kraken_bot:
            market_data = await self.kraken_bot.get_market_data(pair)
            
            # Convert to DataFrame for strategy analysis
            ohlcv_data = market_data['ohlcv']['result'].get(pair, [])
            if ohlcv_data:
                df = pd.DataFrame(ohlcv_data, columns=[
                    'timestamp', 'open', 'high', 'low', 'close', 'vwap', 'volume', 'count'
                ])
                df['timestamp'] = pd.to_datetime(df['timestamp'], unit='s')
                df.set_index('timestamp', inplace=True)
                
                # Add Kraken order book data if available
                if 'orderbook' in market_data:
                    orderbook = market_data['orderbook']['result'].get(pair, {})
                    if orderbook:
                        df['bid'] = float(orderbook['bids'][0][0]) if orderbook['bids'] else df['close']
                        df['ask'] = float(orderbook['asks'][0][0]) if orderbook['asks'] else df['close']
                        df['spread'] = df['ask'] - df['bid']
                
                return df
        
        # Fallback to mock data for testing
        return self._generate_mock_data()
    
    def _generate_mock_data(self) -> pd.DataFrame:
        """Generate mock market data for testing"""
        dates = pd.date_range(end=datetime.now(), periods=100, freq='1H')
        np.random.seed(42)
        
        # Generate realistic price movements
        returns = np.random.normal(0, 0.02, len(dates))
        prices = [100]
        for ret in returns[1:]:
            prices.append(prices[-1] * (1 + ret))
        
        data = {
            'timestamp': dates,
            'open': prices,
            'high': [p * (1 + abs(np.random.normal(0, 0.01))) for p in prices],
            'low': [p * (1 - abs(np.random.normal(0, 0.01))) for p in prices],
            'close': prices,
            'volume': np.random.uniform(100, 1000, len(dates))
        }
        
        df = pd.DataFrame(data)
        df.set_index('timestamp', inplace=True)
        return df
    
    async def analyze_market(self, pair: str) -> Signal:
        """Analyze market and generate trading signal"""
        logger.info(f"Analyzing market for {pair}...")
        
        # Get market data
        market_data = await self.get_market_data(pair)
        
        # Generate signal
        signal = self.strategy.generate_signal(market_data)
        
        # Validate signal
        if self.strategy.validate_signal(signal, self.current_capital):
            logger.info(f"Signal generated: {signal.action} with confidence {signal.confidence:.2f}")
            return signal
        else:
            logger.info("Signal rejected by risk management")
            return Signal("hold", 0.0, 0.0, reasoning="Risk management rejection")
    
    async def execute_trade(self, signal: Signal, pair: str) -> Dict:
        """Execute trading signal"""
        if signal.action == 'hold':
            return {'status': 'no_action', 'reason': 'Hold signal'}
        
        logger.info(f"Executing {signal.action} trade for {pair}")
        
        # Calculate position size
        position_size = self.strategy.calculate_position_size(signal, self.current_capital)
        
        # Execute trade
        if self.kraken_bot:
            result = await self.kraken_bot.execute_trade(
                signal.__dict__, pair, position_size
            )
        else:
            # Mock execution for testing
            result = {
                'status': 'success',
                'order_id': f"mock_{datetime.now().timestamp()}",
                'executed_price': 100.0,  # Mock price
                'executed_size': position_size
            }
        
        # Record trade
        if result['status'] == 'success':
            self._record_trade(signal, pair, result)
        
        return result
    
    def _record_trade(self, signal: Signal, pair: str, result: Dict):
        """Record trade in history"""
        trade = {
            'timestamp': datetime.now().isoformat(),
            'pair': pair,
            'action': signal.action,
            'signal_confidence': signal.confidence,
            'position_size': result.get('executed_size', 0),
            'order_id': result.get('order_id'),
            'status': result['status']
        }
        
        self.trade_history.append(trade)
        self.performance_metrics['total_trades'] += 1
        
        logger.info(f"Trade recorded: {trade}")
    
    async def validate_performance(self):
        """Validate performance and update reputation"""
        if not self.validation_manager:
            return
        
        # Calculate performance metrics
        if len(self.trade_history) > 0:
            # Update performance metrics
            self._calculate_performance_metrics()
            
            # Submit validation request
            validation_data = {
                'agent_id': self.config['agent_id'],
                'performance': self.performance_metrics,
                'trades': self.trade_history[-10:],  # Last 10 trades
                'timestamp': datetime.now().isoformat()
            }
            
            await self.validation_manager.submit_validation(validation_data)
            logger.info("Performance validation submitted")
    
    def _calculate_performance_metrics(self):
        """Calculate performance metrics"""
        if not self.trade_history:
            return
        
        # Simple PnL calculation (mock for now)
        winning_trades = len([t for t in self.trade_history if t.get('pnl', 0) > 0])
        self.performance_metrics['win_rate'] = winning_trades / len(self.trade_history)
        
        # Update other metrics
        self.performance_metrics['total_trades'] = len(self.trade_history)
        # Add more sophisticated calculations here
    
    async def run_trading_loop(self):
        """Main trading loop"""
        self.is_running = True
        pairs = self.config.get('trading_pairs', ['BTC/USD', 'ETH/USD'])
        
        logger.info(f"Starting trading loop for pairs: {pairs}")
        
        while self.is_running:
            try:
                for pair in pairs:
                    # Analyze market
                    signal = await self.analyze_market(pair)
                    
                    # Execute trade if signal is valid
                    if signal.action != 'hold':
                        result = await self.execute_trade(signal, pair)
                        logger.info(f"Trade execution result: {result}")
                
                # Validate performance periodically
                if len(self.trade_history) % 5 == 0:  # Every 5 trades
                    await self.validate_performance()
                
                # Wait before next iteration
                await asyncio.sleep(self.config.get('analysis_interval', 60))
                
            except KeyboardInterrupt:
                logger.info("Trading loop interrupted by user")
                break
            except Exception as e:
                logger.error(f"Error in trading loop: {e}")
                await asyncio.sleep(10)  # Wait before retrying
    
    async def shutdown(self):
        """Shutdown the agent"""
        logger.info("Shutting down AI Trading Agent...")
        self.is_running = False
        
        if self.kraken_bot:
            await self.kraken_bot.shutdown()
        
        logger.info("AI Trading Agent shutdown complete")
    
    def get_status(self) -> Dict:
        """Get current agent status"""
        return {
            'is_running': self.is_running,
            'current_capital': self.current_capital,
            'total_trades': len(self.trade_history),
            'performance_metrics': self.performance_metrics,
            'strategy_type': self.config['strategy_type'],
            'kraken_enabled': self.config['kraken_enabled']
        }

async def main():
    parser = argparse.ArgumentParser(description='AI Trading Agent')
    parser.add_argument('--config', type=str, default='config.json', help='Configuration file path')
    parser.add_argument('--network', type=str, default='sepolia', help='Blockchain network')
    parser.add_argument('--strategy', type=str, default='mean_reversion', help='Trading strategy')
    parser.add_argument('--risk-level', type=str, default='medium', help='Risk level')
    parser.add_argument('--kraken-enabled', action='store_true', help='Enable Kraken integration')
    
    args = parser.parse_args()
    
    # Create configuration
    config = {
        'network': args.network,
        'strategy_type': args.strategy,
        'risk_level': args.risk_level,
        'kraken_enabled': args.kraken_enabled,
        'trading_pairs': ['BTC/USD', 'ETH/USD'],
        'analysis_interval': 60,
        'initial_capital': 10000.0
    }
    
    # Save configuration
    with open(args.config, 'w') as f:
        json.dump(config, f, indent=2)
    
    # Initialize and run agent
    agent = AITradingAgent(args.config)
    
    try:
        await agent.initialize()
        await agent.run_trading_loop()
    except KeyboardInterrupt:
        logger.info("Shutting down...")
    finally:
        await agent.shutdown()

if __name__ == "__main__":
    asyncio.run(main())
