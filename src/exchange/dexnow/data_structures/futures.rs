#![allow(dead_code)]
use solana_sdk::pubkey::Pubkey;
use crate::exchange::dexnow::data_structures::line_px::LinePx;

/// Contains data about futures market
pub struct Futures {
    pub bids_tree_account: Pubkey,
    pub asks_tree_account: Pubkey,
    pub bid_orders_account: Pubkey,
    pub ask_orders_account: Pubkey,
    pub lines_account: Pubkey,
    pub maps_account: Pubkey,
    pub client_infos_account: Pubkey,
    pub client_infos2_account: Pubkey,
    pub client_accounts_account: Pubkey,
    /// Futures open interest
    pub open_int: Option<f64>,
    /// Last trade price
    pub last_px: Option<f64>,
    /// Last close price (8am UTC)
    pub last_close_px: Option<f64>,
    /// Futures in last trade
    pub last_futures: Option<f64>,
    pub day_notional_volume: Option<f64>,
    pub day_market_volume: Option<f64>,
    pub day_trades: Option<u64>,
    pub alltime_notional_volume: Option<f64>,
    pub alltime_market_volume: Option<f64>,
    pub alltime_trades: Option<u64>,
    pub apr: Option<f64>,
    /// Time of last trade in seconds since epoch
    pub last_trade_time: Option<u64>,
    /// Orderbook bid quotes
    pub bid_quotes: Option<Vec<LinePx>>,
    /// Orderbook ask quotes
    pub ask_quotes: Option<Vec<LinePx>>,
}