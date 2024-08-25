# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.5] - 2024-08-25

### Added

- Websocket subscription for the Solana account with DEXnow best 20 spot prices
- Price account decoder (using ported TS library)

### Changed

- Engine -> DEXnowEngine
- OrderBookUpdate structure contains non-optional types

## [0.0.4] - 2024-08-24

### Added

- Ported TS library to Rust for the DEXnow exchange
- DEXnow exchange engine

## [0.0.3] - 2024-08-22

### Added

- Implemented order book management for `Bybit` exchange via WebSocket
- Implemented automatic reconnection mechanism in case of WebSocket errors
- Added ability to select trading pair at program startup

## [0.0.2] - 2024-08-20

### Added
- Basic project structure and initial implementation of core components
- `exchange` module with:
    - Generic `Exchange` trait
    - `OrderBook` structure for managing bids and asks
    - `ExchangeUpdate` structure for handling updates
- `bybit` module with initial `BybitExchange` implementation
- `arbitrage_manager` module with `ArbitrageManager` structure for managing exchanges and analyzing opportunities

## [0.0.1] - 2024-08-20

### Added
- Initial repo files