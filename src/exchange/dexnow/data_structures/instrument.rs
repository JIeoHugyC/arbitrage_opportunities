#![allow(dead_code)]
use solana_sdk::pubkey::Pubkey;
use crate::exchange::dexnow::data_structures::derivative::Derivative;
use crate::exchange::dexnow::data_structures::spot::Spot;

/// Contains data about instrument
#[derive(Default)]
pub struct Instrument {
    pub static_account: Pubkey,
    /// Account that stores all trading data about instrument
    pub dynamic_account: Pubkey,
    pub trace_account: Option<Pubkey>,
    pub lut_account: Option<Pubkey>,
    /// Instrument ID
    pub id: u64,
    pub asset_token_id: u64,
    pub base_crncy_token_id: u64,
    /// True value means options pool token instrument
    pub pool: bool,
    /// If instrument has derivatives refers to options pool token ID
    pub pool_token_id: Option<u64>,
    pub protocol_fees: Option<f64>,
    /// Spot trading data
    pub spot: Spot,
    pub derivatives_count: u64,
    /// If instrument has derivatives indicates estimated options pool token price
    pub options_pool_est_px: Option<f64>,
    /// If instrument has derivatives indicates options pool token supply
    pub options_pool_supply: Option<f64>,
    pub dividends_time: Option<u64>,
    /// Derivatives trading data
    pub derivatives: Vec<Derivative>,
}