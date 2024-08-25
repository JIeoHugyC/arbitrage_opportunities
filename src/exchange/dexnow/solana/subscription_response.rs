use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SubscriptionResponse {
    pub jsonrpc: String,
    pub result: u64,
    pub id: u64,
}