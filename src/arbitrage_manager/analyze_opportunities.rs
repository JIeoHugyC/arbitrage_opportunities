use futures_util::AsyncReadExt;
use crate::arbitrage_manager::arbitrage_manager::ArbitrageManager;
use crate::exchange::order_book::{OrderBook, TPrice};

impl ArbitrageManager {
    pub(crate) async fn analyze_opportunities(&self) {
        if let Some(opportunity) = self.check_local_opportunities() {
            println!("Found arbitrage opportunity in local index: {:?}", opportunity);
            if let Some(confirmed_opportunity) = self.confirm_opportunity(opportunity).await {
                println!("Confirmed arbitrage opportunity: {:?}", confirmed_opportunity);
            }
        }
    }

    fn check_local_opportunities(&self) -> Option<ArbitrageOpportunity> {
        let mut best_bid: Option<(&String, &TPrice)> = None;
        let mut best_ask: Option<(&String, &TPrice)> = None;

        for (exchange, prices) in &self.best_prices {
            if best_bid.is_none() || prices.best_bid > *best_bid.unwrap().1 {
                best_bid = Some((exchange, &prices.best_bid));
            }
            if best_ask.is_none() || prices.best_ask < *best_ask.unwrap().1 {
                best_ask = Some((exchange, &prices.best_ask));
            }
        }

        if let (Some((buy_exchange, buy_price)), Some((sell_exchange, sell_price))) = (best_ask, best_bid) {
            if buy_price < sell_price {
                return Some(ArbitrageOpportunity {
                    buy_exchange: buy_exchange.clone(),
                    sell_exchange: sell_exchange.clone(),
                    buy_price: *buy_price,
                    sell_price: *sell_price,
                });
            }
        }
        None
    }

    async fn confirm_opportunity(&self, opportunity: ArbitrageOpportunity) -> Option<ConfirmedArbitrageOpportunity> {
        let buy_exchange = self.exchanges.get(&opportunity.buy_exchange)?;
        let sell_exchange = self.exchanges.get(&opportunity.sell_exchange)?;

        let buy_orderbook = buy_exchange.get_order_book();
        let sell_orderbook = sell_exchange.get_order_book();

        let buy_orderbook = buy_orderbook.read().await;
        let sell_orderbook = sell_orderbook.read().await;

        let best_ask = buy_orderbook.get_best_ask()?;
        let best_bid = sell_orderbook.get_best_bid()?;

        if best_ask < best_bid {
            Some(ConfirmedArbitrageOpportunity {
                buy_exchange: opportunity.buy_exchange,
                sell_exchange: opportunity.sell_exchange,
                buy_price: best_ask,
                sell_price: best_bid,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct ArbitrageOpportunity {
    buy_exchange: String,
    sell_exchange: String,
    buy_price: TPrice,
    sell_price: TPrice,
}

#[derive(Debug)]
struct ConfirmedArbitrageOpportunity {
    buy_exchange: String,
    sell_exchange: String,
    buy_price: TPrice,
    sell_price: TPrice,
    // available_volume: f64,
}