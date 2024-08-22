use crate::arbitrage_manager::arbitrage_manager::ArbitrageManager;

impl ArbitrageManager {
    /// Analyze for opportunities in the arbitrage manager based on the best prices of the added exchanges
    pub(crate) async fn analyze_opportunities(&self) {
        println!("Analyzing opportunities for {:?}...\n", self.trading_pair);
        println!("Best prices: {:?}", self.best_prices);
    }
}