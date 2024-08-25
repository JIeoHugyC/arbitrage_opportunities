use std::env;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::interval;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentLevel;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use crate::exchange::dexnow::engine::Engine;
use crate::exchange::dexnow::solana::account_notification::AccountNotification;
use crate::exchange::dexnow::solana::account_subscribe::SubscribeMessage;
use crate::exchange::dexnow::solana::subscription_response::SubscriptionResponse;
use crate::exchange::exchange_update::ExchangeUpdate;
use crate::exchange::order_book::OrderBook;

const PING_INTERVAL: Duration = Duration::from_secs(1);
const PONG_TIMEOUT: Duration = Duration::from_secs(5);

impl Engine {
    pub async fn connect_and_listen(
        &self,
        account_pubkey: &Pubkey
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_url = env::var("SOLANA_WS_URL").expect("SOLANA_WS_URL must be set");

        loop {
            let dyn_acc = self.connection.get_account(&account_pubkey).await;
            if let Ok(dyn_acc) = dyn_acc {
                let dyn_data = self.decode_instr_dynamic_account(&dyn_acc.data);
                let _ = self.process_orderbook_update(&dyn_data, 0).await;
            }
            println!("[INFO][DEXnow] Connecting to Solana WebSocket to listen account {}...", account_pubkey);
            let (ws_stream, _) = connect_async(&ws_url).await?;
            let (mut write, mut read) = ws_stream.split();

            let subscribe_message = SubscribeMessage::new(account_pubkey, CommitmentLevel::Confirmed);
            let json_message = serde_json::to_string(&subscribe_message).unwrap();

            write.send(Message::Text(json_message)).await?;
            println!("[INFO][DEXnow] Subscribed to account: {}", account_pubkey);

            let mut ping_interval = interval(PING_INTERVAL);
            let mut last_pong = Instant::now();
            let mut ping_msg: u8 = 0;

            loop {
                tokio::select! {
                    _ = ping_interval.tick() => {
                        ping_msg = ping_msg.wrapping_add(1);
                        write.send(Message::Ping(vec![ping_msg])).await?;
                    }
                    Some(message) = read.next() => {
                        match message? {
                            Message::Text(text) => {
                                if let Ok(subscription) = serde_json::from_str::<SubscriptionResponse>(&text) {
                                    println!("[INFO][DEXnow] Subscription confirmed: {:?}", subscription);
                                } else if let Ok(notification) = serde_json::from_str::<AccountNotification>(&text) {
                                    // println!("Account notification: {:?}", notification.params.subscription);
                                    let account_data = notification.params.result.value.data;
                                    let decoded_account = self.decode_instr_dynamic_account(&account_data);
                                    let _ = self.process_orderbook_update(&decoded_account, notification.params.result.context.slot).await;
                                } else {
                                    println!("[WARNING][DEXnow] Received unknown message format");
                                }
                            }
                            Message::Pong(response) => {
                                if response.len() > 0 && response[0] == ping_msg {
                                    last_pong = Instant::now();
                                    continue;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if last_pong.elapsed() > PONG_TIMEOUT {
                    println!("[ERROR][DEXnow] No pong received within timeout, reconnecting...");
                    break;
                }
            }

            // If we're here, it means the connection was closed or an error occurred
            // Wait for a short time before attempting to reconnect
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("[INFO][DEXnow] Attempting to reconnect to DEXnow WebSocket...");
        }
    }
}