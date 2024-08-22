use tokio::sync::mpsc;
use crate::arbitrage_manager::arbitrage_manager::ArbitrageManager;

const MESSAGE_BUFFER_SIZE: usize = 100;

impl ArbitrageManager {

    /// Start the arbitrage manager and continuously update the order books for the added exchanges
    pub(crate) async fn run(&mut self) {
        let (tx, mut rx) = mpsc::channel(MESSAGE_BUFFER_SIZE);

        for exchange in &self.exchanges {
            let exchange_clone = exchange.1.clone();
            let tx_clone = tx.clone();
            let trading_pair = self.trading_pair.clone();
            tokio::spawn(async move {
                exchange_clone.start(trading_pair, tx_clone).await;
            });
        }

        while let Some(update) = rx.recv().await {
            self.best_prices.insert(update.exchange_name, update.best_prices);
            self.analyze_opportunities().await;
        }
    }
}