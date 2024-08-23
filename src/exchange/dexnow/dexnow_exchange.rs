use std::env;
use std::str::FromStr;
use super::engine::Engine;
use crate::exchange::exchange::Exchange;
use crate::exchange::order_book::OrderBook;
use async_trait::async_trait;
use std::sync::Arc;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;
use crate::exchange::exchange_update::ExchangeUpdate;
use crate::trading_pair::ETradingPair;

pub struct DexnowExchange {
    name: String,
    engine: Engine,
    orderbook: Arc<RwLock<OrderBook>>,
}

#[async_trait]
impl Exchange for DexnowExchange {
    fn new() -> Self {
        let rpc_client = RpcClient::new(env::var("SOLANA_RPC_URL").unwrap());
        let root_account = Pubkey::from_str(&*env::var("SOLANA_ROOT_ACCOUNT").unwrap()).unwrap();
        let program_id = Pubkey::from_str(&*env::var("SOLANA_PROGRAM_ID").unwrap()).unwrap();

        DexnowExchange {
            name: "DEXnow".to_string(),
            engine: Engine::new(rpc_client, root_account, program_id),
            orderbook: Arc::new(RwLock::new(OrderBook::new())),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    async fn start(&self, trading_pair: ETradingPair, update_sender: Sender<ExchangeUpdate>) {
        // self.engine.
    }

    fn get_order_book(&self) -> Arc<RwLock<OrderBook>> {
        self.orderbook.clone()
    }
}