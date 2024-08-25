use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Debug;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Local, TimeDelta};
use crate::arbitrage_manager::arbitrage_manager::ArbitrageManager;
use crate::exchange::order_book::{TPrice};
use colored::Colorize;

const MAX_ORDERBOOK_TIME_GAP: TimeDelta = TimeDelta::milliseconds(500);
const MAX_CURRENT_TIME_GAP: Duration = Duration::from_millis(300);

impl ArbitrageManager {
    pub(crate) async fn analyze_opportunities(&self) {
        if let Some(opportunity) = self.check_local_opportunities() {
            // println!("Found arbitrage opportunity in local index: {:?}", opportunity);
            if let Some(confirmed_opportunity) = self.confirm_opportunity(opportunity).await {
                println!("{}", confirmed_opportunity);
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

        let buy_orderbook_ptr = buy_exchange.get_order_book();
        let sell_orderbook_ptr = sell_exchange.get_order_book();

        let buy_orderbook = buy_orderbook_ptr.read().await;
        let sell_orderbook = sell_orderbook_ptr.read().await;

        // *** Check orderbook time gap ***
        let time_gap = buy_orderbook.last_updated.signed_duration_since(sell_orderbook.last_updated);

        if time_gap > MAX_ORDERBOOK_TIME_GAP {
            // println!("Orderbook time gap too large: {:?}", time_gap);
            return None;
        }

        // *** Check current time gap ***
        let now = SystemTime::now();
        let current_gap = match now.duration_since(SystemTime::from(buy_orderbook.last_updated)) {
            Ok(duration) => duration,
            Err(_) => {
                // println!("Error: Current time is earlier than orderbook update time");
                return None;
            }
        };

        if current_gap > MAX_CURRENT_TIME_GAP {
            // println!("Current time gap too large: {:?}", current_gap);
            return None;
        }

        let mut total_volume = 0.0;
        let mut total_profit = 0.0;

        let mut ask_volumes: BTreeMap<TPrice, f64> = buy_orderbook.asks.clone();
        let mut bid_volumes: BTreeMap<TPrice, f64> = sell_orderbook.bids.clone();

        while let (Some((&ask_price, ask_volume)), Some((&bid_price, bid_volume))) = (ask_volumes.iter().next(), bid_volumes.iter().next_back()) {
            if ask_price >= bid_price {
                break;
            }

            let volume = ask_volume.min(*bid_volume);
            let profit = (bid_price.into_inner() - ask_price.into_inner()) * volume;

            total_volume += volume;
            total_profit += profit;

            if *ask_volume > *bid_volume {
                ask_volumes.insert(ask_price, ask_volume - bid_volume);
                bid_volumes.remove(&bid_price);
            } else if *ask_volume < *bid_volume {
                bid_volumes.insert(bid_price, bid_volume - ask_volume);
                ask_volumes.remove(&ask_price);
            } else {
                ask_volumes.remove(&ask_price);
                bid_volumes.remove(&bid_price);
            }
        }

        if total_volume > 0.0 {
            Some(ConfirmedArbitrageOpportunity {
                buy_exchange: opportunity.buy_exchange,
                sell_exchange: opportunity.sell_exchange,
                buy_price: *buy_orderbook.asks.first_key_value()?.0,
                sell_price: *sell_orderbook.bids.last_key_value()?.0,
                volume: total_volume,
                estimated_profit: total_profit,
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
    volume: f64,
    estimated_profit: f64,
}

impl fmt::Display for ArbitrageOpportunity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (buy) -> {} (sell) | Price: {} -> {}",
               self.buy_exchange.blue(),
               self.sell_exchange.blue(),
               self.buy_price.to_string().yellow(),
               self.sell_price.to_string().yellow()
        )
    }
}

impl fmt::Display for ConfirmedArbitrageOpportunity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timestamp: DateTime<Local> = Local::now();
        let profit_percentage = (self.estimated_profit / (self.buy_price.into_inner() * self.volume) * 100.0).abs();

        writeln!(f, "{}", "Confirmed Arbitrage Opportunity".green().bold())?;
        writeln!(f, "{}", "=================================".green())?;
        writeln!(f, "Time: {}", timestamp.format("%Y-%m-%d %H:%M:%S%.3f %Z").to_string().cyan())?;
        writeln!(f, "Buy Exchange: {}", self.buy_exchange.blue())?;
        writeln!(f, "Sell Exchange: {}", self.sell_exchange.blue())?;
        writeln!(f, "Buy Price: {}", self.buy_price.to_string().yellow())?;
        writeln!(f, "Sell Price: {}", self.sell_price.to_string().yellow())?;
        writeln!(f, "Volume: {:.8} units", self.volume.to_string().magenta())?;
        writeln!(f, "Estimated Profit: {:.8}", self.estimated_profit.to_string().green())?;
        writeln!(f, "Profit Percentage: {:.2}%", profit_percentage.to_string().green())?;
        writeln!(f, "{}", "=================================".green())
    }
}