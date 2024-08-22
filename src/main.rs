use std::sync::Arc;
use dotenv::dotenv;
use crate::arbitrage_manager::arbitrage_manager::ArbitrageManager;
use crate::exchange::bybit::bybit_exchange::BybitExchange;
use crate::exchange::exchange::Exchange;
use crate::trading_pair::ETradingPair;

mod arbitrage_manager;
mod exchange;
mod trading_pair;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut arbitrage_manager = ArbitrageManager::new(ETradingPair::BtcUsdc);
    let bybit_exchange = Arc::new(BybitExchange::new());
    arbitrage_manager.add_exchange(bybit_exchange);
    arbitrage_manager.run().await;
}
