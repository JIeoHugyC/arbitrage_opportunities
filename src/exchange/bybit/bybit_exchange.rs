use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::time::sleep;
use crate::exchange::exchange::Exchange;
use crate::exchange::exchange_update::{BestPrices, ExchangeUpdate};
use crate::exchange::order_book::OrderBook;
use crate::exchange::bybit::bybit_websocket::BybitWebSocket;

pub struct BybitExchange {
    name: String,
    orderbook: Arc<RwLock<OrderBook>>,
    websocket: BybitWebSocket,
}

#[async_trait]
impl Exchange for BybitExchange {
    fn new() -> Self {
        BybitExchange {
            name: "Bybit".to_string(),
            orderbook: Arc::new(RwLock::new(OrderBook::new())),
            websocket: BybitWebSocket {},
        }
    }

    async fn start(&self, update_sender: Sender<ExchangeUpdate>) {
        loop {
            println!("Starting Bybit exchange...");
            match self.websocket.connect_and_listen().await {
                Ok(_) => {
                    println!("Bybit WebSocket connection closed normally");
                }
                Err(e) => {
                    eprintln!("Error in Bybit WebSocket connection: {:?}", e);
                }
            }

            // If we're here, it means the connection was closed or an error occurred
            // Wait for a short time before attempting to reconnect
            sleep(Duration::from_secs(5)).await;
            println!("Attempting to reconnect to Bybit WebSocket...");

            // Update the order book and send updates
            self.update_and_send(&update_sender).await;
        }
    }

    fn get_order_book(&self) -> Arc<RwLock<OrderBook>> {
        self.orderbook.clone()
    }
}

impl BybitExchange {
    async fn update_and_send(&self, update_sender: &Sender<ExchangeUpdate>) {
        let orderbook = self.orderbook.read().await;
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
            eprintln!("Failed to send update, receiver might be closed");
        }
    }
}