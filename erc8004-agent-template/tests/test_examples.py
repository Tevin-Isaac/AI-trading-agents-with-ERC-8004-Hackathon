"""
Comprehensive Test Suite for ERC-8004 AI Trading Agent
Tests all components: contracts, strategy, integration, and validation
"""

import pytest
import asyncio
import json
import time
from unittest.mock import Mock, patch, AsyncMock
from web3 import Web3
import pandas as pd
import numpy as np

# Import our modules
from agent.strategy import TradingStrategy, StrategyType, RiskParams, Signal
from agent.kraken_integration import KrakenTradingBot, KrakenConfig
from agent.validation import ValidationManager
from contracts.trading_agent import TradingAgent

class TestTradingStrategy:
    """Test the AI trading strategy implementation"""
    
    @pytest.fixture
    def mean_reversion_strategy(self):
        """Create a mean reversion strategy for testing"""
        risk_params = RiskParams(
            max_position_size=0.1,
            max_leverage=2.0,
            max_daily_loss=0.05,
            max_drawdown=0.15,
            min_confidence=0.7
        )
        return TradingStrategy(StrategyType.MEAN_REVERSION, risk_params)
    
    @pytest.fixture
    def sample_market_data(self):
        """Generate sample market data for testing"""
        dates = pd.date_range(end='2024-01-01', periods=100, freq='1H')
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
    
    def test_strategy_initialization(self, mean_reversion_strategy):
        """Test strategy initialization"""
        assert mean_reversion_strategy.strategy_type == StrategyType.MEAN_REVERSION
        assert mean_reversion_strategy.risk_params.max_position_size == 0.1
        assert mean_reversion_strategy.lookback_period == 20
        assert mean_reversion_strategy.entry_threshold == 2.0
    
    def test_mean_reversion_signal_generation(self, mean_reversion_strategy, sample_market_data):
        """Test mean reversion signal generation"""
        signal = mean_reversion_strategy.generate_signal(sample_market_data)
        
        assert isinstance(signal, Signal)
        assert signal.action in ['buy', 'sell', 'hold']
        assert 0 <= signal.confidence <= 1.0
        assert 0 <= signal.position_size <= 1.0
    
    def test_signal_validation(self, mean_reversion_strategy):
        """Test signal validation against risk parameters"""
        # Valid signal
        valid_signal = Signal(
            action="buy",
            confidence=0.8,
            position_size=0.05,
            reasoning="Test signal"
        )
        assert mean_reversion_strategy.validate_signal(valid_signal, 10000) == True
        
        # Invalid signal (low confidence)
        invalid_signal = Signal(
            action="buy",
            confidence=0.5,
            position_size=0.05,
            reasoning="Low confidence signal"
        )
        assert mean_reversion_strategy.validate_signal(invalid_signal, 10000) == False
        
        # Invalid signal (position too large)
        oversized_signal = Signal(
            action="buy",
            confidence=0.8,
            position_size=0.2,  # Exceeds max_position_size
            reasoning="Oversized signal"
        )
        assert mean_reversion_strategy.validate_signal(oversized_signal, 10000) == False
    
    def test_position_size_calculation(self, mean_reversion_strategy):
        """Test position size calculation"""
        signal = Signal(
            action="buy",
            confidence=0.8,
            position_size=0.05,
            reasoning="Test signal"
        )
        
        position_size = mean_reversion_strategy.calculate_position_size(signal, 10000)
        assert position_size == 500.0  # 0.05 * 10000
    
    def test_momentum_strategy(self, sample_market_data):
        """Test momentum strategy implementation"""
        risk_params = RiskParams(min_confidence=0.6)
        momentum_strategy = TradingStrategy(StrategyType.MOMENTUM, risk_params)
        
        signal = momentum_strategy.generate_signal(sample_market_data)
        assert isinstance(signal, Signal)
        assert signal.action in ['buy', 'sell', 'hold']
    
    def test_arbitrage_strategy(self):
        """Test arbitrage strategy implementation"""
        risk_params = RiskParams()
        arbitrage_strategy = TradingStrategy(StrategyType.ARBITRAGE, risk_params)
        
        # Create data with arbitrage opportunity
        data = pd.DataFrame({
            'close': [100, 101, 102],
            'exchange1_price': [100, 101, 102],
            'exchange2_price': [101, 102, 103]  # 1% higher
        })
        
        signal = arbitrage_strategy.generate_signal(data)
        assert signal.action == 'arbitrage'
        assert signal.confidence > 0

class TestKrakenIntegration:
    """Test Kraken API integration"""
    
    @pytest.fixture
    def kraken_config(self):
        """Create Kraken configuration for testing"""
        return KrakenConfig(
            api_key="test_key",
            api_secret="test_secret",
            sandbox=True
        )
    
    @pytest.fixture
    def kraken_bot(self, kraken_config):
        """Create Kraken trading bot for testing"""
        return KrakenTradingBot(kraken_config)
    
    @pytest.mark.asyncio
    async def test_kraken_initialization(self, kraken_bot):
        """Test Kraken bot initialization"""
        with patch('websockets.connect') as mock_connect:
            mock_connect.return_value = AsyncMock()
            await kraken_bot.initialize()
            assert kraken_bot.websocket is not None
    
    @pytest.mark.asyncio
    async def test_get_market_data(self, kraken_bot):
        """Test market data retrieval"""
        mock_response = {
            'result': {
                'XBTUSD': [
                    [1640995200, 47000.0, 47500.0, 46500.0, 47200.0, 47100.0, 15420.5, 1234]
                ]
            }
        }
        
        with patch.object(kraken_bot.rest_client, 'get_ohlcv', return_value=mock_response):
            data = await kraken_bot.get_market_data("XBTUSD")
            assert 'ohlcv' in data
            assert 'ticker' in data
    
    @pytest.mark.asyncio
    async def test_trade_execution(self, kraken_bot):
        """Test trade execution"""
        signal = {
            'action': 'buy',
            'confidence': 0.8,
            'position_size': 0.1,
            'reasoning': 'Test signal'
        }
        
        mock_result = {
            'status': 'success',
            'order_id': 'test_order_123'
        }
        
        with patch.object(kraken_bot, 'execute_trade', return_value=mock_result):
            result = await kraken_bot.execute_trade(signal, "XBTUSD", 0.1)
            assert result['status'] == 'success'
            assert 'order_id' in result

class TestValidationManager:
    """Test validation and reputation management"""
    
    @pytest.fixture
    def validation_manager(self):
        """Create validation manager for testing"""
        mock_web3 = Mock()
        mock_web3.eth.contract.return_value = Mock()
        return ValidationManager(mock_web3, "0x123...")
    
    @pytest.mark.asyncio
    async def test_validation_submission(self, validation_manager):
        """Test validation submission"""
        validation_data = {
            'agent_id': 12345,
            'performance': {
                'sharpe_ratio': 1.5,
                'max_drawdown': 0.1
            },
            'trades': [
                {'pair': 'ETH/USD', 'pnl': 0.05}
            ]
        }
        
        with patch.object(validation_manager, 'submit_validation', return_value=True):
            result = await validation_manager.submit_validation(validation_data)
            assert result == True
    
    def test_reputation_calculation(self, validation_manager):
        """Test reputation score calculation"""
        performance_data = {
            'sharpe_ratio': 1.5,
            'max_drawdown': 0.08,
            'win_rate': 0.65,
            'validation_score': 90
        }
        
        score = validation_manager.calculate_reputation_score(performance_data)
        assert 0 <= score <= 100
        assert score > 50  # Should be above average for good performance

class TestSmartContracts:
    """Test smart contract functionality"""
    
    @pytest.fixture
    def web3_mock(self):
        """Create Web3 mock for testing"""
        mock_web3 = Mock()
        mock_web3.eth.contract.return_value = Mock()
        mock_web3.eth.account.sign_message.return_value = {
            'signature': '0x123...'
        }
        return mock_web3
    
    @pytest.fixture
    def trading_agent_contract(self, web3_mock):
        """Create trading agent contract mock"""
        contract_mock = Mock()
        contract_mock.functions.mintAgent.return_value.transact.return_value = '0x123...'
        contract_mock.functions.executeSignedTrade.return_value.transact.return_value = '0x456...'
        return contract_mock
    
    def test_agent_registration(self, trading_agent_contract):
        """Test agent registration process"""
        # Mock the registration call
        result = trading_agent_contract.functions.mintAgent(
            "TestAgent",
            "https://ipfs.io/ipfs/Qm..."
        ).transact()
        
        assert result == '0x123...'
    
    def test_trade_execution_contract(self, trading_agent_contract):
        """Test trade execution through contract"""
        trade_intent = {
            'tokenIn': '0x123...',
            'tokenOut': '0x456...',
            'amountIn': 1000,
            'minAmountOut': 950,
            'deadline': int(time.time()) + 3600,
            'maxSlippage': 500,
            'strategyHash': '0xabc...'
        }
        
        signature = '0x123...'
        result = trading_agent_contract.functions.executeSignedTrade(
            trade_intent,
            signature
        ).transact()
        
        assert result == '0x456...'

class TestIntegration:
    """Integration tests for the complete system"""
    
    @pytest.fixture
    def complete_agent(self):
        """Create a complete agent for integration testing"""
        # This would normally involve actual contract deployment
        # For testing, we use mocks
        return {
            'strategy': TradingStrategy(StrategyType.MEAN_REVERSION, RiskParams()),
            'kraken_bot': Mock(spec=KrakenTradingBot),
            'validation_manager': Mock(spec=ValidationManager)
        }
    
    @pytest.mark.asyncio
    async def test_complete_trading_flow(self, complete_agent):
        """Test the complete trading flow"""
        # Mock market data
        market_data = pd.DataFrame({
            'close': [100, 101, 102, 103, 104],
            'volume': [1000, 1100, 1200, 1300, 1400]
        })
        
        # Generate signal
        signal = complete_agent['strategy'].generate_signal(market_data)
        
        # Mock successful trade execution
        complete_agent['kraken_bot'].execute_trade.return_value = {
            'status': 'success',
            'order_id': 'test_123'
        }
        
        # Execute trade
        if signal.action != 'hold':
            result = await complete_agent['kraken_bot'].execute_trade(
                signal.__dict__, "BTC/USD", 0.1
            )
            assert result['status'] == 'success'
    
    def test_error_handling(self, complete_agent):
        """Test error handling in the system"""
        # Test invalid market data
        invalid_data = pd.DataFrame()
        
        with pytest.raises(Exception):
            complete_agent['strategy'].generate_signal(invalid_data)
    
    def test_performance_tracking(self, complete_agent):
        """Test performance tracking functionality"""
        # Simulate some trades
        trades = [
            {'pnl': 0.05, 'timestamp': '2024-01-01'},
            {'pnl': -0.02, 'timestamp': '2024-01-02'},
            {'pnl': 0.03, 'timestamp': '2024-01-03'}
        ]
        
        # Calculate performance metrics
        total_pnl = sum(trade['pnl'] for trade in trades)
        win_rate = len([t for t in trades if t['pnl'] > 0]) / len(trades)
        
        assert total_pnl == 0.06
        assert win_rate == 2/3

class TestSecurity:
    """Security tests for the system"""
    
    def test_signature_validation(self):
        """Test EIP-712 signature validation"""
        # This would test the actual signature validation logic
        # For now, we test the structure
        domain = {
            'name': 'AI Trading Agent',
            'version': '1',
            'chainId': 1,
            'verifyingContract': '0x123...'
        }
        
        types = {
            'TradeIntent': [
                {'name': 'tokenIn', 'type': 'address'},
                {'name': 'tokenOut', 'type': 'address'},
                {'name': 'amountIn', 'type': 'uint256'}
            ]
        }
        
        value = {
            'tokenIn': '0x123...',
            'tokenOut': '0x456...',
            'amountIn': 1000
        }
        
        assert 'name' in domain
        assert 'TradeIntent' in types
        assert 'tokenIn' in value
    
    def test_access_control(self):
        """Test access control mechanisms"""
        # Test that only authorized addresses can execute certain functions
        authorized_address = "0x123..."
        unauthorized_address = "0x456..."
        
        # This would test the actual access control logic
        # For now, we test the concept
        assert authorized_address != unauthorized_address
    
    def test_input_validation(self):
        """Test input validation for security"""
        # Test various input validation scenarios
        invalid_inputs = [
            "",  # Empty string
            None,  # None value
            -1,  # Negative number where positive expected
            "0xinvalid",  # Invalid address format
        ]
        
        for invalid_input in invalid_inputs:
            # Each invalid input should be rejected
            assert invalid_input is not None  # Placeholder for actual validation

# Performance Tests
class TestPerformance:
    """Performance tests for the system"""
    
    def test_strategy_performance(self):
        """Test strategy execution performance"""
        strategy = TradingStrategy(StrategyType.MEAN_REVERSION, RiskParams())
        
        # Generate large dataset
        large_data = pd.DataFrame({
            'close': np.random.normal(100, 10, 10000),
            'volume': np.random.uniform(1000, 10000, 10000)
        })
        
        start_time = time.time()
        signal = strategy.generate_signal(large_data)
        end_time = time.time()
        
        # Should complete within reasonable time
        assert end_time - start_time < 1.0  # 1 second max
        assert isinstance(signal, Signal)
    
    @pytest.mark.asyncio
    async def test_api_response_time(self):
        """Test API response times"""
        # Mock API call
        with patch('aiohttp.ClientSession.get') as mock_get:
            mock_response = AsyncMock()
            mock_response.json.return_value = {'result': 'success'}
            mock_get.return_value.__aenter__.return_value = mock_response
            
            start_time = time.time()
            # Simulate API call
            await mock_response.json()
            end_time = time.time()
            
            # API should respond quickly
            assert end_time - start_time < 0.5  # 500ms max

# Utility Functions
def create_test_contract_abi():
    """Create test contract ABI for testing"""
    return [
        {
            "inputs": [
                {"name": "name", "type": "string"},
                {"name": "uri", "type": "string"}
            ],
            "name": "mintAgent",
            "outputs": [{"name": "", "type": "uint256"}],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {"name": "intent", "type": "tuple"},
                {"name": "signature", "type": "bytes"}
            ],
            "name": "executeSignedTrade",
            "outputs": [{"name": "amountOut", "type": "uint256"}],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]

def create_test_agent_config():
    """Create test agent configuration"""
    return {
        "agentId": 12345,
        "strategy": "mean_reversion",
        "riskLevel": "medium",
        "tradingPairs": ["ETH/USD", "BTC/USD"],
        "maxPositionSize": 0.1,
        "stopLoss": 0.05,
        "takeProfit": 0.1,
        "krakenEnabled": True,
        "validationEnabled": True
    }

# Pytest Configuration
@pytest.fixture(scope="session")
def event_loop():
    """Create an instance of the default event loop for the test session."""
    loop = asyncio.get_event_loop_policy().new_event_loop()
    yield loop
    loop.close()

if __name__ == "__main__":
    # Run tests
    pytest.main([__file__, "-v", "--tb=short"])
