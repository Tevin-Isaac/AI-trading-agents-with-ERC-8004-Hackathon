//! Kraken CLI MCP Integration - TypeScript Implementation
//! 
//! Provides TypeScript client for interacting with Kraken's CLI through the Model Context Protocol

import { spawn, ChildProcess } from 'child_process';
import { EventEmitter } from 'events';
import * as uuid from 'uuid';
import {
  KrakenConfig,
  MCPRequest,
  MCPResponse,
  TradingSignal,
  TradeResult,
  MarketData,
  AccountStatus,
  Position
} from '../types';

export class MCPClient extends EventEmitter {
  private config: KrakenConfig;
  private process?: ChildProcess;
  private nextRequestId = 1;
  private pendingRequests = new Map<string | number, (response: MCPResponse) => void>();

  constructor(config: KrakenConfig) {
    super();
    this.config = config;
  }

  public async start(): Promise<void> {
    console.log('Starting Kraken CLI MCP server...');

    const args = ['mcp', '-s', this.config.services.join(',')];
    if (this.config.allowDangerous) {
      args.push('--allow-dangerous');
    }

    this.process = spawn(this.config.krakenCliPath, args);

    if (!this.process.stdin || !this.process.stdout) {
      throw new Error('Failed to spawn Kraken CLI process');
    }

    // Setup stdout handling
    this.process.stdout.on('data', (data: Buffer) => {
      const lines = data.toString().split('\n').filter(line => line.trim());
      
      for (const line of lines) {
        try {
          const response: MCPResponse = JSON.parse(line);
          this.handleResponse(response);
        } catch (error) {
          console.error('Failed to parse MCP response:', error);
        }
      }
    });

    // Setup error handling
    this.process.on('error', (error) => {
      console.error('Kraken CLI process error:', error);
      this.emit('error', error);
    });

    this.process.on('exit', (code) => {
      console.log(`Kraken CLI process exited with code ${code}`);
      this.emit('exit', code);
    });

    // Initialize MCP session
    await this.initializeSession();
    
    console.log('MCP server started successfully');
  }

  private async initializeSession(): Promise<void> {
    const initRequest: MCPRequest = {
      jsonrpc: '2.0',
      id: this.nextRequestId++,
      method: 'initialize',
      params: {
        protocolVersion: '2024-11-05',
        capabilities: {
          tools: {}
        },
        clientInfo: {
          name: 'ERC-8004 AI Trading Agent',
          version: '1.0.0'
        }
      }
    };

    const response = await this.sendRequest(initRequest);
    
    if (response.error) {
      throw new Error(`MCP initialization failed: ${response.error.message}`);
    }

    // Send initialized notification
    const notification: MCPRequest = {
      jsonrpc: '2.0',
      method: 'notifications/initialized'
    };

    this.sendNotification(notification);
  }

  private sendRequest(request: MCPRequest): Promise<MCPResponse> {
    return new Promise((resolve, reject) => {
      const id = request.id || this.nextRequestId++;
      
      // Store callback
      this.pendingRequests.set(id, (response) => {
        if (response.error) {
          reject(new Error(response.error.message));
        } else {
          resolve(response);
        }
      });

      // Send request
      const requestJson = JSON.stringify(request) + '\n';
      this.process?.stdin?.write(requestJson);
    });
  }

  private sendNotification(notification: MCPRequest): void {
    const notificationJson = JSON.stringify(notification) + '\n';
    this.process?.stdin?.write(notificationJson);
  }

  private handleResponse(response: MCPResponse): void {
    const id = response.id;
    if (id !== undefined) {
      const callback = this.pendingRequests.get(id);
      if (callback) {
        this.pendingRequests.delete(id);
        callback(response);
      }
    }
  }

  public async listTools(): Promise<any[]> {
    const request: MCPRequest = {
      jsonrpc: '2.0',
      id: this.nextRequestId++,
      method: 'tools/list'
    };

    const response = await this.sendRequest(request);
    return response.result?.tools || [];
  }

  public async callTool(toolName: string, arguments?: any): Promise<any> {
    const request: MCPRequest = {
      jsonrpc: '2.0',
      id: this.nextRequestId++,
      method: 'tools/call',
      params: {
        name: toolName,
        arguments: arguments || {}
      }
    };

    const response = await this.sendRequest(request);
    return response.result;
  }

  public async getTicker(pair: string): Promise<any> {
    return this.callTool('kraken_ticker', { pair: [pair] });
  }

  public async getBalance(): Promise<any> {
    return this.callTool('kraken_balance');
  }

  public async placeOrder(
    pair: string,
    side: 'buy' | 'sell',
    volume: number,
    price?: number
  ): Promise<any> {
    const args: any = {
      pair,
      volume: volume.toString()
    };

    if (price !== undefined) {
      args.price = price.toString();
    }

    const toolName = `kraken_order_${side}`;
    return this.callTool(toolName, args);
  }

  public async getOpenOrders(): Promise<any> {
    return this.callTool('kraken_open_orders');
  }

  public async cancelOrder(txid: string): Promise<any> {
    return this.callTool('kraken_order_cancel', { txid: [txid] });
  }

  public async getOHLCV(pair: string, interval: number): Promise<any> {
    return this.callTool('kraken_ohlc', { pair, interval });
  }

  public async getOrderbook(pair: string, count: number): Promise<any> {
    return this.callTool('kraken_orderbook', { pair, count });
  }

  public async resetPaperTrading(): Promise<any> {
    return this.callTool('kraken_paper_reset');
  }

  public stop(): void {
    if (this.process) {
      this.process.kill();
      this.process = undefined;
    }
  }
}

export class KrakenBot extends EventEmitter {
  private mcpClient: MCPClient;
  private positionTracker = new Map<string, Position>();

  constructor(config: KrakenConfig) {
    super();
    this.mcpClient = new MCPClient(config);
  }

  public async initialize(): Promise<void> {
    await this.mcpClient.start();
    
    // Setup event forwarding
    this.mcpClient.on('error', (error) => this.emit('error', error));
    this.mcpClient.on('exit', (code) => this.emit('exit', code));
  }

  public async getMarketData(pair: string): Promise<MarketData> {
    const [ticker, orderbook, ohlcv] = await Promise.all([
      this.mcpClient.getTicker(pair),
      this.mcpClient.getOrderbook(pair, 100),
      this.mcpClient.getOHLCV(pair, 60)
    ]);

    return {
      pair,
      ticker,
      orderbook,
      ohlcv,
      timestamp: new Date()
    };
  }

  public async executeTrade(signal: TradingSignal, maxPositionSize: number): Promise<TradeResult> {
    if (signal.confidence < 0.7) {
      return {
        status: 'rejected',
        reason: 'Low confidence'
      };
    }

    const actualSize = Math.min(signal.positionSize * maxPositionSize, maxPositionSize);
    
    try {
      const orderResult = await this.mcpClient.placeOrder(
        signal.pair,
        signal.action,
        actualSize,
        signal.price
      );

      // Extract order ID
      const orderId = orderResult.result?.txid?.[0] || 'unknown';

      // Track position
      const position: Position = {
        pair: signal.pair,
        side: signal.action,
        size: actualSize,
        timestamp: new Date(),
        entryPrice: signal.price,
        orderId
      };

      this.positionTracker.set(orderId, position);

      return {
        status: 'success',
        reason: 'Order placed',
        orderId
      };
    } catch (error) {
      return {
        status: 'error',
        reason: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  public async getAccountStatus(): Promise<AccountStatus> {
    const [balance, openOrders] = await Promise.all([
      this.mcpClient.getBalance(),
      this.mcpClient.getOpenOrders()
    ]);

    return {
      balance,
      openOrders,
      positions: Array.from(this.positionTracker.values()),
      timestamp: new Date()
    };
  }

  public async cancelAllOrders(): Promise<string[]> {
    const accountStatus = await this.getAccountStatus();
    const cancelledOrders: string[] = [];

    if (accountStatus.openOrders.result?.open) {
      const openOrders = accountStatus.openOrders.result.open;
      
      for (const txid of Object.keys(openOrders)) {
        try {
          await this.mcpClient.cancelOrder(txid);
          cancelledOrders.push(txid);
          console.log(`Cancelled order: ${txid}`);
        } catch (error) {
          console.error(`Failed to cancel order ${txid}:`, error);
        }
      }
    }

    return cancelledOrders;
  }

  public stop(): void {
    this.mcpClient.stop();
  }
}

export class UnifiedKrakenIntegration {
  private krakenBot?: KrakenBot;
  private config: KrakenConfig;

  constructor(config: KrakenConfig) {
    this.config = config;
  }

  public async initialize(): Promise<void> {
    this.krakenBot = new KrakenBot(this.config);
    await this.krakenBot.initialize();
  }

  public async getMarketData(pair: string): Promise<MarketData> {
    if (!this.krakenBot) {
      throw new Error('Kraken integration not initialized');
    }
    return this.krakenBot.getMarketData(pair);
  }

  public async executeTrade(signal: TradingSignal, maxPositionSize: number): Promise<TradeResult> {
    if (!this.krakenBot) {
      throw new Error('Kraken integration not initialized');
    }
    return this.krakenBot.executeTrade(signal, maxPositionSize);
  }

  public async getAccountStatus(): Promise<AccountStatus> {
    if (!this.krakenBot) {
      throw new Error('Kraken integration not initialized');
    }
    return this.krakenBot.getAccountStatus();
  }

  public stop(): void {
    if (this.krakenBot) {
      this.krakenBot.stop();
    }
  }
}

export default {
  MCPClient,
  KrakenBot,
  UnifiedKrakenIntegration
};
