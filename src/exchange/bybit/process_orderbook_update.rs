use std::collections::BTreeMap;
use ordered_float::{Float, OrderedFloat};
use tokio::sync::mpsc::Sender;
use crate::exchange::bybit::bybit_exchange::BybitExchange;
use crate::exchange::bybit::ws_spot_orderbook::{OrderBookUpdate, PriceLevel, UpdateType};
use crate::exchange::exchange_update::{ExchangeUpdate};
use crate::exchange::order_book::{OrderBook, TPrice, TVolume};
use crate::exchange::send_orderbook_update::send_orderbook_update;

impl BybitExchange {
    pub(crate) async fn process_orderbook_update(
        &self,
        update_sender: &Sender<ExchangeUpdate>,
        update: OrderBookUpdate)
    {
        let mut orderbook = self.orderbook.write().await;

        // Check if this update is newer than our current state
        if update.data.seq <= orderbook.sequence {
            println!("Skipping outdated update: received seq {}, current seq {}", update.data.seq, orderbook.sequence);
            return;
        }

        // println!("Processing orderbook update: {:?}", update.update_type);

        match update.update_type {
            UpdateType::Snapshot => {
                *orderbook = OrderBook::new();
                self.apply_updates(&mut orderbook.bids, &update.data.b);
                self.apply_updates(&mut orderbook.asks, &update.data.a);
            }
            UpdateType::Delta => {
                // For delta updates we can only apply the update if it's in the correct order
                if update.data.seq < orderbook.sequence {
                    println!("Warning: Received out-of-order delta update. Expected seq >= {}, got {}", orderbook.sequence + 1, update.data.seq);
                    return;
                }
                self.apply_updates(&mut orderbook.bids, &update.data.b);
                self.apply_updates(&mut orderbook.asks, &update.data.a);
            }
        }

        orderbook.sequence = update.data.seq;
        orderbook.last_updated = update.cts;

        // println!("Applied orderbook {:?} at {}. New seq: {}", update.update_type, update.ts, orderbook.sequence);
        // println!("Best bid: {:?}", orderbook.get_best_bid());
        // println!("Best ask: {:?}", orderbook.get_best_ask());

        // *** Notify subscribers about the updated orderbook ***
        send_orderbook_update(
            &update_sender,
            &self.name,
            orderbook.get_best_bid().unwrap_or(TPrice::min_value()),
            orderbook.get_best_ask().unwrap_or(TPrice::max_value()),
        ).await;
    }
    fn apply_updates(&self, side: &mut BTreeMap<TPrice, TVolume>, updates: &[PriceLevel]) {
        for price_level in updates {
            let price = OrderedFloat(*price_level.price());
            let amount = *price_level.size();

            if amount == 0.0 {
                side.remove(&price);
            } else {
                side.insert(price, amount);
            }
        }
    }
}