use std::collections::BTreeMap;

type TPrice = f64;
type TVolume = f64;

/// An order book for a single market pair
/// It maintains both buy and sell orders in sorted order of price
pub struct OrderBook {
    bids: BTreeMap<TPrice, TVolume>,
    asks: BTreeMap<TPrice, TVolume>,
}

impl OrderBook {
    pub(crate) fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
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