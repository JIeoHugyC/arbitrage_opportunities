#![allow(dead_code)]
use solana_sdk::pubkey::Pubkey;
use crate::exchange::dexnow::data_structures::line_px::LinePx;

/// Contains data about spot market
#[derive(Default)]
pub struct Spot {
    pub bids_tree_account: Pubkey,
    pub asks_tree_account: Pubkey,
    pub bid_orders_account: Pubkey,
    pub ask_orders_account: Pubkey,
    pub lines_account: Pubkey,
    pub maps_account: Pubkey,
    pub client_infos_account: Pubkey,
    pub client_infos2_account: Pubkey,
    pub client_accounts_account: Pubkey,
    pub m1_candles_account: Pubkey,
    pub m15_candles_account: Pubkey,
    pub day_candles_account: Pubkey,
    /// LP pool tokens supply
    pub pool_tokens_supply: Option<f64>,
    /// Asset tokens amount in pool
    pub asset_tokens_in_pool: Option<f64>,
    /// Base currency tokens in pool
    pub base_crncy_tokens_in_pool: Option<f64>,
    /// Undistributed asset token fees
    pub asset_token_fees_in_pool: Option<f64>,
    /// Undistributed base currency token fees
    pub base_crncy_token_fees_in_pool: Option<f64>,
    pub day_sigma: Option<f64>,
    pub day_sigma2: Option<f64>,
    pub hour_sigma: Option<f64>,
    pub last_hour_px: Option<f64>,
    /// Last trade price
    pub last_px: Option<f64>,
    /// Last close price (8am UTC)
    pub last_close_px: Option<f64>,
    /// Last Fixing Price (8am UTC). Use for derivatives settlement.
    pub fixing_px: Option<f64>,
    /// Asset tokens day volume
    pub day_asset_tokens: Option<f64>,
    /// Base Currency tokens day volume
    pub day_base_crncy_tokens: Option<f64>,
    pub day_trades: Option<u64>,
    pub alltime_asset_tokens: Option<f64>,
    pub alltime_base_crncy_tokens: Option<f64>,
    pub alltime_trades: Option<u64>,
    /// Asset tokens in last trade
    pub last_asset_tokens: Option<f64>,
    /// Base Currency tokens in last trade
    pub last_base_crncy_tokens: Option<f64>,
    /// Time of last trade. Seconds since epoch
    pub last_trade_time: Option<u64>,
    /// Orderbook bid quotes
    pub bid_quotes: Option<Vec<LinePx>>,
    /// Orderbook ask quotes
    pub ask_quotes: Option<Vec<LinePx>>,
}