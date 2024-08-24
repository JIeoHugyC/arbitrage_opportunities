use crate::exchange::dexnow::engine::Engine;

impl Engine {
    pub fn token_dec(&self, token_id: u64) -> f64 {
        let token = self.tokens.get(&token_id).expect("Token not found");
        10f64.powi(token.decimals as i32)
    }
}