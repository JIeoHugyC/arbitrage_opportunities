#![allow(dead_code)]
/// Contains data about option strike
pub struct OptionStrike {
    /// Strike ID
    pub id: u64,
    /// Strike price
    pub price: f64,
    /// Calls open interest
    pub calls: f64,
    /// Puts open interest
    pub puts: f64,
    pub height: f64,
    pub call_px: Option<f64>,
    pub put_px: Option<f64>,
}