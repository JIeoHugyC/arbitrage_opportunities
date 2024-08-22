use serde::Serialize;

/// Represents a ping message for the server
#[derive(Debug, Serialize)]
pub struct PingMessage {
    /// The operation type
    pub op: String,
    /// Customised ID, which is optional
    pub req_id: String,
}

impl PingMessage {
    /// Creates a new ping message with the given operation type and optional request ID
    pub fn new(req_id: Option<String>) -> Self {
        PingMessage {
            op: "ping".to_string(),
            req_id: req_id.unwrap_or_else(|| String::new()),
        }
    }
}