use serde::Deserialize;
use base64::Engine;
use base64::engine::general_purpose;

#[derive(Debug, Deserialize)]
pub struct AccountNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Params,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub result: Result,
    pub subscription: u64,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    pub context: Context,
    pub value: Value,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    pub slot: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    #[serde(deserialize_with = "deserialize_account_data")]
    pub data: Vec<u8>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    pub rent_epoch: u64,
    pub space: u64,
}

fn deserialize_account_data<'de, D>(deserializer: D) -> std::result::Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Helper {
        data: String,
        encoding: String,
    }

    let helper = Helper::deserialize(deserializer)?;
    match helper.encoding.as_str() {
        "base64" => general_purpose::STANDARD
            .decode(&helper.data).map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom(format!("Unsupported encoding: {}", helper.encoding))),
    }
}