use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::time::sleep;
use crate::exchange::exchange::Exchange;
use crate::exchange::exchange_update::{ExchangeUpdate};
use crate::exchange::order_book::OrderBook;
use crate::trading_pair::ETradingPair;

pub struct BybitExchange {
    pub(crate) name: String,
    pub(crate) orderbook: Arc<RwLock<OrderBook>>,
}

#[async_trait]
impl Exchange for BybitExchange {
    fn new() -> Self {
        BybitExchange {
            name: "Bybit".to_string(),
            orderbook: Arc::new(RwLock::new(OrderBook::new())),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    async fn start(&self, trading_pair: ETradingPair, order_book_update_sender: Sender<ExchangeUpdate>) {
        loop {
            println!("Starting Bybit exchange...");
            match self.connect_and_listen(&trading_pair, &order_book_update_sender).await {
                Ok(_) => {
                    println!("Bybit WebSocket connection has been closed");
                }
                Err(e) => {
                    eprintln!("Error in Bybit WebSocket connection: {:?}", e);
                }
            }

            // If we're here, it means the connection was closed or an error occurred
            // Wait for a short time before attempting to reconnect
            sleep(Duration::from_secs(1)).await;
            println!("Attempting to reconnect to Bybit WebSocket...");
        }
    }

    fn get_order_book(&self) -> Arc<RwLock<OrderBook>> {
        self.orderbook.clone()
    }
}