use std::env;
use std::time::{Duration};
use serde_json;


const RECONNECT_DELAY: Duration = Duration::from_secs(5);
const PING_INTERVAL: Duration = Duration::from_secs(20);

pub struct BybitWebSocket {}

impl BybitWebSocket {
    pub async fn connect_and_listen(&self) {
        let url = env::var("BYBIT_WS_URL").expect("BYBIT_WS_URL must be set");
        let instrument = env::var("TRADING_INSTRUMENT").expect("TRADING_INSTRUMENT must be set");
    }
}