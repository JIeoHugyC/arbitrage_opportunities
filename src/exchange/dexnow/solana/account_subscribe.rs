use serde::Serialize;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::pubkey::Pubkey;

#[derive(Serialize)]
pub struct SubscribeMessage {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Params,
}

type TAccountKey = String;

#[derive(Serialize)]
struct SubscribeConfig {
    encoding: String,
    commitment: CommitmentLevel,
}

#[derive(Serialize)]
struct Params(TAccountKey, SubscribeConfig);


impl SubscribeMessage {
    pub fn new(account_pubkey: &Pubkey, commitment_level: CommitmentLevel) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "accountSubscribe".to_string(),
            params: Params(
                account_pubkey.to_string(),
                SubscribeConfig {
                    encoding: "base64".to_string(),
                    commitment: commitment_level,
                },
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_subscribe_message_serialization() {
        let pubkey = Pubkey::new_unique();

        let message = SubscribeMessage::new(&pubkey, CommitmentLevel::Confirmed);

        let serialized = serde_json::to_value(&message).unwrap();

        let expected = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "accountSubscribe",
            "params": [
                pubkey.to_string(),
                {
                    "encoding": "base64",
                    "commitment": "confirmed"
                }
            ]
        });

        assert_eq!(serialized, expected);
    }
}