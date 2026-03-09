//! Type definitions for ERC-8004 AI Trading Agent

export interface AgentConfig {
  agentId: string;
  agentName: string;
  kraken: KrakenConfig;
  erc8004: ERC8004Config;
  minReputationThreshold: number;
  maxDailyTrades: number;
  emergencyStop: boolean;
  logLevel: string;
  logFile?: string;
}

export interface KrakenConfig {
  krakenCliPath: string;
  services: string[];
  allowDangerous: boolean;
  paperTradingOnly: boolean;
  timeoutSeconds: number;
  maxPositionSize: number;
  riskLevel: string;
}

export interface ERC8004Config {
  rpcUrl: string;
  privateKey: string;
  chainId: number;
  identityRegistryAddress: string;
  reputationRegistryAddress: string;
  validationRegistryAddress: string;
}

export interface TradingSignal {
  action: 'buy' | 'sell';
  pair: string;
  confidence: number;
  positionSize: number;
  price?: number;
  reasoning: string;
}

export interface TradeResult {
  status: string;
  reason: string;
  orderId?: string;
}

export interface MarketData {
  pair: string;
  ticker: any;
  orderbook: any;
  ohlcv: any;
  timestamp: Date;
}

export interface AccountStatus {
  balance: any;
  openOrders: any;
  positions: Position[];
  timestamp: Date;
}

export interface Position {
  pair: string;
  side: string;
  size: number;
  timestamp: Date;
  entryPrice?: number;
  orderId: string;
}

export interface AgentState {
  agentId: string;
  isRunning: boolean;
  currentPositions: string[];
  lastTradeTimestamp?: Date;
  totalTrades: number;
  successfulTrades: number;
  currentBalance?: number;
}

export interface AgentProfile {
  info: AgentInfo;
  reputation: number;
  reputationHistory: number[];
}

export interface AgentInfo {
  id: string;
  metadata: string;
  verified: boolean;
  registrationTimestamp: bigint;
}

export interface StrategyInfo {
  id: string;
  metadata: string;
  valid: boolean;
  validationTimestamp: bigint;
}

export interface RiskParameters {
  maxPositionSize: number;
  maxDailyLoss: number;
  confidenceThreshold: number;
  stopLossPercentage: number;
  takeProfitPercentage: number;
}

export interface MCPRequest {
  jsonrpc: string;
  id?: any;
  method: string;
  params?: any;
}

export interface MCPResponse {
  jsonrpc: string;
  id?: any;
  result?: any;
  error?: MCPError;
}

export interface MCPError {
  code: number;
  message: string;
  data?: any;
}

export interface StrategyType {
  name: string;
  type: string;
  riskParameters: RiskParameters;
}

export interface TradingCycleResult {
  signalGenerated: boolean;
  signalExecuted: boolean;
  signal: TradingSignal;
  tradeResult?: TradeResult;
  reason: string;
}

export interface LogLevel {
  error: number;
  warn: number;
  info: number;
  debug: number;
}

export interface LoggerConfig {
  level: string;
  file?: string;
  console: boolean;
  format: string;
}
