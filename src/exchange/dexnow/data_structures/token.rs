#![allow(dead_code)]
use solana_sdk::pubkey::Pubkey;

/// Contains data about Token
pub struct Token {
    /// DEXnow.io account that stores data about registered token
    pub account: Pubkey,
    /// Mint token address
    pub mint: Pubkey,
    /// SPL token account in which tokens are stored
    pub program_address: Pubkey,
    /// Token ID
    pub id: u64,
    pub decimals: u8,
    pub base_crncy: bool,
    /// True if options pool token
    pub pool: bool,
    pub token_2022: bool,
    /// Options pool token indicates main instrument ID
    pub main_instr_id: Option<u64>,
}