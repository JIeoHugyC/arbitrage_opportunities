use serde::Deserialize;

/// Represents the response to a subscription request
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeResponse {
    /// Indicates whether the subscription was successful
    pub success: bool,
    /// A message describing the result of the subscription attempt
    pub ret_msg: String,
    /// A unique identifier for the connection
    pub conn_id: String,
    /// The operation type, usually "subscribe"
    pub op: String,
    /// A unique identifier for the request
    pub req_id: String,
}
