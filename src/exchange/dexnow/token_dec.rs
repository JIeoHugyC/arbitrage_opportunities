use crate::exchange::dexnow::dexnow_engine::DEXnowEngine;

impl DEXnowEngine {
    pub fn token_dec(&self, token_id: u64) -> f64 {
        let token = self.tokens.get(&token_id).expect("Token not found");
        10f64.powi(token.decimals as i32)
    }
}