# Arbitrage Opportunities

This project aims to identify and analyze arbitrage opportunities between CEX and DEX spot markets.

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Project Overview](#project-overview)
4. [Limitations](#limitations)
5. [Algorithm](#algorithm)

## Introduction

Arbitrage Opportunities is a project designed to detect and analyze price 
discrepancies between centralized exchanges (CEX) and decentralized exchanges 
(DEX) in the spot market, potentially allowing for profit from these differences.

## Getting Started

To run the project, you need to set up the environment variables. 
An `.env.example` file is provided in the repository. 
You can rename it to `.env`, and the project will run using these variables.

## Project Overview

Currently, the project supports searching for arbitrage opportunities 
only in the SOL/USDC pair. However, the project is not technically 
limited to specific exchanges. New exchanges can be added by writing a driver 
for the required exchange.

The project is designed for a single run without the ability to stop during execution.

## Limitations

The proposed algorithm for finding arbitrage opportunities is one of many possible approaches. 
It does not claim to be the best approach (and is not).

## Algorithm

The general algorithm of the program works as follows:

1. At startup, exchange drivers are created and then assigned to the arbitrage manager, which launches them
2. Each driver independently maintains a WebSocket connection in a separate task and keeps the order book up to date
3. Each such task receives a channel for sending messages to the arbitrage manager, which analyzes any market change

To avoid overloading the system, the arbitrage manager conducts analysis in two stages:

1. It checks the local index for the presence of the best price as a fact (without capturing a mutex on order books)
2. If there is an arbitrage opportunity, order books of specific exchanges are captured, and a deeper analysis is performed. The algorithm evaluates a potential transaction to buy one asset on one exchange and sell it on another

**Important note:** The algorithm does not take into account exchange commissions and Solana network fees.