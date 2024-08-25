use tokio::sync::mpsc::Sender;
use crate::exchange::bybit::ws_spot_orderbook::OrderBookUpdate;
use crate::exchange::dexnow::data_structures::instr_dynamic_account::InstrDynamicAccount;
use crate::exchange::dexnow::engine::Engine;
use crate::exchange::exchange_update::ExchangeUpdate;

impl Engine {
    pub(crate) async fn process_orderbook_update(
        &self,
        order_book_update_sender: &Sender<ExchangeUpdate>,
        instr_dynamic_account: &InstrDynamicAccount)
    {

    }
}