pub struct BestPrices {
    pub best_bid: f64,
    pub best_ask: f64,
}

pub struct ExchangeUpdate {
    pub exchange_name: String,
    pub best_prices: BestPrices,
}