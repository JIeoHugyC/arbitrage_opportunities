use base64::Engine;
use base64::engine::general_purpose;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use crate::exchange::dexnow::dexnow_engine::DEXnowEngine;

impl DEXnowEngine {
    pub async fn find_accounts_by_tag(&self, tag: u32) -> Result<Vec<(Pubkey, Account)>, Box<dyn std::error::Error>> {
        let mut tag_buf = [0u8; 8];
        tag_buf[0..4].copy_from_slice(&tag.to_le_bytes());
        tag_buf[4..8].copy_from_slice(&(self.version as u32).to_le_bytes());
        let encoded_tag_buf = general_purpose::STANDARD.encode(tag_buf);

        let config = RpcProgramAccountsConfig {
            filters: Some(vec![
                RpcFilterType::Memcmp(Memcmp::new(0, MemcmpEncodedBytes::Base64(encoded_tag_buf))),
            ]),
            account_config: RpcAccountInfoConfig {
                commitment: None,
                data_slice: None,
                encoding: Some(UiAccountEncoding::Base64),
                min_context_slot: None,
            },
            with_context: Some(false),
            sort_results: None,
        };

        let accounts = self.connection.get_program_accounts_with_config(&self.program_id, config).await?;
        Ok(accounts)
    }
}