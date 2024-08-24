use std::collections::HashMap;
use std::str::FromStr;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_account_decoder::{UiAccountEncoding, UiDataSliceConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{system_program};
use crate::exchange::dexnow::data_structures::constants::*;
use crate::exchange::dexnow::data_structures::derivative::Derivative;
use crate::exchange::dexnow::data_structures::futures::Futures;
use crate::exchange::dexnow::data_structures::instrument::Instrument;
use crate::exchange::dexnow::data_structures::spot::Spot;
use crate::exchange::dexnow::data_structures::token::Token;
use crate::exchange::dexnow::engine::Engine;
use crate::exchange::dexnow::get_instrument_id::GetInstrIdArgs;

const SOL_TOKEN_ID: u32 = 0;

impl Engine {

    fn read_pubkey(data: &[u8], offset: usize) -> Result<Pubkey, Box<dyn std::error::Error>> {
        if data.len() < offset + 32 {
            return Err("Insufficient data for Pubkey".into());
        }
        Ok(Pubkey::new_from_array(data[offset..offset + 32].try_into()?))
    }
    pub async fn find_accounts_by_tag(&self, tag: u32) -> Result<Vec<(Pubkey, Account)>, Box<dyn std::error::Error>> {
        let mut tag_buf = [0u8; 8];
        tag_buf[0..4].copy_from_slice(&tag.to_le_bytes());
        tag_buf[4..8].copy_from_slice(&(self.version as u32).to_le_bytes());
        let encoded_tag_buf = base64::encode(tag_buf);

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

    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let root_info = self.connection.get_account(&self.root_account).await?;
        if root_info.data.len() < ROOT_ACCOUNT_SIZE {
            return Err("Invalid Root Account".into());
        }

        self.version = u32::from_le_bytes(
            root_info.data[ROOT_ACCOUNT_VERSION_OFFSET..ROOT_ACCOUNT_VERSION_OFFSET + 4].try_into().unwrap()
        ) as u8;
        self.distrib_account = Self::read_pubkey(&root_info.data, ROOT_ACCOUNT_DISTRIB_ADDRESS_OFFSET)?;
        self.community_account = Self::read_pubkey(&root_info.data, ROOT_ACCOUNT_COMMUNITY_ADDRESS_OFFSET)?;
        let sol_program_address = Pubkey::new_from_array(root_info.data[ROOT_ACCOUNT_SOL_PROGRAM_ADDRESS_OFFSET..][..32].try_into()?);


        let token_accounts = self.find_accounts_by_tag(TOKEN_TAG).await?;

        self.tokens = HashMap::new();
        self.tokens.insert(0, Token {
            id: 0,
            account: system_program::id(),
            mint: system_program::id(),
            program_address: sol_program_address,
            decimals: 9,
            base_crncy: false,
            pool: false,
            token_2022: false,
            main_instr_id: None,
        });

        self.instruments = HashMap::new();

        for (pubkey, account) in token_accounts {
            let id = u32::from_le_bytes(account.data[TOKEN_ACCOUNT_ID_OFFSET..][..4].try_into()?) as u64;
            let mask = u32::from_le_bytes(account.data[TOKEN_ACCOUNT_MASK_OFFSET..][..4].try_into()?);
            let token_2022 = (mask & 0x80000000) != 0;
            let main_instr_id = u32::from_le_bytes(account.data[TOKEN_ACCOUNT_BASE_INSTR_ID_OFFSET..][..4].try_into()?);

            self.tokens.insert(id, Token {
                id,
                account: pubkey,
                mint: Self::read_pubkey(&account.data, TOKEN_ACCOUNT_ADDRESS_OFFSET)?,
                program_address: Self::read_pubkey(&account.data, TOKEN_ACCOUNT_PROGRAM_ADDRESS_OFFSET)?,
                decimals: (mask & 0xF) as u8,
                base_crncy: (mask & 0x40000000) != 0,
                pool: main_instr_id != NULL_INSTR,
                token_2022,
                main_instr_id: if main_instr_id != NULL_INSTR { Some(main_instr_id as u64) } else { None },
            });
        }

        let instr_accounts = self.find_accounts_by_tag(INSTR_STATIC_TAG).await?;

        for (pubkey, account) in instr_accounts {
            let id = u32::from_le_bytes(account.data[INSTR_STATIC_ACCOUNT_ID_OFFSET..][..4].try_into()?) as u64;
            let mask = u32::from_le_bytes(account.data[INSTR_STATIC_ACCOUNT_MASK_OFFSET..][..4].try_into()?);
            let pool = (mask & 0x80000000) != 0;
            let derivatives_count = u32::from_le_bytes(account.data[INSTR_STATIC_ACCOUNT_TASKS_COUNT_OFFSET..][..4].try_into()?) as u64;

            let mut derivatives = Vec::new();
            for j in 0..derivatives_count {
                let offset = INSTR_STATIC_ACCOUNT_SIZE + j as usize * TASK_STATIC_SIZE;
                derivatives.push(Derivative {
                    futures: Futures {
                        bids_tree_account: Self::read_pubkey(&account.data, offset + TASK_STATIC_BIDS_TREE_ADDRESS_OFFSET)?,
                        asks_tree_account: Self::read_pubkey(&account.data, offset + TASK_STATIC_ASKS_TREE_ADDRESS_OFFSET)?,
                        bid_orders_account: Self::read_pubkey(&account.data, offset + TASK_STATIC_BID_ORDERS_ADDRESS_OFFSET)?,
                        ask_orders_account: Self::read_pubkey(&account.data, offset + TASK_STATIC_ASK_ORDERS_ADDRESS_OFFSET)?,
                        lines_account: Self::read_pubkey(&account.data, offset + TASK_STATIC_LINES_ADDRESS_OFFSET)?,
                        maps_account: Self::read_pubkey(&account.data, offset + TASK_STATIC_MAPS_ADDRESS_OFFSET)?,
                        client_infos_account: Self::read_pubkey(&account.data, offset + TASK_STATIC_CLIENT_INFOS_ADDRESS_OFFSET)?,
                        client_infos2_account: Self::read_pubkey(&account.data, offset + TASK_STATIC_CLIENT_INFOS2_ADDRESS_OFFSET)?,
                        client_accounts_account: Self::read_pubkey(&account.data, offset + TASK_STATIC_CLIENT_ACCOUNTS_ADDRESS_OFFSET)?,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }

            self.instruments.insert(id, Instrument {
                id,
                static_account: pubkey,
                dynamic_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_DYNAMIC_ADDRESS_OFFSET)?,
                trace_account: Some(Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_TRACE_ADDRESS_OFFSET)?),
                lut_account: Some(Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_LUT_ADDRESS_OFFSET)?),
                asset_token_id: u32::from_le_bytes(account.data[INSTR_STATIC_ACCOUNT_TOKEN_ID_OFFSET..][..4].try_into()?) as u64,
                derivatives_count,
                base_crncy_token_id: u32::from_le_bytes(account.data[INSTR_STATIC_ACCOUNT_MINT_ID_OFFSET..][..4].try_into()?) as u64,
                pool,
                pool_token_id: if !pool { Some(u32::from_le_bytes(account.data[INSTR_STATIC_ACCOUNT_POOL_TOKEN_ID_OFFSET..][..4].try_into()?) as u64) } else { None },
                spot: Spot {
                    bids_tree_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_BIDS_TREE_ADDRESS_OFFSET)?,
                    asks_tree_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_ASKS_TREE_ADDRESS_OFFSET)?,
                    bid_orders_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_BID_ORDERS_ADDRESS_OFFSET)?,
                    ask_orders_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_ASK_ORDERS_ADDRESS_OFFSET)?,
                    lines_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_LINES_ADDRESS_OFFSET)?,
                    maps_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_MAPS_ADDRESS_OFFSET)?,
                    client_infos_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_CLIENT_INFOS_ADDRESS_OFFSET)?,
                    client_infos2_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_CLIENT_INFOS2_ADDRESS_OFFSET)?,
                    client_accounts_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_CLIENT_ACCOUNTS_ADDRESS_OFFSET)?,
                    m1_candles_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_M1_CANDLES_ADDRESS_OFFSET)?,
                    m15_candles_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_M15_CANDLES_ADDRESS_OFFSET)?,
                    day_candles_account: Self::read_pubkey(&account.data, INSTR_STATIC_ACCOUNT_DAY_CANDLES_ADDRESS_OFFSET)?,
                    ..Default::default()
                },
                derivatives,
                ..Default::default()
            });
        }

        let usdc_token_id = self.get_token_id(&Pubkey::from_str("A2Pz6rVyXuadFkKnhMXd1w9xgSrZd8m8sEGpuGuyFhaj").unwrap()).await.unwrap();
        let instr_id = self.get_instr_id(GetInstrIdArgs{
            base_crncy_token_id: usdc_token_id.unwrap(),
            asset_token_id: SOL_TOKEN_ID,
        }).await.unwrap();
        println!("USDC dyn acc: {:?}, instr id: {:?}", usdc_token_id, instr_id);

        if let Some(instr_id) = instr_id {
            let target_instrument =
                self.instruments.values().find(|instr| instr.id == instr_id as u64);
            if let Some(target_instrument) = target_instrument {
                println!("Target instrument: {:?}", target_instrument.dynamic_account);
                let dyn_acc = self.connection.get_account(&target_instrument.dynamic_account).await;
                if let Ok(dyn_acc) = dyn_acc {
                    let dyn_data = self.decode_instr_dynamic_account(&dyn_acc);
                    println!("Dynamic account data: {:?}", dyn_data);
                }
            }
        }
        Ok(())
    }
}