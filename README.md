# Trading Framework

*A Rust trading engine acting on strategies defined in Python.*

**Early design**. Described below are the intended features and project architecture.

## Overview

This project aims to provide a reusable trading framework for backtesting, paper trading, and live trading.

Rust handles historical and live market data, portfolio accounting, risk management, position sizing, and order execution. Trading strategies are developed independently in Python and return **BUY**, **SELL**, or **HOLD** signals to the Rust engine.

## Design Principles

- Strategies never place broker orders directly.
- Strategies can use Python's statistical and machine-learning ecosystem without being part of this repository.
- Backtesting, paper trading, and live trading use the same signal validation, position sizing, allocation, and risk-management logic.
- Historical data, strategy versions, parameters, signals, and execution outcomes should be traceable.

## Architecture

The framework consists of the following components:

1. **Market Data (Rust):** Fetch, normalize, validate, store, and stream historical and live market data.
2. **Strategy Interface (Python):** Evaluate market conditions and generate trading signals. Support multiple concurrent strategies.
3. **Signal and Allocation Engine (Rust):** Validate signals, resolve conflicting strategy demands, and determine intended portfolio exposure.
4. **Risk Management (Rust):** Calculate portfolio risk, estimate risk of ruin, determine position sizes, and enforce risk limits.
5. **Order Management (Rust):** Create, submit, track, cancel, and reconcile orders.
6. **Execution Adapters (Rust):** Execute orders through historical simulation, paper trading, or live broker/exchange connections.

**Data flow:**

Market Data →  Strategies →  Signal & Allocation Engine →  Risk Management →  Order Management → Backtest / Paper / Live Execution →  Portfolio State

## Strategy Interface

Strategies are developed in Python and communicate with the Rust engine through a PYO3 interface.

A strategy receives market data and relevant portfolio context and returns:

- **BUY:** Propose increasing long exposure or reducing short exposure.
- **SELL:** Propose reducing long exposure or increasing short exposure, subject to configuration.
- **HOLD:** No new action is requested.
- **ERROR:** Strategy evaluation failed. Rust records and handles the failure independently.

Signals may later include optional information such as the instrument, stop-loss level, confidence, or target exposure.

**A strategy signal is not an executable order.** Rust determines whether the proposed trade is permitted and calculates the appropriate quantity.

(For live trading, Python strategies may run in supervised worker processes to provide fault isolation, timeouts, and independent restart capabilities.)

## Centralized Risk Management

A single Rust risk-management implementation serves **all strategies and the entire portfolio**.

Risk management operates at three levels.

### Portfolio-Level Risk

- Maximum portfolio exposure and leverage
- Capital availability and allocation
- Portfolio drawdown limits
- Concentration and correlated exposures
- Portfolio-wide risk-of-ruin estimates

### Strategy-Level Risk

- Allocated capital and risk budget
- Maximum position exposure
- Strategy-specific loss limits
- Permitted instruments and trading constraints

### Order-Level Risk

- Position and order sizing
- Stop-loss distance and estimated loss
- Available balance and margin requirements
- Fees, exchange lot sizes, and price precision
- Leverage and order validation

Concurrent strategies must account for both existing positions and pending orders to prevent double allocation of available capital.

## Trading Modes

The framework supports three operating modes.

### Backtesting

- Historical market-data replay
- Strategy evaluation using past market events
- Simulated order execution, fees, and slippage
- Portfolio performance and risk analysis
- Protection against look-ahead bias

### Paper Trading

- Real-time market data
- Simulated orders and fills
- Identical strategy and risk-management interfaces
- Evaluation of live signal behavior without financial exposure

### Live Trading

- Real broker or exchange connections
- Actual order submission and tracking
- Centralized pre-trade risk checks
- Account and order reconciliation
- Monitoring, logging, and emergency trading controls

The three modes share the same trading-decision and risk-management logic, but use different data and execution adapters.

## Safety and Reliability

The engine should:

- Reject invalid, duplicate, expired, or out-of-order signals.
- Distinguish strategy errors from legitimate HOLD decisions.
- Apply risk limits independently of Python strategy code.
- Reconcile uncertain order submissions with the exchange.
- Handle partial fills, disconnections, and rejected orders.
- Persist positions, balances, orders, and execution history.
- Provide configurable fail-safe behavior and an emergency kill switch.
- Require explicit activation of live trading.

A strategy failure should not automatically cause liquidation. The response must follow an explicit, tested policy.

## Development Roadmap

1. **Historical Data:** Implement a Rust data fetcher, normalization, validation, and local storage.
2. **Core Models:** Define market events, instruments, positions, orders, fills, and portfolio state.
3. **Backtesting:** Implement a deterministic historical event-replay and execution simulator.
4. **Risk Management:** Add position sizing, portfolio-wide limits, capital reservations, and strategy risk budgets.
5. **Python Integration:** Implement strategy communication, signal validation, and concurrent strategy support.
6. **Paper Trading:** Integrate real-time market feeds with simulated order execution.
7. **Live Trading:** Add exchange adapters, account reconciliation, logging, and operational safeguards.
8. **Quantitative Research:** Extend statistical analysis, strategy evaluation, robustness testing, and risk-of-ruin estimation.
9. **Safety Guards:** Remotely executed kill switch and clean stopping procedures.

## Project Scope

This repository contains the **trading framework and infrastructure**, not individual trading strategies.

The long-term objective is a reusable Rust trading platform that supports flexible quantitative research in Python while maintaining centralized control of capital, risk, and execution.
