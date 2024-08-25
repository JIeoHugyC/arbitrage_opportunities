use std::sync::Arc;
use dotenv::dotenv;
use strum::IntoEnumIterator;
use crate::arbitrage_manager::arbitrage_manager::ArbitrageManager;
use crate::exchange::bybit::bybit_exchange::BybitExchange;
use crate::exchange::dexnow::dexnow_exchange::DexnowExchange;
use crate::exchange::exchange::Exchange;
use crate::trading_pair::ETradingPair;

mod arbitrage_manager;
mod exchange;
mod trading_pair;

#[tokio::main]
async fn main() {
    dotenv().ok();
    for pair in ETradingPair::iter() {
        println!("Supported currency: {}", pair);
    }
    let mut arbitrage_manager = ArbitrageManager::new(ETradingPair::SolUsdc);
    let bybit_exchange = Arc::new(BybitExchange::new());
    let dexnow_exchange = Arc::new(DexnowExchange::new());
    // arbitrage_manager.add_exchange(bybit_exchange);
    arbitrage_manager.add_exchange(dexnow_exchange);
    arbitrage_manager.run().await;
}
