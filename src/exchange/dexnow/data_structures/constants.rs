#![allow(dead_code)]
use std::str::FromStr;
use lazy_static::lazy_static;
use solana_sdk::pubkey::Pubkey;

lazy_static! {
    pub static ref ADDRESS_LOOKUP_TABLE_PROGRAM_ID: Pubkey = Pubkey::from_str("AddressLookupTab1e1111111111111111111111111").unwrap();
}

pub const U32_BIT_31 : u32 = 1<<31;
pub const U32_BIT_30 : u32 = 1<<30;
pub const DEC: u64 = 1_000_000_000;
pub const NULL_ORDER: u16 = 0xFFFF;
pub const NULL_TASK: u16 = 0xFFFF;
pub const NULL_CLIENT: u32 = 0xFFFFFF;
pub const NULL_INSTR: u32 = 0xFFFFFFF;

pub const CLIENT_DEXNOW_TAG: u8 = 35;
pub const CLIENT_DRV_TAG: u8 = 32;
pub const CLIENT_PRIMARY_TAG: u8 = 31;
pub const COMMUNITY_TAG: u8 = 34;
pub const DAY_CANDLES_CAPACITY: u16 = 5844;
pub const DISTRIB_TAG: u8 = 33;
pub const FOUR: u8 = 4;
pub const FRACTIONS: u8 = 65;
pub const FUTURES_ASK_ORDERS_TAG: u8 = 29;
pub const FUTURES_ASKS_TREE_TAG: u8 = 27;
pub const FUTURES_BID_ORDERS_TAG: u8 = 28;
pub const FUTURES_BIDS_TREE_TAG: u8 = 26;
pub const FUTURES_CLIENT_ACCOUNTS_TAG: u8 = 23;
pub const FUTURES_CLIENT_INFOS_TAG: u8 = 24;
pub const FUTURES_CLIENT_INFOS2_TAG: u8 = 25;
pub const FUTURES_LINES_TAG: u8 = 30;
pub const FUTURES_MAPS_TAG: u8 = 22;
pub const HOLDER_TAG: u8 = 1;
pub const I128_SIZE: u8 = 16;
pub const I32_SIZE: u8 = 4;
pub const I64_SIZE: u8 = 8;
pub const INSTR_DYNAMIC_TAG: u8 = 7;
pub const INSTR_STATIC_TAG: u32 = 6;
pub const INSTR_TRACE_TAG: u8 = 8;
pub const M1_CANDLES_CAPACITY: u16 = 10080;
pub const M15_CANDLES_CAPACITY: u16 = 2688;
pub const MAPS_SIZE: u32 = 42160;
pub const MARKET_DEPTH: usize = 20;
pub const MAX_DURATION: u8 = 28;
pub const MAX_LINES: u16 = 2048;
pub const MAX_ORDERS: u16 = 14336;
pub const PK_SIZE: u8 = 32;
pub const ROOT_TAG: u8 = 2;
pub const SOL_TAG: u8 = 3;
pub const SPOT_15M_CANDLES_TAG: u8 = 20;
pub const SPOT_1M_CANDLES_TAG: u8 = 19;
pub const SPOT_ASK_ORDERS_TAG: u8 = 17;
pub const SPOT_ASKS_TREE_TAG: u8 = 15;
pub const SPOT_BID_ORDERS_TAG: u8 = 16;
pub const SPOT_BIDS_TREE_TAG: u8 = 14;
pub const SPOT_CLIENT_ACCOUNTS_TAG: u8 = 11;
pub const SPOT_CLIENT_INFOS_TAG: u8 = 12;
pub const SPOT_CLIENT_INFOS2_TAG: u8 = 13;
pub const SPOT_DAY_CANDLES_TAG: u8 = 21;
pub const SPOT_LINES_TAG: u8 = 18;
pub const SPOT_MAPS_TAG: u8 = 10;
pub const STRIKES_COUNT: u8 = 100;
pub const TOKEN_TAG: u32 = 4;
pub const U32_SIZE: u8 = 4;
pub const U8_SIZE: u8 = 1;

pub const BASE_CRNCY_MINT_ID_OFFSET: usize = 0;
pub const BASE_CRNCY_RESERVED_OFFSET: usize = 4;
pub const BASE_CRNCY_FUNDS_OFFSET: usize = 8;
pub const BASE_CRNCY_RATE_OFFSET: usize = 16;
pub const BASE_CRNCY_SIZE: usize = 24;

pub const ROOT_ACCOUNT_TAG_OFFSET: usize = 0;
pub const ROOT_ACCOUNT_VERSION_OFFSET: usize = 4;
pub const ROOT_ACCOUNT_OPERATOR_ADDRESS_OFFSET: usize = 8;
pub const ROOT_ACCOUNT_HOLDER_ADDRESS_OFFSET: usize = 40;
pub const ROOT_ACCOUNT_COMMUNITY_ADDRESS_OFFSET: usize = 72;
pub const ROOT_ACCOUNT_DISTRIB_ADDRESS_OFFSET: usize = 104;
pub const ROOT_ACCOUNT_DEXNOW_MINT_ADDRESS_OFFSET: usize = 136;
pub const ROOT_ACCOUNT_LUT_ADDRESS_OFFSET: usize = 168;
pub const ROOT_ACCOUNT_SOL_PROGRAM_ADDRESS_OFFSET: usize = 200;
pub const ROOT_ACCOUNT_SOL_MASK_OFFSET: usize = 232;
pub const ROOT_ACCOUNT_CLIENTS_COUNT_OFFSET: usize = 236;
pub const ROOT_ACCOUNT_TOKENS_COUNT_OFFSET: usize = 240;
pub const ROOT_ACCOUNT_INSTR_COUNT_OFFSET: usize = 244;
pub const ROOT_ACCOUNT_SIZE: usize = 248;

pub const TOKEN_ACCOUNT_TAG_OFFSET: usize = 0;
pub const TOKEN_ACCOUNT_VERSION_OFFSET: usize = 4;
pub const TOKEN_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const TOKEN_ACCOUNT_ADDRESS_OFFSET: usize = 40;
pub const TOKEN_ACCOUNT_PROGRAM_ADDRESS_OFFSET: usize = 72;
pub const TOKEN_ACCOUNT_ID_OFFSET: usize = 104;
pub const TOKEN_ACCOUNT_MASK_OFFSET: usize = 108;
pub const TOKEN_ACCOUNT_BASE_INSTR_ID_OFFSET: usize = 112;
pub const TOKEN_ACCOUNT_BASE_CRNCY_OFFSET: usize = 116;
pub const TOKEN_ACCOUNT_SIZE: usize = 120;

pub const INSTR_STATIC_ACCOUNT_TAG_OFFSET: usize = 0;
pub const INSTR_STATIC_ACCOUNT_VERSION_OFFSET: usize = 4;
pub const INSTR_STATIC_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const INSTR_STATIC_ACCOUNT_DYNAMIC_ADDRESS_OFFSET: usize = 40;
pub const INSTR_STATIC_ACCOUNT_ID_OFFSET: usize = 72;
pub const INSTR_STATIC_ACCOUNT_TOKEN_ID_OFFSET: usize = 76;
pub const INSTR_STATIC_ACCOUNT_MINT_ID_OFFSET: usize = 80;
pub const INSTR_STATIC_ACCOUNT_MASK_OFFSET: usize = 84;
pub const INSTR_STATIC_ACCOUNT_POOL_INSTR_ID_OFFSET: usize = 88;
pub const INSTR_STATIC_ACCOUNT_TASKS_COUNT_OFFSET: usize = 92;
pub const INSTR_STATIC_ACCOUNT_TOKEN_DECS_COUNT_OFFSET: usize = 96;
pub const INSTR_STATIC_ACCOUNT_MINT_DECS_COUNT_OFFSET: usize = 100;
pub const INSTR_STATIC_ACCOUNT_POOL_TOKEN_ID_OFFSET: usize = 104;
pub const INSTR_STATIC_ACCOUNT_RESERVED_OFFSET: usize = 108;
pub const INSTR_STATIC_ACCOUNT_MAPS_ADDRESS_OFFSET: usize = 112;
pub const INSTR_STATIC_ACCOUNT_CLIENT_ACCOUNTS_ADDRESS_OFFSET: usize = 144;
pub const INSTR_STATIC_ACCOUNT_CLIENT_INFOS_ADDRESS_OFFSET: usize = 176;
pub const INSTR_STATIC_ACCOUNT_CLIENT_INFOS2_ADDRESS_OFFSET: usize = 208;
pub const INSTR_STATIC_ACCOUNT_BIDS_TREE_ADDRESS_OFFSET: usize = 240;
pub const INSTR_STATIC_ACCOUNT_ASKS_TREE_ADDRESS_OFFSET: usize = 272;
pub const INSTR_STATIC_ACCOUNT_BID_ORDERS_ADDRESS_OFFSET: usize = 304;
pub const INSTR_STATIC_ACCOUNT_ASK_ORDERS_ADDRESS_OFFSET: usize = 336;
pub const INSTR_STATIC_ACCOUNT_LINES_ADDRESS_OFFSET: usize = 368;
pub const INSTR_STATIC_ACCOUNT_M1_CANDLES_ADDRESS_OFFSET: usize = 400;
pub const INSTR_STATIC_ACCOUNT_M15_CANDLES_ADDRESS_OFFSET: usize = 432;
pub const INSTR_STATIC_ACCOUNT_DAY_CANDLES_ADDRESS_OFFSET: usize = 464;
pub const INSTR_STATIC_ACCOUNT_TRACE_ADDRESS_OFFSET: usize = 496;
pub const INSTR_STATIC_ACCOUNT_LUT_ADDRESS_OFFSET: usize = 528;
pub const INSTR_STATIC_ACCOUNT_DEC_FACTOR_OFFSET: usize = 560;
pub const INSTR_STATIC_ACCOUNT_SIZE: usize = 568;

pub const TASK_STATIC_MAPS_ADDRESS_OFFSET: usize = 0;
pub const TASK_STATIC_CLIENT_ACCOUNTS_ADDRESS_OFFSET: usize = 32;
pub const TASK_STATIC_CLIENT_INFOS_ADDRESS_OFFSET: usize = 64;
pub const TASK_STATIC_CLIENT_INFOS2_ADDRESS_OFFSET: usize = 96;
pub const TASK_STATIC_BIDS_TREE_ADDRESS_OFFSET: usize = 128;
pub const TASK_STATIC_ASKS_TREE_ADDRESS_OFFSET: usize = 160;
pub const TASK_STATIC_BID_ORDERS_ADDRESS_OFFSET: usize = 192;
pub const TASK_STATIC_ASK_ORDERS_ADDRESS_OFFSET: usize = 224;
pub const TASK_STATIC_LINES_ADDRESS_OFFSET: usize = 256;
pub const TASK_STATIC_MASK_OFFSET: usize = 288;
pub const TASK_STATIC_SIZE: usize = 296;


pub const INSTR_DYNAMIC_ACCOUNT_ID_OFFSET: usize = 40;
pub const INSTR_DYNAMIC_ACCOUNT_BIDS_OFFSET: usize = 384;
pub const INSTR_DYNAMIC_ACCOUNT_ASKS_OFFSET: usize = 704;