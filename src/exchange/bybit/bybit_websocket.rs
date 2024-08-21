use std::env;
use std::time::{Duration, Instant};
use tokio::time::interval;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use crate::exchange::bybit::ws_ping::PingMessage;
use crate::exchange::bybit::ws_pong::PongMessage;
use crate::exchange::bybit::ws_spot_orderbook::OrderBookUpdate;
use crate::exchange::bybit::ws_spot_subscribe::SubscribeRequest;
use crate::exchange::bybit::ws_subscribe_response::SubscribeResponse;

const PING_INTERVAL: Duration = Duration::from_secs(20);
const PONG_TIMEOUT: Duration = Duration::from_secs(5);

pub struct BybitWebSocket {}

impl BybitWebSocket {
    pub async fn connect_and_listen(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = env::var("BYBIT_WS_URL").expect("BYBIT_WS_URL must be set");
        let instrument = env::var("TRADING_INSTRUMENT").expect("TRADING_INSTRUMENT must be set");

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
                            if let Ok(pong) = serde_json::from_str::<PongMessage>(&text) {
                                last_pong = Instant::now();
                            } else if let Ok(subscribe_response) = serde_json::from_str::<SubscribeResponse>(&text) {
                                println!("Subscription response: {:?}", subscribe_response);
                            } else if let Ok(orderbook_update) = serde_json::from_str::<OrderBookUpdate>(&text) {
                                // Process the orderbook update
                                self.process_orderbook_update(orderbook_update);
                            } else {
                                println!("Received unknown message: {}", text);
                            }
                        }
                        Message::Ping(_) => {
                            write.send(Message::Pong(vec![])).await?;
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

    fn process_orderbook_update(&self, update: OrderBookUpdate) {
        // Implement orderbook update processing logic here
        println!("Received orderbook update: {:?}", update);
    }
}