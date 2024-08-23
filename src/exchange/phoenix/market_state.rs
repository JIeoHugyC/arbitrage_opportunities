use std::collections::BTreeMap;
use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

// These structs need to be explicitly defined outside of the macro generation because the
// OrderPacket type (which contains these units) implements BorshSerialize and BorshDeserialize
#[derive(Debug, Clone, Copy, Zeroable, Pod, Deserialize, Serialize, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct QuoteLots {
    inner: u64,
}
#[derive(Debug, Clone, Copy, Zeroable, Pod, Deserialize, Serialize, Eq, PartialEq, Default)]
#[repr(transparent)]
pub struct BaseLots {
    inner: u64,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod, Deserialize, Serialize, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Ticks {
    inner: u64,
}

#[repr(C)]
#[derive(
    Eq, BorshDeserialize, BorshSerialize, PartialEq, Debug, Default, Copy, Clone, Zeroable, Pod,
)]
pub struct FIFOOrderId {
    /// The price of the order, in ticks. Each market has a designated
    /// tick size (some number of quote lots per base unit) that is used to convert the price to ticks.
    /// For example, if the tick size is 0.01, then a price of 1.23 is converted to 123 ticks.
    /// If the quote lot size is 0.001, this means that there is a spacing of 10 quote lots
    /// in between each tick.
    pub price_in_ticks: Ticks,

    /// This is the unique identifier of the order, which is used to determine the side of the order.
    /// It is derived from the sequence number of the market.
    ///
    /// If the order is a bid, the sequence number will have its bits inverted, and if it is an ask,
    /// the sequence number will be used as is.
    ///
    /// The way to identify the side of the order is to check the leading bit of `order_id`.
    /// A leading bit of 0 indicates an ask, and a leading bit of 1 indicates a bid. See Side::from_order_id.
    pub order_sequence_number: u64,
}

pub trait OrderbookKey {
    fn price(&self) -> f64;
}

pub trait OrderbookValue {
    fn size(&self) -> f64;
}

#[derive(Debug, Clone, Default)]
pub struct Orderbook<K: Ord + OrderbookKey + Copy, V: OrderbookValue + Copy> {
    pub raw_base_units_per_base_lot: f64,
    pub quote_units_per_raw_base_unit_per_tick: f64,
    pub bids: BTreeMap<K, V>,
    pub asks: BTreeMap<K, V>,
}

#[derive(Clone, Copy, Debug)]
pub struct PhoenixOrder {
    pub num_base_lots: u64,
    pub maker_id: Pubkey,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Zeroable, Pod)]
pub struct TraderState {
    pub quote_lots_locked: QuoteLots,
    pub quote_lots_free: QuoteLots,
    pub base_lots_locked: BaseLots,
    pub base_lots_free: BaseLots,
    _padding: [u64; 8],
}


pub struct MarketState {
    /// State of the bids and offers in the market.
    pub orderbook: Orderbook<FIFOOrderId, PhoenixOrder>,
    /// Authorized makers in the market.
    pub traders: BTreeMap<Pubkey, TraderState>,
}