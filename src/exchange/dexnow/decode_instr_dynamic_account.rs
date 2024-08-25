use solana_sdk::account::Account;
use crate::exchange::dexnow::data_structures::constants::*;
use crate::exchange::dexnow::data_structures::instr_dynamic_account::InstrDynamicAccount;
use crate::exchange::dexnow::data_structures::line_px::LinePx;
use crate::exchange::dexnow::engine::Engine;
use crate::exchange::dexnow::utils::read_basic_types::{read_u32, read_i64};

impl Engine {
    pub fn decode_instr_dynamic_account(&self, data: &Vec<u8>) -> InstrDynamicAccount {

        let instr_id = read_u32(data, INSTR_DYNAMIC_ACCOUNT_ID_OFFSET) as u64;
        let instr = self.instruments.get(&instr_id).expect("Instrument not found");
        let asset_token_dec = self.token_dec(instr.asset_token_id);

        let mut spot_bids = Vec::new();
        let mut spot_asks = Vec::new();

        for i in 0..MARKET_DEPTH {
            // *** Read spot bid ***
            let offset = INSTR_DYNAMIC_ACCOUNT_BIDS_OFFSET + i * 16;
            let px = read_i64(data, offset) as f64 / DEC as f64;
            if px == 0.0 {
                break;
            }
            let qty = read_i64(data, offset + 8) as f64 / asset_token_dec;

            spot_bids.push(LinePx {
                px: (px * 1e9).round() / 1e9,
                qty: (qty * 1e9).round() / 1e9,
            });

            // *** Read spot ask ***
            let offset = INSTR_DYNAMIC_ACCOUNT_ASKS_OFFSET + i * 16;
            let px = read_i64(data, offset) as f64 / DEC as f64;
            if px == 0.0 {
                break;
            }
            let qty = read_i64(data, offset + 8) as f64 / asset_token_dec;

            spot_asks.push(LinePx {
                px: (px * 1e9).round() / 1e9,
                qty: (qty * 1e9).round() / 1e9,
            });
        }

        InstrDynamicAccount {
            spot_bids,
            spot_asks,
        }
    }
}