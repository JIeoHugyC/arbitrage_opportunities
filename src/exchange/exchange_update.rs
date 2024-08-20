/// Best prices for a specific exchange
pub struct BestPrices {
    pub best_bid: f64,
    pub best_ask: f64,
}

/// Represents an update to the best prices of an exchange
pub struct ExchangeUpdate {
    pub exchange_name: String,
    pub best_prices: BestPrices,
}