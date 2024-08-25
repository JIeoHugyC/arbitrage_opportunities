use chrono::{Utc};
use ordered_float::{OrderedFloat};
use thiserror::Error;
use crate::exchange::dexnow::data_structures::instr_dynamic_account::InstrDynamicAccount;
use crate::exchange::dexnow::dexnow_engine::DEXnowEngine;
use crate::exchange::order_book::{OrderBook};
use crate::exchange::send_orderbook_update::send_orderbook_update;

#[derive(Error, Debug)]
pub enum ProcessOrderbookError {
    #[error("Update sender not initialized")]
    UpdateSenderNotInitialized,
}

impl DEXnowEngine {
    pub async fn process_orderbook_update(
        &self,
        instr_dynamic_account: &InstrDynamicAccount,
        slot: u64,
    ) -> Result<(), Box<dyn std::error::Error>>
    {
        let update_sender = self.update_sender.as_ref()
            .ok_or(ProcessOrderbookError::UpdateSenderNotInitialized)?;
        // println!("[INFO][DEXnow] Received orderbook update: {:?}", instr_dynamic_account);
        let mut orderbook = self.orderbook.write().await;
        // Zero slot means a new orderbook (got from HTTP request instead of WS)
        // Clear the current orderbook and start from the provided slot.
        if slot == 0 {
            println!("[INFO][DEXnow] Created new orderbook");
            *orderbook = OrderBook::new();
        } else if slot < orderbook.sequence {
            println!("[WARNING][DEXnow] Skipping outdated update: received slot {}, current slot {}", slot, orderbook.sequence);
            return Ok(());
        } else {
            orderbook.sequence = slot;
            // println!("[INFO][DEXnow] Applied orderbook {slot}");
        }
        orderbook.last_updated = Utc::now();
        orderbook.asks.clear();
        orderbook.bids.clear();
        for line_px in instr_dynamic_account.spot_asks.iter() {
            let price = OrderedFloat(line_px.px);
            orderbook.asks.insert(price, line_px.qty);
        }
        for line_px in instr_dynamic_account.spot_bids.iter() {
            let price = OrderedFloat(line_px.px);
            orderbook.bids.insert(price, line_px.qty);
        }

        // *** Notify subscribers about the updated orderbook ***
        send_orderbook_update(
            &update_sender,
            &self.name,
            &orderbook.get_best_bid(),
            &orderbook.get_best_ask(),
        ).await;

        Ok(())
    }
}