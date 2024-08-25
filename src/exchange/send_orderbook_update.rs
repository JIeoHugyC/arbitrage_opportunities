use ordered_float::Float;
use tokio::sync::mpsc::Sender;
use crate::exchange::exchange_update::{BestPrices, ExchangeUpdate};
use crate::exchange::order_book::TPrice;

pub async fn send_orderbook_update(
    update_sender: &Sender<ExchangeUpdate>,
    exchange_name: &String,
    best_bid: &Option<TPrice>,
    best_ask: &Option<TPrice>,
) {
    let best_bid = best_bid.unwrap_or(TPrice::min_value());
    let best_ask = best_ask.unwrap_or(TPrice::max_value());
    update_sender.send(ExchangeUpdate {
        exchange_name: exchange_name.clone(),
        best_prices: BestPrices {
            best_bid,
            best_ask,
        },
    })
        .await
        .map_err(|e| {
            eprintln!("[ERROR][{exchange_name}] Failed to send order book update: {}", e);
        })
        .unwrap_or(());
}