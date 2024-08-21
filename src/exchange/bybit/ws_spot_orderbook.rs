use std::str::FromStr;
use serde::{Deserialize, Deserializer};

/// Represents an update to the order book
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookUpdate {
    /// Topic name
    pub topic: String,
    /// Data type. `snapshot`,`delta`
    pub type_: String,
    /// The timestamp (ms) that the system generates the data
    pub ts: u64,
    /// The actual order book data
    pub data: OrderBookData,
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
    /// The timestamp from the match engine when this orderbook data is produced. It can be correlated with `T` from public trade channel
    pub cts: u64,
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
        let price = f64::from_str(&price_str).map_err(serde::de::Error::custom)?;
        let size = f64::from_str(&size_str).map_err(serde::de::Error::custom)?;
        Ok(PriceLevel(price, size))
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