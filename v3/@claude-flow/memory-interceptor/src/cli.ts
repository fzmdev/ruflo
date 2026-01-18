#!/usr/bin/env node
/**
 * Memory Interceptor CLI
 *
 * Starts the MCP server that intercepts memory operations.
 *
 * Usage:
 *   npx @claude-flow/memory-interceptor
 *   npx @claude-flow/memory-interceptor --db ./custom-memory.db
 *   npx @claude-flow/memory-interceptor --debug
 */

import * as path from 'path';
import * as os from 'os';
import { startInterceptor } from './server.js';

async function main(): Promise<void> {
  const args = process.argv.slice(2);

  // Parse arguments
  const debug = args.includes('--debug') || args.includes('-d');
  const dbPathIndex = args.findIndex(a => a === '--db' || a === '-D');
  const dbPath = dbPathIndex !== -1 && args[dbPathIndex + 1]
    ? args[dbPathIndex + 1]
    : path.join(os.homedir(), '.claude-flow', 'memory-interceptor.db');

  if (args.includes('--help') || args.includes('-h')) {
    console.log(`
Memory Interceptor - Redirect Claude Code memory operations

Usage:
  memory-interceptor [options]

Options:
  --db, -D <path>    Database file path (default: ~/.claude-flow/memory-interceptor.db)
  --debug, -d        Enable debug logging
  --help, -h         Show this help

MCP Registration:
  Add to your Claude settings to intercept memory operations:

  claude mcp add memory-interceptor npx @claude-flow/memory-interceptor

  Or with custom database:

  claude mcp add memory-interceptor npx @claude-flow/memory-interceptor --db /path/to/db

Note:
  Register this BEFORE claude-flow in your MCP config to shadow its memory tools.
  The first registered tool with a given name takes precedence.
`);
    process.exit(0);
  }

  // Handle shutdown
  process.on('SIGINT', () => {
    if (debug) console.error('[MemoryInterceptor] Shutting down...');
    process.exit(0);
  });

  process.on('SIGTERM', () => {
    if (debug) console.error('[MemoryInterceptor] Terminated');
    process.exit(0);
  });

  // Start server
  try {
    if (debug) {
      console.error('[MemoryInterceptor] Starting with config:', { dbPath, debug });
    }

    await startInterceptor({ dbPath, debug });

    // Keep process alive
    await new Promise(() => {});
  } catch (error) {
    console.error('[MemoryInterceptor] Fatal error:', error);
    process.exit(1);
  }
}

main();
