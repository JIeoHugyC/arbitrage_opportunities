#![allow(dead_code)]
/// Contains data about orderbook line
#[derive(Debug)]
pub struct LinePx {
    /// Price
    pub px: f64,
    /// Quantity
    pub qty: f64,
}