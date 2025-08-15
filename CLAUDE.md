# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`rusty-gateway` is a WeaveVM [0xbabe bundles](https://github.com/weaveVM/bundler) gateway written in Rust. It provides an HTTP API to resolve envelopes from bundles by fetching data from WeaveVM's bundler system and serving it with appropriate content types and caching headers.

The gateway serves as a bridge between WeaveVM bundle transaction IDs and their envelope contents, handling hex decoding and content-type detection automatically.

## Architecture

### Core Components

- **Main Application** (`src/main.rs`): Standard Axum web server running on port 8000 with two main endpoints
- **REST API Module** (`src/utils/rest_api.rs`): HTTP handlers for status and envelope retrieval
- **Bundle Utils** (`src/utils/bundles.rs`): Wrapper around the bundler crate for retrieving envelope data

### Key Dependencies

- **bundler**: External git dependency from WeaveVM for core bundle operations
- **axum**: Web framework for HTTP routing and responses
- **tokio**: Async runtime with full features enabled, including TcpListener

### API Endpoints

- `GET /`: Health status endpoint
- `GET /bundle/:bundle_txid/:envelope_index`: Retrieves and serves envelope content from a bundle

## Development Commands

### Building and Running
```bash
# Build the project
cargo build

# Run locally on port 8000
cargo run

# Check code without building
cargo check

# Run tests
cargo test
```

## Key Implementation Details

### Bundle Resolution Flow
1. Bundle TXID and envelope index provided via URL path
2. `retrieve_bundle_envelopes()` uses bundler crate with `ADDRESS_BABE1` constant
3. Envelope index validation and bounds checking
4. Content-type detection from envelope tags (defaults to "application/octet-stream")
5. Hex decoding of input data with "0x" prefix handling
6. Response with proper content-type and cache headers (1-year cache)

### Error Handling
- Envelope index out of range: 404 NOT_FOUND
- Missing input data: 400 BAD_REQUEST  
- Hex decode failures: 400 BAD_REQUEST
- Bundle retrieval errors: Unwrapped (will panic - consider handling gracefully)

### Content Processing
- Input data is hex-encoded and may have "0x" prefix
- Content-type extracted from envelope tags using case-insensitive "content-type" lookup
- Aggressive caching headers set (`public, max-age=31536000`)