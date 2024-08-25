#![allow(dead_code)]

use std::sync::Arc;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
};
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use crate::exchange::dexnow::data_structures::instrument::Instrument;
use crate::exchange::dexnow::data_structures::token::Token;
use crate::exchange::exchange_update::ExchangeUpdate;
use crate::exchange::order_book::OrderBook;

pub struct Engine {
    pub(super) name: String,
    pub orderbook: Arc<RwLock<OrderBook>>,
    pub update_sender: Option<Sender<ExchangeUpdate>>,
    pub version: u8,
    pub connection: RpcClient,
    pub program_id: Pubkey,
    pub dexnow_authority: Pubkey,
    pub root_account: Pubkey,
    pub distrib_account: Pubkey,
    pub community_account: Pubkey,
    pub lut_account: Option<Pubkey>,
    pub wallet: Option<Keypair>,
    pub original_client_id: Option<u64>,
    pub client_primary_account: Option<Pubkey>,
    pub client_drv_account: Option<Pubkey>,
    pub client_dexnow_account: Option<Pubkey>,
    pub client_lut_account: Option<Pubkey>,
    pub tokens: std::collections::HashMap<u64, Token>,
    pub instruments: std::collections::HashMap<u64, Instrument>,
}

impl Engine {
    pub fn new(
        connection: RpcClient,
        root_account: Pubkey,
        name: String,
        program_id: Pubkey,
        orderbook: Arc<RwLock<OrderBook>>,
    ) -> Self {
        let dexnow_authority = Pubkey::find_program_address(&[b"ndxnt"], &program_id).0;

        Engine {
            name,
            update_sender: None,
            orderbook,
            version: 1,
            connection,
            program_id,
            dexnow_authority,
            root_account,
            distrib_account: Pubkey::default(),
            community_account: Pubkey::default(),
            lut_account: None,
            wallet: None,
            original_client_id: None,
            client_primary_account: None,
            client_drv_account: None,
            client_dexnow_account: None,
            client_lut_account: None,
            tokens: std::collections::HashMap::new(),
            instruments: std::collections::HashMap::new(),
        }
    }
}



