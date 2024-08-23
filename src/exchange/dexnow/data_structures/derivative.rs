#![allow(dead_code)]
use crate::exchange::dexnow::data_structures::futures::Futures;
use crate::exchange::dexnow::data_structures::option_strike::OptionStrike;

/// Contains general data about derivative
pub struct Derivative {
    /// Derivative instance ID
    pub instance_id: Option<u64>,
    /// Expiration date in seconds since epoch
    pub expiration: Option<u64>,
    /// Derivative minimum price tick size
    pub px_granular: Option<f64>,
    /// Derivative minimum quantity tick size
    pub contract_size: Option<f64>,
    pub options_cashflow: Option<f64>,
    pub options_hedge_collateral: Option<f64>,
    pub options_hedge_pos: Option<f64>,
    pub options_hedge_result: Option<f64>,
    pub options_hedge_edge: Option<f64>,
    /// Lower limit of the trading range
    pub min_px: Option<f64>,
    /// Upper limit of the trading range
    pub max_px: Option<f64>,
    pub min_px_height: Option<f64>,
    pub max_px_height: Option<f64>,
    /// Futures data
    pub futures: Futures,
    pub options_day_notional_volume: Option<f64>,
    pub options_day_market_volume: Option<f64>,
    pub options_day_premium_volume: Option<f64>,
    pub options_day_trades: Option<u64>,
    pub options_alltime_notional_volume: Option<f64>,
    pub options_alltime_market_volume: Option<f64>,
    pub options_alltime_premium_volume: Option<f64>,
    pub options_alltime_trades: Option<u64>,
    /// Options data
    pub options: Option<Vec<OptionStrike>>,
}
