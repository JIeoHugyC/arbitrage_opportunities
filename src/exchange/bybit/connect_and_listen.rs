use std::env;
use std::time::{Duration, Instant};
use tokio::time::interval;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use crate::exchange::bybit::bybit_exchange::BybitExchange;
use crate::exchange::bybit::ws_ping::PingMessage;
use crate::exchange::bybit::ws_pong::PongMessage;
use crate::exchange::bybit::ws_spot_orderbook::{OrderBookUpdate};
use crate::exchange::bybit::ws_spot_subscribe::SubscribeRequest;
use crate::exchange::bybit::ws_subscribe_response::SubscribeResponse;
use crate::exchange::exchange_update::{ExchangeUpdate};
use crate::trading_pair::ETradingPair;

const PING_INTERVAL: Duration = Duration::from_secs(1);
const PONG_TIMEOUT: Duration = Duration::from_secs(5);

impl BybitExchange {
    pub async fn connect_and_listen(
        &self,
        trading_pair: &ETradingPair,
        order_book_update_sender: &Sender<ExchangeUpdate>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = env::var("BYBIT_WS_URL").expect("BYBIT_WS_URL must be set");
        let instrument = match trading_pair {
            ETradingPair::BtcUsdc => "BTCUSDC".to_string(),
            ETradingPair::SolUsdc => "SOLUSDC".to_string(),
        };

        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Subscribe to the orderbook
        let subscribe_request = SubscribeRequest::new(&[instrument]);
        write.send(Message::Text(serde_json::to_string(&subscribe_request)?)).await?;

        let mut ping_interval = interval(PING_INTERVAL);
        let mut last_pong = Instant::now();

        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    let ping = PingMessage::new(None);
                    write.send(Message::Text(serde_json::to_string(&ping)?)).await?;
                }
                Some(message) = read.next() => {
                    match message? {
                        Message::Text(text) => {
                            if let Ok(orderbook_update) = serde_json::from_str::<OrderBookUpdate>(&text) {
                                self.process_orderbook_update(order_book_update_sender, orderbook_update).await;
                            } else if let Ok(_pong) = serde_json::from_str::<PongMessage>(&text) {
                                last_pong = Instant::now();
                            } else if let Ok(subscribe_response) = serde_json::from_str::<SubscribeResponse>(&text) {
                                println!("Subscribed to: {:?}", subscribe_response);
                            } else {
                                println!("Received unknown message: {}", text);
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

        Ok(())
    }
}