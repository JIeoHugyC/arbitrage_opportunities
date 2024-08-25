use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubscriptionResponse {
    pub jsonrpc: String,
    pub result: u64,
    pub id: u64,
}