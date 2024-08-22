use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::exchange::exchange::Exchange;
use crate::exchange::exchange_update::{BestPrices};
use crate::trading_pair::ETradingPair;

const MESSAGE_BUFFER_SIZE: usize = 100;

/// An arbitrage manager that manages multiple exchanges and analyzes for opportunities
pub struct ArbitrageManager {
    /// List of exchanges to be managed in the arbitrage manager
    exchanges: Vec<Arc<dyn Exchange>>,
    /// Cache best prices for each exchange
    best_prices: HashMap<String, BestPrices>,
    /// The trading pair for which the arbitrage manager is managing exchanges
    trading_pair: ETradingPair,
}

impl ArbitrageManager {
    pub(crate) fn new(trading_pair: ETradingPair) -> Self {
        ArbitrageManager {
            exchanges: Vec::new(),
            best_prices: HashMap::new(),
            trading_pair,
        }
    }

    /// Add a new exchange to the arbitrage manager to manage and update its order book
    pub(crate) fn add_exchange(&mut self, exchange: Arc<dyn Exchange>) {
        self.exchanges.push(exchange);
    }

    /// Start the arbitrage manager and continuously update the order books for the added exchanges
    pub(crate) async fn run(&mut self) {
        let (tx, mut rx) = mpsc::channel(MESSAGE_BUFFER_SIZE);

        for exchange in &self.exchanges {
            let exchange_clone = exchange.clone();
            let tx_clone = tx.clone();
            let trading_pair = self.trading_pair.clone();
            tokio::spawn(async move {
                exchange_clone.start(trading_pair, tx_clone).await;
            });
        }

        while let Some(update) = rx.recv().await {
            self.best_prices.insert(update.exchange_name, update.best_prices);
        }
    }

    /// Analyze for opportunities in the arbitrage manager based on the best prices of the added exchanges
    async fn analyze_opportunities(&self) {
        todo!(); // Implement logic to analyze opportunities based on best prices and trade signals
    }
}