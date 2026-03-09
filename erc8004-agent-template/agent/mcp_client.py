"""
MCP Client for Kraken CLI Integration
Model Context Protocol client to communicate with Kraken CLI's built-in MCP server
"""

import asyncio
import json
import logging
import subprocess
from typing import Dict, List, Optional, Any
from dataclasses import dataclass
import sys
from pathlib import Path

@dataclass
class MCPConfig:
    """Configuration for MCP server connection"""
    kraken_cli_path: str = "kraken"  # Assumes kraken-cli is in PATH
    services: List[str] = None  # Services to enable: market, account, trade, paper
    allow_dangerous: bool = False  # Allow dangerous operations
    timeout: int = 30
    
    def __post_init__(self):
        if self.services is None:
            self.services = ["market", "account", "paper"]  # Safe defaults

class MCPClient:
    """Model Context Protocol client for Kraken CLI"""
    
    def __init__(self, config: MCPConfig):
        self.config = config
        self.logger = logging.getLogger(__name__)
        self.process = None
        self.server_capabilities = {}
        
    async def start_server(self):
        """Start Kraken CLI MCP server"""
        try:
            # Build command arguments
            cmd = [self.config.kraken_cli_path, "mcp", "-s", ",".join(self.config.services)]
            
            if self.config.allow_dangerous:
                cmd.append("--allow-dangerous")
            
            self.logger.info(f"Starting Kraken MCP server: {' '.join(cmd)}")
            
            # Start subprocess
            self.process = await asyncio.create_subprocess_exec(
                *cmd,
                stdin=asyncio.subprocess.PIPE,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE
            )
            
            # Initialize MCP session
            await self._initialize_session()
            
        except Exception as e:
            self.logger.error(f"Failed to start MCP server: {e}")
            raise
    
    async def _initialize_session(self):
        """Initialize MCP session with server"""
        # Send initialize request
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "clientInfo": {
                    "name": "ERC-8004 AI Trading Agent",
                    "version": "1.0.0"
                }
            }
        }
        
        response = await self._send_request(init_request)
        
        if "error" in response:
            raise Exception(f"MCP initialization failed: {response['error']}")
        
        self.server_capabilities = response.get("result", {}).get("capabilities", {})
        self.logger.info("MCP session initialized successfully")
        
        # Send initialized notification
        await self._send_notification({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        })
    
    async def _send_request(self, request: Dict) -> Dict:
        """Send JSON-RPC request and wait for response"""
        if not self.process or not self.process.stdin:
            raise Exception("MCP server not connected")
        
        # Send request
        message = json.dumps(request) + "\n"
        self.process.stdin.write(message.encode())
        await self.process.stdin.drain()
        
        # Read response
        response_line = await self.process.stdout.readline()
        if not response_line:
            raise Exception("No response from MCP server")
        
        try:
            response = json.loads(response_line.decode().strip())
            return response
        except json.JSONDecodeError as e:
            raise Exception(f"Invalid JSON response: {e}")
    
    async def _send_notification(self, notification: Dict):
        """Send JSON-RPC notification (no response expected)"""
        if not self.process or not self.process.stdin:
            raise Exception("MCP server not connected")
        
        message = json.dumps(notification) + "\n"
        self.process.stdin.write(message.encode())
        await self.process.stdin.drain()
    
    async def list_tools(self) -> List[Dict]:
        """List available tools from Kraken CLI"""
        request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        }
        
        response = await self._send_request(request)
        
        if "error" in response:
            raise Exception(f"Failed to list tools: {response['error']}")
        
        return response.get("result", {}).get("tools", [])
    
    async def call_tool(self, tool_name: str, arguments: Dict = None) -> Dict:
        """Call a specific tool on Kraken CLI"""
        if arguments is None:
            arguments = {}
        
        request = {
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            }
        }
        
        response = await self._send_request(request)
        
        if "error" in response:
            raise Exception(f"Tool call failed: {response['error']}")
        
        return response.get("result", {})
    
    async def get_ticker(self, pair: str) -> Dict:
        """Get ticker information for a trading pair"""
        return await self.call_tool("kraken_ticker", {"pair": [pair]})
    
    async def get_balance(self) -> Dict:
        """Get account balance"""
        return await self.call_tool("kraken_balance")
    
    async def place_order(self, 
                         pair: str, 
                         side: str, 
                         volume: float,
                         order_type: str = "market",
                         price: Optional[float] = None) -> Dict:
        """Place a trading order"""
        args = {
            "pair": pair,
            "volume": str(volume)
        }
        
        if price:
            args["price"] = str(price)
        
        tool_name = f"kraken_order_{side}"
        return await self.call_tool(tool_name, args)
    
    async def get_open_orders(self) -> Dict:
        """Get all open orders"""
        return await self.call_tool("kraken_open_orders")
    
    async def cancel_order(self, txid: str) -> Dict:
        """Cancel an open order"""
        return await self.call_tool("kraken_order_cancel", {"txid": [txid]})
    
    async def get_ohlcv(self, pair: str, interval: int = 60) -> Dict:
        """Get OHLCV data for analysis"""
        return await self.call_tool("kraken_ohlc", {"pair": pair, "interval": interval})
    
    async def get_orderbook(self, pair: str, count: int = 100) -> Dict:
        """Get order book for a trading pair"""
        return await self.call_tool("kraken_orderbook", {"pair": pair, "count": count})
    
    async def paper_trade_reset(self) -> Dict:
        """Reset paper trading state"""
        return await self.call_tool("kraken_paper_reset")
    
    async def close(self):
        """Close MCP server connection"""
        if self.process:
            self.process.terminate()
            await self.process.wait()
            self.process = None
            self.logger.info("MCP server connection closed")

class KrakenMCPBot:
    """High-level trading bot using Kraken CLI MCP interface"""
    
    def __init__(self, config: MCPConfig):
        self.config = config
        self.mcp_client = MCPClient(config)
        self.logger = logging.getLogger(__name__)
        self.position_tracker = {}
    
    async def initialize(self):
        """Initialize the MCP trading bot"""
        await self.mcp_client.start_server()
        
        # List available tools
        tools = await self.mcp_client.list_tools()
        self.logger.info(f"Available Kraken tools: {len(tools)}")
        
        self.logger.info("Kraken MCP trading bot initialized")
    
    async def get_market_data(self, pair: str) -> Dict:
        """Get comprehensive market data for analysis"""
        try:
            # Get ticker data
            ticker = await self.mcp_client.get_ticker(pair)
            
            # Get order book
            orderbook = await self.mcp_client.get_orderbook(pair)
            
            # Get recent OHLCV data
            ohlcv = await self.mcp_client.get_ohlcv(pair, interval=60)
            
            return {
                'ticker': ticker,
                'orderbook': orderbook,
                'ohlcv': ohlcv,
                'pair': pair,
                'timestamp': asyncio.get_event_loop().time()
            }
        except Exception as e:
            self.logger.error(f"Failed to get market data for {pair}: {e}")
            raise
    
    async def execute_trade(self, 
                           signal: Dict, 
                           pair: str, 
                           max_position_size: float) -> Dict:
        """Execute a trade based on signal using MCP"""
        
        action = signal.get('action')
        confidence = signal.get('confidence', 0)
        position_size = signal.get('position_size', 0)
        
        if action not in ['buy', 'sell'] or confidence < 0.7:
            self.logger.info(f"Signal not actionable: {signal}")
            return {'status': 'rejected', 'reason': 'Low confidence or invalid action'}
        
        # Calculate actual position size
        actual_size = min(position_size * max_position_size, max_position_size)
        
        try:
            # Place order via MCP
            order_result = await self.mcp_client.place_order(
                pair=pair,
                side=action,
                volume=actual_size
            )
            
            if order_result.get('error'):
                self.logger.error(f"Order failed: {order_result['error']}")
                return {'status': 'error', 'error': order_result['error']}
            
            # Track position
            order_data = order_result.get('result', {})
            order_id = order_data.get('txid', ['unknown'])[0] if isinstance(order_data.get('txid'), list) else 'unknown'
            
            self.position_tracker[order_id] = {
                'pair': pair,
                'action': action,
                'size': actual_size,
                'timestamp': asyncio.get_event_loop().time(),
                'signal': signal
            }
            
            self.logger.info(f"Order placed via MCP: {action} {actual_size} {pair}")
            return {'status': 'success', 'order_id': order_id, 'result': order_result}
            
        except Exception as e:
            self.logger.error(f"Trade execution failed: {e}")
            return {'status': 'error', 'error': str(e)}
    
    async def get_account_status(self) -> Dict:
        """Get comprehensive account status"""
        try:
            balance = await self.mcp_client.get_balance()
            open_orders = await self.mcp_client.get_open_orders()
            
            return {
                'balance': balance,
                'open_orders': open_orders,
                'position_tracker': self.position_tracker,
                'timestamp': asyncio.get_event_loop().time()
            }
        except Exception as e:
            self.logger.error(f"Failed to get account status: {e}")
            raise
    
    async def shutdown(self):
        """Shutdown the MCP trading bot"""
        await self.mcp_client.close()
        self.logger.info("Kraken MCP trading bot shutdown complete")

# Example usage
async def example_mcp_usage():
    """Example of using Kraken CLI MCP integration"""
    config = MCPConfig(
        services=["market", "account", "paper"],  # Safe trading only
        allow_dangerous=False
    )
    
    bot = KrakenMCPBot(config)
    
    try:
        await bot.initialize()
        
        # Get market data
        market_data = await bot.get_market_data("BTCUSD")
        print(f"Market data: {market_data['ticker']}")
        
        # Get account status
        account = await bot.get_account_status()
        print(f"Account balance: {account['balance']}")
        
        # Example trade execution (paper trading)
        signal = {
            'action': 'buy',
            'confidence': 0.8,
            'position_size': 0.5,
            'reasoning': 'Mean reversion signal'
        }
        
        result = await bot.execute_trade(signal, "BTCUSD", 0.1)
        print(f"Trade result: {result}")
        
    finally:
        await bot.shutdown()

if __name__ == "__main__":
    asyncio.run(example_mcp_usage())
