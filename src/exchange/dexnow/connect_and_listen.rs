use std::env;
use std::time::{Duration, Instant};
use tokio::time::interval;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentLevel;
use tokio::sync::mpsc::Sender;
use crate::exchange::dexnow::engine::Engine;
use crate::exchange::dexnow::solana::account_notification::AccountNotification;
use crate::exchange::dexnow::solana::account_subscribe::SubscribeMessage;
use crate::exchange::dexnow::solana::subscription_response::SubscriptionResponse;
use crate::exchange::exchange_update::ExchangeUpdate;

const PING_INTERVAL: Duration = Duration::from_secs(1);
const PONG_TIMEOUT: Duration = Duration::from_secs(5);

impl Engine {
    pub async fn connect_and_listen(
        &self,
        account_pubkey: &Pubkey,
        order_book_update_sender: &Sender<ExchangeUpdate>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_url = env::var("SOLANA_WS_URL").expect("SOLANA_WS_URL must be set");

        loop {
            let dyn_acc = self.connection.get_account(&account_pubkey).await;
            if let Ok(dyn_acc) = dyn_acc {
                let dyn_data = self.decode_instr_dynamic_account(&dyn_acc.data);
                println!("Dynamic account data: {:?}", dyn_data);
            }
            println!("Connecting to Solana WebSocket to listen DEXnow account {}...", account_pubkey);
            let (ws_stream, _) = connect_async(&ws_url).await?;
            let (mut write, mut read) = ws_stream.split();

            let subscribe_message = SubscribeMessage::new(account_pubkey, CommitmentLevel::Confirmed);
            let json_message = serde_json::to_string(&subscribe_message).unwrap();

            write.send(Message::Text(json_message)).await?;
            println!("Subscribed to account: {}", account_pubkey);

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
                                    println!("Subscription confirmed: {:?}", subscription);
                                } else if let Ok(notification) = serde_json::from_str::<AccountNotification>(&text) {
                                    println!("Account notification: {:?}", notification.params.subscription);
                                    let account_data = notification.params.result.value.data;
                                    let decoded_account = self.decode_instr_dynamic_account(&account_data);
                                    println!("Decoded account: {:?}", decoded_account);

                                } else {
                                    println!("Received unknown message format");
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
                    println!("No pong received within timeout, reconnecting...");
                    break;
                }
            }

            // If we're here, it means the connection was closed or an error occurred
            // Wait for a short time before attempting to reconnect
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Attempting to reconnect to DEXnow WebSocket...");
        }
    }
}