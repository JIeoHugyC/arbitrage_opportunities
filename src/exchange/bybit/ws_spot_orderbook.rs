use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

/// Order book update type
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UpdateType {
    Snapshot,
    Delta,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookUpdate {
    /// Topic name
    pub topic: String,
    /// The timestamp when the system generates this update message
    #[serde(with = "timestamp_ms")]
    pub ts: DateTime<Utc>,
    /// Data type. `snapshot`,`delta`
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    /// The actual order book data
    pub data: OrderBookData,
    /// The timestamp from the match engine when this orderbook data was produced
    #[serde(with = "timestamp_ms")]
    pub cts: DateTime<Utc>,
}

/// Represents the data of an order book update
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookData {
    /// Symbol name
    pub s: String,
    /// Bids. For `snapshot` stream, the element is sorted by price in descending order
    pub b: Vec<PriceLevel>,
    /// Asks. For `snapshot` stream, the element is sorted by price in ascending order
    pub a: Vec<PriceLevel>,
    /// Update ID. Is a sequence. Occasionally, you'll receive "u"=1, which is a snapshot data due to the restart of the service. So please overwrite your local orderbook
    pub u: u64,
    /// Cross sequence
    /// You can use this field to compare different levels orderbook data, and for the smaller seq, then it means the data is generated earlier.
    pub seq: u64,
}

/// Represents a price level in the order book
///
/// The first element (index 0) is the price
/// The second element (index 1) is the size (amount)
#[derive(Debug)]
pub struct PriceLevel(pub f64, pub f64);

impl<'de> Deserialize<'de> for PriceLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (price_str, size_str): (String, String) = Deserialize::deserialize(deserializer)?;

        let parse_float = |s: &str, field: &str| {
            f64::from_str(s).map_err(|e| {
                serde::de::Error::custom(format!("Failed to parse {} '{}' as f64: {}", field, s, e))
            })
        };

        let price = parse_float(&price_str, "price")?;
        let size = parse_float(&size_str, "size")?;

        Ok(PriceLevel(price, size))
    }
}

mod timestamp_ms {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms = i64::deserialize(deserializer)?;
        Utc.timestamp_millis_opt(ms)
            .single()
            .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))
    }
}

impl PriceLevel {
    /// Returns the price of this price level
    ///
    /// Bid price or Ask price
    pub fn price(&self) -> &f64 {
        &self.0
    }

    /// Returns the size (amount) at this price level
    ///
    /// Bid size or Ask size
    /// The delta data has size=0, which means that all quotations for this price have been filled or cancelled
    pub fn size(&self) -> &f64 {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use super::*;
    use serde_json;

    #[test]
    fn test_orderbook_update_deserialization() {
        let json_data = r#"
        {
            "topic":"orderbook.50.BTCUSDC",
            "ts":1724318672920,
            "type":"snapshot",
            "data":{
                "s":"BTCUSDC",
                "b":[["60938.3","0.016796"],["60936","0.0249"]],
                "a":[["60947.94","0.010144"],["60947.95","0.010144"]],
                "u":45468681,
                "seq":46063562942
            },
            "cts":1724318672915
        }
        "#;

        let result = serde_json::from_str::<OrderBookUpdate>(json_data);

        assert!(result.is_ok(), "Failed to parse JSON: {:?}", result.err());

        let update = result.unwrap();
        assert_eq!(update.topic, "orderbook.50.BTCUSDC");
        let expected_ts = Utc.timestamp_millis_opt(1724318672920).unwrap();
        assert_eq!(update.ts, expected_ts, "Incorrect system timestamp (ts)");

        let expected_cts = Utc.timestamp_millis_opt(1724318672915).unwrap();
        assert_eq!(update.cts, expected_cts, "Incorrect match engine timestamp (cts)");

        assert_eq!(update.update_type, UpdateType::Snapshot);

        let data = update.data;
        assert_eq!(data.s, "BTCUSDC");
        assert_eq!(data.u, 45468681);
        assert_eq!(data.seq, 46063562942);

        assert_eq!(data.b.len(), 2);
        assert_eq!(data.b[0].price(), &60938.3);
        assert_eq!(data.b[0].size(), &0.016796);

        assert_eq!(data.a.len(), 2);
        assert_eq!(data.a[0].price(), &60947.94);
        assert_eq!(data.a[0].size(), &0.010144);
    }

    #[test]
    fn test_orderbook_update_deserialization_delta() {
        let json_data = r#"
        {
            "topic":"orderbook.50.BTCUSDC",
            "ts":1724318672920,
            "type":"delta",
            "data":{
                "s":"BTCUSDC",
                "b":[["60938.3","0.016796"],["60936","0.0249"]],
                "a":[["60947.94","0.010144"],["60947.95","0.010144"]],
                "u":45468681,
                "seq":46063562942
            },
            "cts":1724318672915
        }
        "#;

        let result = serde_json::from_str::<OrderBookUpdate>(json_data);

        assert!(result.is_ok(), "Failed to parse JSON: {:?}", result.err());

        let update = result.unwrap();
        assert_eq!(update.update_type, UpdateType::Delta);
    }

    #[test]
    fn test_orderbook_update_deserialization_invalid_type() {
        let json_data = r#"
        {
            "topic":"orderbook.50.BTCUSDC",
            "ts":1724318672920,
            "type":"invalid",
            "data":{
                "s":"BTCUSDC",
                "b":[["60938.3","0.016796"],["60936","0.0249"]],
                "a":[["60947.94","0.010144"],["60947.95","0.010144"]],
                "u":45468681,
                "seq":46063562942
            },
            "cts":1724318672915
        }
        "#;

        let result = serde_json::from_str::<OrderBookUpdate>(json_data);

        assert!(result.is_err(), "Expected an error, but parsing succeeded");
        assert!(result.unwrap_err().to_string().contains("unknown variant `invalid`"), "Unexpected error message");
    }
}