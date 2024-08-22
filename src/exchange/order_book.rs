use std::collections::BTreeMap;
use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;

pub(super) type TPrice = OrderedFloat<f64>;
pub(super) type TVolume = f64;

/// An order book for a single market pair
/// It maintains both buy and sell orders in sorted order of price
pub struct OrderBook {
    pub bids: BTreeMap<TPrice, TVolume>,
    pub asks: BTreeMap<TPrice, TVolume>,
    /// The timestamp from the match engine when this orderbook data was produced
    pub cts: DateTime<Utc>,
    /// Cross sequence
    pub sequence: u64,
}

impl OrderBook {
    pub(crate) fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            cts: Utc::now(),
            sequence: 0,
        }
    }

    /// Returns the best bid price and volume
    pub fn get_best_bid(&self) -> Option<TPrice> {
        self.bids.keys().next_back().cloned()
    }

    /// Returns the best ask price and volume
    pub fn get_best_ask(&self) -> Option<TPrice> {
        self.asks.keys().next().cloned()
    }
}