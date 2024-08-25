use std::env;
use std::str::FromStr;
use super::dexnow_engine::DEXnowEngine;
use crate::exchange::exchange::Exchange;
use crate::exchange::order_book::OrderBook;
use async_trait::async_trait;
use std::sync::Arc;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use tokio::sync::{Mutex, RwLock};
use tokio::sync::mpsc::Sender;
use crate::exchange::exchange_update::ExchangeUpdate;
use crate::trading_pair::ETradingPair;

pub struct DexnowExchange {
    name: String,
    engine: Arc<Mutex<DEXnowEngine>>,
    orderbook: Arc<RwLock<OrderBook>>,
}

#[async_trait]
impl Exchange for DexnowExchange {
    fn new() -> Self {
        let rpc_client = RpcClient::new(env::var("SOLANA_RPC_URL").unwrap());
        let root_account = Pubkey::from_str(&*env::var("ROOT_ACCOUNT_PK").unwrap()).unwrap();
        let program_id = Pubkey::from_str(&*env::var("PROGRAM_ID_PK").unwrap()).unwrap();

        let orderbook = Arc::new(RwLock::new(OrderBook::new()));
        let name = "DEXnow".to_string();
        DexnowExchange {
            name: name.clone(),
            engine: Arc::new(Mutex::new(DEXnowEngine::new(rpc_client, root_account, name, program_id, orderbook.clone()))),
            orderbook,
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    async fn start(&self, trading_pair: ETradingPair, update_sender: Sender<ExchangeUpdate>) {
        let mut engine = self.engine.lock().await;
        engine.initialize(trading_pair, update_sender).await.unwrap();
    }

    fn get_order_book(&self) -> Arc<RwLock<OrderBook>> {
        self.orderbook.clone()
    }
}