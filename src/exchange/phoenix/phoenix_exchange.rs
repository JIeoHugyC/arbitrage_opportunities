use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::time::interval;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use crate::exchange::exchange::Exchange;
use crate::exchange::exchange_update::{ExchangeUpdate, BestPrices};
use crate::exchange::order_book::{OrderBook, TPrice, TVolume};
use crate::exchange::phoenix::market_state::MarketState;
use crate::trading_pair::ETradingPair;

pub struct PhoenixExchange {
    name: String,
    pub orderbook: Arc<RwLock<OrderBook>>,
    market_address: Pubkey,
    rpc_client: RpcClient,
}

#[async_trait]
impl Exchange for PhoenixExchange {
    fn new() -> Self {
        let rpc_url = std::env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set");
        PhoenixExchange {
            name: "Phoenix".to_string(),
            orderbook: Arc::new(RwLock::new(OrderBook::new())),
            market_address: "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg".parse().unwrap(),
            rpc_client: RpcClient::new(rpc_url),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    async fn start(&self, trading_pair: ETradingPair, order_book_update_sender: Sender<ExchangeUpdate>) {
        loop {
            println!("Starting Phoenix exchange...");
            match self.connect_and_listen(&order_book_update_sender).await {
                Ok(_) => {
                    println!("Phoenix connection has been closed");
                }
                Err(e) => {
                    eprintln!("Error in Phoenix connection: {:?}", e);
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Attempting to reconnect to Phoenix...");
        }
    }

    fn get_order_book(&self) -> Arc<RwLock<OrderBook>> {
        self.orderbook.clone()
    }
}

impl PhoenixExchange {
    async fn connect_and_listen(&self, order_book_update_sender: &Sender<ExchangeUpdate>) -> Result<(), Box<dyn std::error::Error>> {
        let mut update_interval = interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                _ = update_interval.tick() => {
                    self.fetch_orderbook().await?;

                    let orderbook = self.orderbook.read().await;
                    let best_bid = orderbook.get_best_bid();
                    let best_ask = orderbook.get_best_ask();

                    if let (Some(bid), Some(ask)) = (best_bid, best_ask) {
                        let update = ExchangeUpdate {
                            exchange_name: self.name.clone(),
                            best_prices: BestPrices {
                                best_bid: Some(bid),
                                best_ask: Some(ask),
                            },
                        };
                        order_book_update_sender.send(update).await?;
                    }
                }
            }
        }
    }

    async fn fetch_orderbook(&self) -> Result<(), Box<dyn std::error::Error>> {
        let account = self.rpc_client.get_account(&self.market_address)?;
        let market_state = bytemuck::try_from_bytes::<MarketState>(&account.data).unwrap();

        let mut orderbook = self.orderbook.write().await;

        orderbook.bids.clear();
        orderbook.asks.clear();

        // for book_entry in market_state.bids().iter() {
        //     let price = book_entry.price_in_ticks.get() as f64 / market_state.tick_size_in_quote_atoms_per_base_unit.get() as f64;
        //     let size = book_entry.base_lot_free.get() as f64 * market_state.base_lots_per_base_unit.get() as f64;
        //     orderbook.bids.insert(TPrice::from(price), size);
        // }
        //
        // for book_entry in market_state.asks().iter() {
        //     let price = book_entry.price_in_ticks.get() as f64 / market_state.tick_size_in_quote_atoms_per_base_unit.get() as f64;
        //     let size = book_entry.base_lot_free.get() as f64 * market_state.base_lots_per_base_unit.get() as f64;
        //     orderbook.asks.insert(TPrice::from(price), size);
        // }
        //
        // orderbook.sequence += 1;
        // orderbook.last_updated = chrono::Utc::now();

        Ok(())
    }
}