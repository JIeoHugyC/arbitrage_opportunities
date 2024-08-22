use std::collections::HashMap;
use std::sync::Arc;
use crate::exchange::exchange::Exchange;
use crate::exchange::exchange_update::{BestPrices};
use crate::trading_pair::ETradingPair;

/// An arbitrage manager that manages multiple exchanges and analyzes for opportunities
pub struct ArbitrageManager {
    /// List of exchanges to be managed in the arbitrage manager
    pub(super) exchanges: Vec<Arc<dyn Exchange>>,
    /// Cache best prices for each exchange
    pub(super) best_prices: HashMap<String, BestPrices>,
    /// The trading pair for which the arbitrage manager is managing exchanges
    pub(super) trading_pair: ETradingPair,
}

impl ArbitrageManager {
    pub(crate) fn new(trading_pair: ETradingPair) -> Self {
        ArbitrageManager {
            exchanges: Vec::new(),
            best_prices: HashMap::new(),
            trading_pair,
        }
    }
}