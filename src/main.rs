use std::sync::Arc;
use crate::arbitrage_manager::arbitrage_manager::ArbitrageManager;
use crate::exchange::bybit::bybit_exchange::BybitExchange;
use crate::exchange::exchange::Exchange;

mod arbitrage_manager;
mod exchange;

#[tokio::main]
async fn main() {
    let mut arbitrage_manager = ArbitrageManager::new();
    let bybit_exchange = Arc::new(BybitExchange::new());
    arbitrage_manager.add_exchange(bybit_exchange);
    arbitrage_manager.run().await;
}
