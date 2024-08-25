use solana_account_decoder::UiDataSliceConfig;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::pubkey::Pubkey;
use crate::exchange::dexnow::data_structures::constants::TOKEN_ACCOUNT_ID_OFFSET;
use crate::exchange::dexnow::dexnow_engine::DEXnowEngine;

impl DEXnowEngine {
    /// Get Token ID from mint public key if this token registered on DEXnow.io
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key
    ///
    /// # Returns
    ///
    /// Token ID
    pub async fn get_token_id(&self, mint: &Pubkey) -> Result<Option<u32>, Box<dyn std::error::Error>> {
        let mut buf = mint.to_bytes();
        buf[28..32].copy_from_slice(&(self.version as u32).to_le_bytes());

        for i in (0..=255u8).rev() {
            let seeds = &[
                &buf[..],
                &self.dexnow_authority.to_bytes(),
                &[i],
            ];

            let pk = match Pubkey::create_program_address(seeds, &self.program_id) {
                Ok(pk) => {
                    pk
                }
                Err(_) => continue,
            };

            let config = RpcAccountInfoConfig {
                encoding: None,
                data_slice: Some(UiDataSliceConfig {
                    offset: TOKEN_ACCOUNT_ID_OFFSET,
                    length: 4,
                }),
                commitment: None,
                min_context_slot: None,
            };

            match self.connection.get_account_with_config(&pk, config).await {
                Ok(opt_account) => {
                    if let Some(account) = opt_account.value {
                        let id = u32::from_le_bytes(account.data[..4].try_into()?);
                        return Ok(Some(id));
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(None)
    }
}