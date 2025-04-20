# Architecture Overview

## Components

1. **Extractor**
   - SBF bytecode parser
   - Instruction decoder
   - Account layout analyzer

2. **Analyzer**
   - On-chain data fetcher
   - Performance benchmarks

3. **Risk Engine**
   - Pattern database
   - Risk scoring

## Data Flow

Program Binary → Extractor → Analyzer → Risk Engine → Report 

## Key Design Decisions

- **No Heavy Dependencies**: Pure Rust for core analysis
- **Extensible Risk Patterns**: Simple file-based database
- **CI-Ready**: Tested on Linux/Mac/Windows