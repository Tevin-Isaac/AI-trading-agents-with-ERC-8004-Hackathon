"""
Kraken API Integration for AI Trading Agents
Now supports both direct API access and Kraken CLI MCP server integration
Recommended: Use MCP integration for better compatibility and features
"""

import asyncio
import json
import logging
import aiohttp
import websockets
from typing import Dict, List, Optional, Any
from dataclasses import dataclass
import hmac
import hashlib
import base64
import time
from urllib.parse import urlencode
from .mcp_client import MCPClient, MCPConfig, KrakenMCPBot

@dataclass
class KrakenConfig:
    api_key: str
    api_secret: str
    sandbox: bool = True
    timeout: int = 30
    
    @property
    def base_url(self) -> str:
        return "https://api.sandbox.kraken.com" if self.sandbox else "https://api.kraken.com"
    
    @property
    def ws_url(self) -> str:
        return "wss://ws.sandbox.kraken.com" if self.sandbox else "wss://ws.kraken.com"

class KrakenRESTClient:
    def __init__(self, config: KrakenConfig):
        self.config = config
        self.logger = logging.getLogger(__name__)
        self.session = None
    
    async def __aenter__(self):
        self.session = aiohttp.ClientSession(timeout=aiohttp.ClientTimeout(total=self.config.timeout))
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self.session:
            await self.session.close()
    
    def _sign_request(self, endpoint: str, data: Dict[str, Any]) -> str:
        """Sign API request with HMAC-SHA512"""
        nonce = str(int(time.time() * 1000))
        data['nonce'] = nonce
        
        # Create signature
        urlencoded = urlencode(data)
        message = (nonce + urlencoded).encode('utf-8')
        secret = base64.b64decode(self.config.api_secret)
        signature = hmac.new(secret, message, hashlib.sha512)
        signature_digest = base64.b64encode(signature.digest())
        
        return signature_digest.decode('utf-8')
    
    async def _make_request(self, method: str, endpoint: str, data: Optional[Dict] = None, private: bool = False) -> Dict:
        """Make HTTP request to Kraken API"""
        url = f"{self.config.base_url}{endpoint}"
        headers = {
            'API-Key': self.config.api_key,
            'Content-Type': 'application/x-www-form-urlencoded'
        }
        
        if private and data:
            headers['API-Sign'] = self._sign_request(endpoint, data or {})
        
        try:
            if method == 'GET':
                async with self.session.get(url, params=data, headers=headers) as response:
                    return await response.json()
            else:
                async with self.session.post(url, data=data, headers=headers) as response:
                    return await response.json()
        
        except Exception as e:
            self.logger.error(f"Kraken API request failed: {e}")
            raise
    
    async def get_ticker(self, pair: str) -> Dict:
        """Get ticker information for a trading pair"""
        return await self._make_request('GET', '/0/public/Ticker', {'pair': pair})
    
    async def get_ohlcv(self, pair: str, interval: int = 60, since: Optional[int] = None) -> Dict:
        """Get OHLCV data for analysis"""
        params = {'pair': pair, 'interval': interval}
        if since:
            params['since'] = since
        return await self._make_request('GET', '/0/public/OHLC', params)
    
    async def get_order_book(self, pair: str, count: int = 100) -> Dict:
        """Get order book for a trading pair"""
        return await self._make_request('GET', '/0/public/Depth', {'pair': pair, 'count': count})
    
    async def get_balance(self) -> Dict:
        """Get account balance"""
        return await self._make_request('POST', '/0/private/Balance', private=True)
    
    async def place_order(self, 
                         pair: str, 
                         order_type: str, 
                         side: str, 
                         volume: float,
                         price: Optional[float] = None,
                         leverage: Optional[float] = None) -> Dict:
        """Place a trading order"""
        data = {
            'pair': pair,
            'type': side,  # 'buy' or 'sell'
            'ordertype': order_type,  # 'market', 'limit', 'stop-loss', etc.
            'volume': str(volume)
        }
        
        if price:
            data['price'] = str(price)
        if leverage:
            data['leverage'] = str(leverage)
        
        return await self._make_request('POST', '/0/private/AddOrder', data, private=True)
    
    async def cancel_order(self, txid: str) -> Dict:
        """Cancel an open order"""
        return await self._make_request('POST', '/0/private/CancelOrder', {'txid': txid}, private=True)
    
    async def get_open_orders(self) -> Dict:
        """Get all open orders"""
        return await self._make_request('POST', '/0/private/OpenOrders', private=True)
    
    async def get_trade_history(self, start: Optional[str] = None, end: Optional[str] = None) -> Dict:
        """Get trade history"""
        data = {}
        if start:
            data['start'] = start
        if end:
            data['end'] = end
        return await self._make_request('POST', '/0/private/TradesHistory', data, private=True)

class KrakenWebSocketClient:
    def __init__(self, config: KrakenConfig):
        self.config = config
        self.logger = logging.getLogger(__name__)
        self.websocket = None
        self.subscriptions = {}
    
    async def connect(self):
        """Connect to Kraken WebSocket"""
        try:
            self.websocket = await websockets.connect(self.config.ws_url)
            self.logger.info("Connected to Kraken WebSocket")
        except Exception as e:
            self.logger.error(f"Failed to connect to Kraken WebSocket: {e}")
            raise
    
    async def subscribe_ticker(self, pairs: List[str]):
        """Subscribe to ticker updates"""
        subscribe_msg = {
            "event": "subscribe",
            "pair": pairs,
            "subscription": {
                "name": "ticker"
            }
        }
        
        await self.websocket.send(json.dumps(subscribe_msg))
        self.subscriptions['ticker'] = pairs
        self.logger.info(f"Subscribed to ticker for {pairs}")
    
    async def subscribe_book(self, pairs: List[str], depth: int = 100):
        """Subscribe to order book updates"""
        subscribe_msg = {
            "event": "subscribe",
            "pair": pairs,
            "subscription": {
                "name": "book",
                "depth": depth
            }
        }
        
        await self.websocket.send(json.dumps(subscribe_msg))
        self.subscriptions['book'] = pairs
        self.logger.info(f"Subscribed to order book for {pairs}")
    
    async def subscribe_trades(self, pairs: List[str]):
        """Subscribe to trade updates"""
        subscribe_msg = {
            "event": "subscribe",
            "pair": pairs,
            "subscription": {
                "name": "trade"
            }
        }
        
        await self.websocket.send(json.dumps(subscribe_msg))
        self.subscriptions['trades'] = pairs
        self.logger.info(f"Subscribed to trades for {pairs}")
    
    async def listen(self, callback):
        """Listen for WebSocket messages"""
        try:
            async for message in self.websocket:
                data = json.loads(message)
                await callback(data)
        except websockets.exceptions.ConnectionClosed:
            self.logger.warning("WebSocket connection closed")
        except Exception as e:
            self.logger.error(f"WebSocket error: {e}")
    
    async def close(self):
        """Close WebSocket connection"""
        if self.websocket:
            await self.websocket.close()
            self.logger.info("WebSocket connection closed")

class KrakenTradingBot:
    def __init__(self, config: KrakenConfig):
        self.config = config
        self.rest_client = KrakenRESTClient(config)
        self.ws_client = KrakenWebSocketClient(config)
        self.logger = logging.getLogger(__name__)
        self.position_tracker = {}
    
    async def initialize(self):
        """Initialize the trading bot"""
        await self.ws_client.connect()
        self.logger.info("Kraken trading bot initialized")
    
    async def get_market_data(self, pair: str) -> Dict:
        """Get comprehensive market data for analysis"""
        async with self.rest_client:
            # Get ticker data
            ticker = await self.rest_client.get_ticker(pair)
            
            # Get order book
            orderbook = await self.rest_client.get_order_book(pair)
            
            # Get recent OHLCV data
            ohlcv = await self.rest_client.get_ohlcv(pair, interval=60)
            
            return {
                'ticker': ticker,
                'orderbook': orderbook,
                'ohlcv': ohlcv,
                'timestamp': time.time()
            }
    
    async def execute_trade(self, 
                           signal: Dict, 
                           pair: str, 
                           max_position_size: float) -> Dict:
        """Execute a trade based on signal"""
        
        action = signal.get('action')
        confidence = signal.get('confidence', 0)
        position_size = signal.get('position_size', 0)
        
        if action not in ['buy', 'sell'] or confidence < 0.7:
            self.logger.info(f"Signal not actionable: {signal}")
            return {'status': 'rejected', 'reason': 'Low confidence or invalid action'}
        
        # Calculate actual position size
        actual_size = min(position_size * max_position_size, max_position_size)
        
        async with self.rest_client:
            try:
                # Place market order
                order_result = await self.rest_client.place_order(
                    pair=pair,
                    order_type='market',
                    side=action,
                    volume=actual_size
                )
                
                if order_result.get('error'):
                    self.logger.error(f"Order failed: {order_result['error']}")
                    return {'status': 'error', 'error': order_result['error']}
                
                # Track position
                order_data = order_result['result']['txid'][0]
                self.position_tracker[order_data] = {
                    'pair': pair,
                    'action': action,
                    'size': actual_size,
                    'timestamp': time.time(),
                    'signal': signal
                }
                
                self.logger.info(f"Order placed: {action} {actual_size} {pair}")
                return {'status': 'success', 'order_id': order_data}
                
            except Exception as e:
                self.logger.error(f"Trade execution failed: {e}")
                return {'status': 'error', 'error': str(e)}
    
    async def monitor_positions(self):
        """Monitor open positions and manage risk"""
        async with self.rest_client:
            try:
                open_orders = await self.rest_client.get_open_orders()
                
                # Check for risk management actions
                for order_id, position in self.position_tracker.items():
                    # Implement stop-loss, take-profit logic here
                    # This is a simplified example
                    pass
                
            except Exception as e:
                self.logger.error(f"Position monitoring failed: {e}")
    
    async def start_real_time_monitoring(self, pairs: List[str], callback):
        """Start real-time market monitoring"""
        await self.ws_client.subscribe_ticker(pairs)
        await self.ws_client.subscribe_book(pairs)
        
        async def message_handler(data):
            if isinstance(data, dict) and data.get('event') != 'heartbeat':
                await callback(data)
        
        await self.ws_client.listen(message_handler)
    
    async def shutdown(self):
        """Shutdown the trading bot"""
        await self.ws_client.close()
        self.logger.info("Kraken trading bot shutdown complete")

class UnifiedKrakenIntegration:
    """Unified Kraken integration supporting both MCP and direct API access"""
    
    def __init__(self, 
                 api_key: Optional[str] = None, 
                 api_secret: Optional[str] = None,
                 use_mcp: bool = True,
                 mcp_services: List[str] = None,
                 sandbox: bool = True):
        """
        Initialize Kraken integration
        
        Args:
            api_key: Kraken API key (required for direct API)
            api_secret: Kraken API secret (required for direct API)
            use_mcp: Whether to use MCP server (recommended)
            mcp_services: List of MCP services to enable
            sandbox: Use sandbox environment
        """
        self.use_mcp = use_mcp
        self.logger = logging.getLogger(__name__)
        
        if use_mcp:
            # MCP integration
            mcp_config = MCPConfig(
                services=mcp_services or ["market", "account", "paper"],
                allow_dangerous=False
            )
            self.mcp_bot = KrakenMCPBot(mcp_config)
            self.direct_client = None
        else:
            # Direct API integration
            if not api_key or not api_secret:
                raise ValueError("API key and secret required for direct API access")
            
            config = KrakenConfig(
                api_key=api_key,
                api_secret=api_secret,
                sandbox=sandbox
            )
            self.direct_client = KrakenTradingBot(config)
            self.mcp_bot = None
    
    async def initialize(self):
        """Initialize the chosen integration method"""
        if self.use_mcp:
            await self.mcp_bot.initialize()
        else:
            await self.direct_client.initialize()
    
    async def get_market_data(self, pair: str) -> Dict:
        """Get market data using chosen integration"""
        if self.use_mcp:
            return await self.mcp_bot.get_market_data(pair)
        else:
            return await self.direct_client.get_market_data(pair)
    
    async def execute_trade(self, signal: Dict, pair: str, max_position_size: float) -> Dict:
        """Execute trade using chosen integration"""
        if self.use_mcp:
            return await self.mcp_bot.execute_trade(signal, pair, max_position_size)
        else:
            return await self.direct_client.execute_trade(signal, pair, max_position_size)
    
    async def get_account_status(self) -> Dict:
        """Get account status using chosen integration"""
        if self.use_mcp:
            return await self.mcp_bot.get_account_status()
        else:
            # For direct API, we need to implement this
            return {"error": "Not implemented for direct API"}
    
    async def shutdown(self):
        """Shutdown the integration"""
        if self.use_mcp:
            await self.mcp_bot.shutdown()
        else:
            await self.direct_client.shutdown()

# Example usage
async def example_usage():
    """Example of using unified Kraken integration"""
    
    # Method 1: MCP Integration (Recommended)
    mcp_integration = UnifiedKrakenIntegration(use_mcp=True)
    await mcp_integration.initialize()
    
    # Get market data
    market_data = await mcp_integration.get_market_data("BTCUSD")
    print(f"MCP Market data: {market_data['ticker']}")
    
    # Execute trade
    signal = {
        'action': 'buy',
        'confidence': 0.8,
        'position_size': 0.5,
        'reasoning': 'Mean reversion signal'
    }
    
    result = await mcp_integration.execute_trade(signal, "BTCUSD", 0.1)
    print(f"MCP Trade result: {result}")
    
    await mcp_integration.shutdown()
    
    # Method 2: Direct API (Legacy)
    # direct_integration = UnifiedKrakenIntegration(
    #     api_key="your_api_key",
    #     api_secret="your_api_secret",
    #     use_mcp=False
    # )
    # await direct_integration.initialize()
    # ... use same interface

if __name__ == "__main__":
    asyncio.run(example_usage())
