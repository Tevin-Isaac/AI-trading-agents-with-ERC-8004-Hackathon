//! Configuration Management - TypeScript Implementation
//! 
//! Provides configuration structures and management for the ERC-8004 AI trading agent

import * as fs from 'fs';
import * as path from 'path';
import * as dotenv from 'dotenv';
import { AgentConfig, KrakenConfig, ERC8004Config, LoggerConfig } from '../types';

dotenv.config();

export class ConfigManager {
  private static instance: ConfigManager;
  private config: AgentConfig;

  private constructor() {
    this.config = this.createDefaultConfig();
  }

  public static getInstance(): ConfigManager {
    if (!ConfigManager.instance) {
      ConfigManager.instance = new ConfigManager();
    }
    return ConfigManager.instance;
  }

  private createDefaultConfig(): AgentConfig {
    return {
      agentId: process.env.AGENT_ID || 'erc8004_agent_001',
      agentName: process.env.AGENT_NAME || 'ERC-8004 Trading Bot',
      kraken: {
        krakenCliPath: process.env.KRAKEN_CLI_PATH || 'kraken',
        services: (process.env.KRAKEN_MCP_SERVICES || 'market,account,paper').split(','),
        allowDangerous: process.env.KRAKEN_ALLOW_DANGEROOUS === 'true',
        paperTradingOnly: process.env.PAPER_TRADING_ONLY !== 'false',
        timeoutSeconds: parseInt(process.env.KRAKEN_TIMEOUT_SECONDS || '30'),
        maxPositionSize: parseFloat(process.env.MAX_POSITION_SIZE || '0.1'),
        riskLevel: process.env.RISK_LEVEL || 'medium',
      },
      erc8004: {
        rpcUrl: process.env.ETHEREUM_RPC_URL || 'https://sepolia.infura.io/v3/YOUR_PROJECT_ID',
        privateKey: process.env.PRIVATE_KEY || 'your_private_key_here',
        chainId: parseInt(process.env.NETWORK_ID || '11155111'),
        identityRegistryAddress: process.env.IDENTITY_REGISTRY_ADDRESS || '0x0000000000000000000000000000000000000000',
        reputationRegistryAddress: process.env.REPUTATION_REGISTRY_ADDRESS || '0x0000000000000000000000000000000000000000',
        validationRegistryAddress: process.env.VALIDATION_REGISTRY_ADDRESS || '0x0000000000000000000000000000000000000000',
      },
      minReputationThreshold: parseInt(process.env.REPUTATION_THRESHOLD || '70'),
      maxDailyTrades: parseInt(process.env.MAX_DAILY_TRADES || '50'),
      emergencyStop: process.env.EMERGENCY_STOP === 'true',
      logLevel: process.env.LOG_LEVEL || 'info',
      logFile: process.env.LOG_FILE,
    };
  }

  public getConfig(): AgentConfig {
    return this.config;
  }

  public loadFromFile(configPath: string): AgentConfig {
    try {
      const configData = fs.readFileSync(configPath, 'utf8');
      const loadedConfig = JSON.parse(configData) as AgentConfig;
      
      // Merge with environment variables (env vars take precedence)
      this.config = { ...loadedConfig, ...this.createDefaultConfig() };
      
      return this.config;
    } catch (error) {
      throw new Error(`Failed to load config from ${configPath}: ${error}`);
    }
  }

  public saveToFile(configPath: string): void {
    try {
      const configDir = path.dirname(configPath);
      if (!fs.existsSync(configDir)) {
        fs.mkdirSync(configDir, { recursive: true });
      }
      
      const configData = JSON.stringify(this.config, null, 2);
      fs.writeFileSync(configPath, configData);
    } catch (error) {
      throw new Error(`Failed to save config to ${configPath}: ${error}`);
    }
  }

  public validateConfig(): void {
    const config = this.config;

    // Validate Kraken CLI path
    if (!config.kraken.krakenCliPath) {
      throw new Error('Kraken CLI path cannot be empty');
    }

    // Validate RPC URL
    if (!config.erc8004.rpcUrl.startsWith('http')) {
      throw new Error('Invalid RPC URL format');
    }

    // Validate private key
    if (config.erc8004.privateKey.length !== 64) {
      throw new Error('Invalid private key format');
    }

    // Validate contract addresses
    const addresses = [
      config.erc8004.identityRegistryAddress,
      config.erc8004.reputationRegistryAddress,
      config.erc8004.validationRegistryAddress,
    ];

    for (const addr of addresses) {
      if (!addr.startsWith('0x') || addr.length !== 42) {
        throw new Error(`Invalid contract address format: ${addr}`);
      }
    }

    // Validate agent settings
    if (!config.agentId) {
      throw new Error('Agent ID cannot be empty');
    }

    if (config.minReputationThreshold < 0 || config.minReputationThreshold > 100) {
      throw new Error('Reputation threshold must be between 0 and 100');
    }

    if (config.kraken.maxPositionSize <= 0 || config.kraken.maxPositionSize > 1) {
      throw new Error('Max position size must be between 0 and 1');
    }
  }

  public getEnvironmentConfig(env: string): AgentConfig {
    switch (env) {
      case 'development':
        return {
          ...this.createDefaultConfig(),
          kraken: {
            ...this.createDefaultConfig().kraken,
            paperTradingOnly: true,
            allowDangerous: false,
          },
          logLevel: 'debug',
        };

      case 'testing':
        return {
          ...this.createDefaultConfig(),
          kraken: {
            ...this.createDefaultConfig().kraken,
            paperTradingOnly: true,
            allowDangerous: false,
          },
          erc8004: {
            ...this.createDefaultConfig().erc8004,
            chainId: 31337, // Local hardhat
          },
          logLevel: 'info',
        };

      case 'production':
        return {
          ...this.createDefaultConfig(),
          kraken: {
            ...this.createDefaultConfig().kraken,
            paperTradingOnly: false,
            allowDangerous: true,
          },
          erc8004: {
            ...this.createDefaultConfig().erc8004,
            chainId: 1, // Ethereum mainnet
          },
          logLevel: 'warn',
        };

      default:
        throw new Error(`Unknown environment: ${env}`);
    }
  }

  public updateConfig(updates: Partial<AgentConfig>): void {
    this.config = { ...this.config, ...updates };
  }

  public getKrakenConfig(): KrakenConfig {
    return this.config.kraken;
  }

  public getERC8004Config(): ERC8004Config {
    return this.config.erc8004;
  }

  public getLoggerConfig(): LoggerConfig {
    return {
      level: this.config.logLevel,
      file: this.config.logFile,
      console: true,
      format: 'json',
    };
  }
}

export default ConfigManager;
