use std::sync::Arc;
use crate::arbitrage_manager::arbitrage_manager::ArbitrageManager;
use crate::exchange::exchange::Exchange;

impl ArbitrageManager {
    /// Add a new exchange to the arbitrage manager to manage and update its order book
    pub(crate) fn add_exchange(&mut self, exchange: Arc<dyn Exchange>) {
        self.exchanges.push(exchange);
    }
}