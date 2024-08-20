use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::exchange::exchange::Exchange;
use crate::exchange::exchange_update::{BestPrices};

const MESSAGE_BUFFER_SIZE: usize = 100;

struct ArbitrageManager {
    /// List of exchanges to be managed in the arbitrage manager
    exchanges: Vec<Arc<dyn Exchange>>,
    /// Cache best prices for each exchange
    best_prices: HashMap<String, BestPrices>,
}

impl ArbitrageManager {
    fn new() -> Self {
        ArbitrageManager {
            exchanges: Vec::new(),
            best_prices: HashMap::new(),
        }
    }

    fn add_exchange(&mut self, exchange: Arc<dyn Exchange>) {
        self.exchanges.push(exchange);
    }

    async fn run(&mut self) {
        let (tx, mut rx) = mpsc::channel(MESSAGE_BUFFER_SIZE);

        for exchange in &self.exchanges {
            let exchange_clone = exchange.clone();
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                exchange_clone.start(tx_clone).await;
            });
        }

        while let Some(update) = rx.recv().await {
            self.best_prices.insert(update.exchange_name, update.best_prices);
        }
    }

    async fn analyze_opportunities(&self) {
        todo!(); // Implement logic to analyze opportunities based on best prices and trade signals
    }
}