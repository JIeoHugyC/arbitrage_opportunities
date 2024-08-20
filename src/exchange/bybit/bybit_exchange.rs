use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use crate::exchange::exchange::Exchange;
use crate::exchange::exchange_update::{BestPrices, ExchangeUpdate};
use crate::exchange::order_book::OrderBook;

pub struct BybitExchange {
    name: String,
    orderbook: Arc<RwLock<OrderBook>>,
}

#[async_trait]
impl Exchange for BybitExchange {
    fn new() -> Self {
        BybitExchange {
            name: "Bybit".to_string(),
            orderbook: Arc::new(RwLock::new(OrderBook::new())),
        }
    }

    async fn start(&self, update_sender: Sender<ExchangeUpdate>) {
        loop {
            println!("Starting Bybit exchange...");
            // todo: connect to bybit API
            // todo: receive orderbook
            let orderbook = self.orderbook.read().await;
            // todo: update orderbook with received data
            // ...
            let best_bid = orderbook.get_best_bid().unwrap_or(0.0);
            let best_ask = orderbook.get_best_ask().unwrap_or(f64::MAX);
            drop(orderbook);

            let update = ExchangeUpdate {
                exchange_name: self.name.clone(),
                best_prices: BestPrices {
                    best_bid,
                    best_ask,
                },
            };
            if update_sender.send(update).await.is_err() {
                break;
            }
        }
    }

    fn get_order_book(&self) -> Arc<RwLock<OrderBook>> {
        self.orderbook.clone()
    }
}