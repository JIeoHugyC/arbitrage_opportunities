use crate::exchange::order_book::TPrice;

/// Best prices for a specific exchange
#[derive(Debug)]
pub struct BestPrices {
    pub best_bid: Option<TPrice>,
    pub best_ask: Option<TPrice>,
}

/// Represents an update to the best prices of an exchange
#[derive(Debug)]
pub struct ExchangeUpdate {
    pub exchange_name: String,
    pub best_prices: BestPrices,
}