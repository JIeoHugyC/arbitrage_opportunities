use serde::Serialize;

/// Represents a request to subscribe to a WebSocket stream
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeRequest {
    /// The operation type
    pub op: String,
    /// List of topics to subscribe to
    pub args: Vec<String>,
}

impl SubscribeRequest {
    /// Creates a new subscription request for the given symbols
    pub(crate) fn new(symbols: &[String]) -> Self {
        let depth = 1;
        let args = symbols
            .iter()
            .map(|symbol| format!("orderbook.{}.{}", depth, symbol))
            .collect();

        SubscribeRequest {
            op: "subscribe".to_string(),
            args,
        }
    }
}