//! ERC-8004 AI Trading Agent - TypeScript Entry Point
//! 
//! Main entry point for the TypeScript implementation of the ERC-8004 AI trading agent
//! with Kraken CLI MCP integration.

export * from './config';
export * from './kraken';
export * from './erc8004';
export * from './agent';
export * from './strategies';
export * from './utils';

// Main exports
export { AgentFactory } from './agent';
export { default as logger } from './utils/logger';
