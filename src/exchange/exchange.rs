use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::exchange::exchange_update::ExchangeUpdate;
use crate::exchange::order_book::OrderBook;
use crate::trading_pair::ETradingPair;

/// A trait for all exchanges to implement
#[async_trait]
pub trait Exchange: Send + Sync {
    /// Create a new instance of the exchange
    fn new() -> Self where Self: Sized;

    /// Get the name of the exchange
    fn name(&self) -> String;

    /// Start the exchange and continuously update the order book
    async fn start(&self, trading_pair: ETradingPair, update_sender: Sender<ExchangeUpdate>);

    /// Get a reference to the order book locker object
    fn get_order_book(&self) -> Arc<RwLock<OrderBook>>;
}