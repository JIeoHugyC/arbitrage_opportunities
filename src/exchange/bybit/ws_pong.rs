use serde::Deserialize;

/// Represents a pong message received from the server
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PongMessage {
    /// Indicates whether the ping was successful
    pub success: bool,
    /// The operation type, should be "pong"
    pub ret_msg: String,
    /// A unique identifier for the connection
    pub conn_id: String,
    /// The operation type, should be "ping"
    pub op: String,
}