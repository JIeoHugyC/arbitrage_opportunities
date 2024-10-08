use crate::exchange::dexnow::data_structures::constants::*;
use crate::exchange::dexnow::data_structures::instr_dynamic_account::InstrDynamicAccount;
use crate::exchange::dexnow::data_structures::line_px::LinePx;
use crate::exchange::dexnow::dexnow_engine::DEXnowEngine;
use crate::exchange::dexnow::utils::read_basic_types::{read_u32, read_i64};

impl DEXnowEngine {
    pub fn decode_instr_dynamic_account(&self, data: &Vec<u8>) -> InstrDynamicAccount {
        let instr_id = read_u32(data, INSTR_DYNAMIC_ACCOUNT_ID_OFFSET) as u64;
        let instr = self.instruments.get(&instr_id).expect("Instrument not found");
        let asset_token_dec = self.token_dec(instr.asset_token_id);

        fn read_orders(data: &Vec<u8>, start_offset: usize, asset_token_dec: f64) -> Vec<LinePx> {
            let mut orders = Vec::new();
            for i in 0..MARKET_DEPTH {
                let offset = start_offset + i * 16;
                let px = read_i64(data, offset) as f64 / DEC as f64;
                if px == 0.0 {
                    break;
                }
                let qty = read_i64(data, offset + 8) as f64 / asset_token_dec;
                orders.push(LinePx {
                    px,
                    qty,
                });
            }
            orders
        }

        let spot_bids = read_orders(data, INSTR_DYNAMIC_ACCOUNT_BIDS_OFFSET, asset_token_dec);
        let spot_asks = read_orders(data, INSTR_DYNAMIC_ACCOUNT_ASKS_OFFSET, asset_token_dec);

        InstrDynamicAccount {
            spot_bids,
            spot_asks,
        }
    }
}