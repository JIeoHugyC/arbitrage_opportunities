use solana_account_decoder::UiDataSliceConfig;
use solana_sdk::{pubkey::Pubkey};
use crate::exchange::dexnow::data_structures::constants::*;
use crate::exchange::dexnow::engine::Engine;

pub struct GetInstrIdArgs {
    pub asset_token_id: u32,
    pub base_crncy_token_id: u32,
}

impl Engine {
    /**
     * Get Token ID from mint public key if this token registered on DEXnow.io
     * @param mint Public key
     * @returns Token ID
     */
    pub async fn get_instr_id(&self, args: GetInstrIdArgs) -> Result<Option<u32>, Box<dyn std::error::Error>> {
        let mut buf = [0u8; 16];
        buf[0..4].copy_from_slice(&(self.version as u32).to_le_bytes());
        buf[4..8].copy_from_slice(&INSTR_STATIC_TAG.to_le_bytes());
        buf[8..12].copy_from_slice(&args.asset_token_id.to_le_bytes());
        buf[12..16].copy_from_slice(&args.base_crncy_token_id.to_le_bytes());

        for i in (0..=255u8).rev() {
            let seeds = &[
                &buf[..],
                &self.dexnow_authority.to_bytes(),
                &[i],
            ];

            match Pubkey::create_program_address(seeds, &self.program_id) {
                Ok(pk) => {
                    let config = solana_client::rpc_config::RpcAccountInfoConfig {
                        encoding: None,
                        data_slice: Some(UiDataSliceConfig {
                            offset: INSTR_STATIC_ACCOUNT_ID_OFFSET,
                            length: 4,
                        }),
                        commitment: None,
                        min_context_slot: None,
                    };

                    let account_info = self.connection.get_account_with_config(&pk, config).await?;
                    if let Some(info) = account_info.value {
                        let data = &info.data;
                            if data.len() >= 4 {
                                return Ok(Some(u32::from_le_bytes([data[0], data[1], data[2], data[3]])));
                            }

                    }
                },
                Err(_) => continue,
            }
        }

        Ok(None)
    }
}