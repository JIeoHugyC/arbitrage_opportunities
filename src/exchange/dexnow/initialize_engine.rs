use std::collections::HashMap;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::address_lookup_table::state::AddressLookupTable;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::serialize_utils::read_pubkey;
use solana_sdk::system_program;
use crate::exchange::dexnow::data_structures::constants::*;
use crate::exchange::dexnow::data_structures::derivative::Derivative;
use crate::exchange::dexnow::data_structures::futures::Futures;
use crate::exchange::dexnow::data_structures::instrument::Instrument;
use crate::exchange::dexnow::data_structures::spot::Spot;
use crate::exchange::dexnow::data_structures::token::Token;
use crate::exchange::dexnow::engine::Engine;

impl Engine {

    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let root_info = self.connection.get_account(&self.root_account).await?;
        if root_info.data.len() < ROOT_ACCOUNT_SIZE {
            return Err("Invalid Root Account".into());
        }

        self.version = u32::from_le_bytes(
            root_info.data[ROOT_ACCOUNT_VERSION_OFFSET..ROOT_ACCOUNT_VERSION_OFFSET + 4].try_into().unwrap()
        ) as u8;
        // self.lut_account = Some(Pubkey::new_from_array(root_info.data[ROOT_ACCOUNT_LUT_ADDRESS_OFFSET..][..32].try_into()?));
        self.distrib_account = Pubkey::new_from_array(root_info.data[ROOT_ACCOUNT_DISTRIB_ADDRESS_OFFSET..][..32].try_into()?);
        self.community_account = Pubkey::new_from_array(root_info.data[ROOT_ACCOUNT_COMMUNITY_ADDRESS_OFFSET..][..32].try_into()?);
        let sol_program_address = Pubkey::new_from_array(root_info.data[ROOT_ACCOUNT_SOL_PROGRAM_ADDRESS_OFFSET..][..32].try_into()?);

        // if let Some(lut_account) = self.lut_account {
        //     let lut_info = self.connection.get_account(&lut_account).await?;
        //     AddressLookupTableAccount::
        //     self.lut = lut_info.value;
        // }


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

        for t in token_accounts {
            let id = u32::from_le_bytes(t.account.data[TOKEN_ACCOUNT_ID_OFFSET..][..4].try_into()?);
            let mask = u32::from_le_bytes(t.account.data[TOKEN_ACCOUNT_MASK_OFFSET..][..4].try_into()?);
            let token_2022 = (mask & 0x80000000) != 0;
            let main_instr_id = u32::from_le_bytes(t.account.data[TOKEN_ACCOUNT_BASE_INSTR_ID_OFFSET..][..4].try_into()?);

            self.tokens.insert(id, Token {
                id,
                account: t.pubkey,
                mint: read_pubkey(&t.account.data, TOKEN_ACCOUNT_ADDRESS_OFFSET)?,
                program_address: read_pubkey(&t.account.data, TOKEN_ACCOUNT_PROGRAM_ADDRESS_OFFSET)?,
                decimals: (mask & 0xF) as u8,
                base_crncy: (mask & 0x40000000) != 0,
                pool: main_instr_id != NULL_INSTR,
                token_2022,
                main_instr_id: if main_instr_id != NULL_INSTR { Some(main_instr_id) } else { None },
            });
        }

        let instr_accounts = self.find_accounts_by_tag(INSTR_STATIC_TAG).await?;

        for i in instr_accounts {
            let id = u32::from_le_bytes(i.account.data[INSTR_STATIC_ACCOUNT_ID_OFFSET..][..4].try_into()?);
            let mask = u32::from_le_bytes(i.account.data[INSTR_STATIC_ACCOUNT_MASK_OFFSET..][..4].try_into()?);
            let pool = (mask & 0x80000000) != 0;
            let derivatives_count = u32::from_le_bytes(i.account.data[INSTR_STATIC_ACCOUNT_TASKS_COUNT_OFFSET..][..4].try_into()?);

            let mut derivatives = Vec::new();
            for j in 0..derivatives_count {
                let offset = INSTR_STATIC_ACCOUNT_SIZE + j as usize * TASK_STATIC_SIZE;
                derivatives.push(Derivative {
                    futures: Futures {
                        bids_tree_account: read_pubkey(&i.account.data, offset + TASK_STATIC_BIDS_TREE_ADDRESS_OFFSET)?,
                        asks_tree_account: read_pubkey(&i.account.data, offset + TASK_STATIC_ASKS_TREE_ADDRESS_OFFSET)?,
                        bid_orders_account: read_pubkey(&i.account.data, offset + TASK_STATIC_BID_ORDERS_ADDRESS_OFFSET)?,
                        ask_orders_account: read_pubkey(&i.account.data, offset + TASK_STATIC_ASK_ORDERS_ADDRESS_OFFSET)?,
                        lines_account: read_pubkey(&i.account.data, offset + TASK_STATIC_LINES_ADDRESS_OFFSET)?,
                        maps_account: read_pubkey(&i.account.data, offset + TASK_STATIC_MAPS_ADDRESS_OFFSET)?,
                        client_infos_account: read_pubkey(&i.account.data, offset + TASK_STATIC_CLIENT_INFOS_ADDRESS_OFFSET)?,
                        client_infos2_account: read_pubkey(&i.account.data, offset + TASK_STATIC_CLIENT_INFOS2_ADDRESS_OFFSET)?,
                        client_accounts_account: read_pubkey(&i.account.data, offset + TASK_STATIC_CLIENT_ACCOUNTS_ADDRESS_OFFSET)?,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }

            self.instruments.insert(id, Instrument {
                id,
                static_account: i.pubkey,
                dynamic_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_DYNAMIC_ADDRESS_OFFSET)?,
                trace_account: Some(read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_TRACE_ADDRESS_OFFSET)?),
                lut_account: Some(read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_LUT_ADDRESS_OFFSET)?),
                asset_token_id: u32::from_le_bytes(i.account.data[INSTR_STATIC_ACCOUNT_TOKEN_ID_OFFSET..][..4].try_into()?),
                derivatives_count,
                base_crncy_token_id: u32::from_le_bytes(i.account.data[INSTR_STATIC_ACCOUNT_MINT_ID_OFFSET..][..4].try_into()?),
                pool,
                pool_token_id: if !pool { Some(u32::from_le_bytes(i.account.data[INSTR_STATIC_ACCOUNT_POOL_TOKEN_ID_OFFSET..][..4].try_into()?)) } else { None },
                spot: Spot {
                    bids_tree_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_BIDS_TREE_ADDRESS_OFFSET)?,
                    asks_tree_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_ASKS_TREE_ADDRESS_OFFSET)?,
                    bid_orders_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_BID_ORDERS_ADDRESS_OFFSET)?,
                    ask_orders_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_ASK_ORDERS_ADDRESS_OFFSET)?,
                    lines_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_LINES_ADDRESS_OFFSET)?,
                    maps_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_MAPS_ADDRESS_OFFSET)?,
                    client_infos_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_CLIENT_INFOS_ADDRESS_OFFSET)?,
                    client_infos2_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_CLIENT_INFOS2_ADDRESS_OFFSET)?,
                    client_accounts_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_CLIENT_ACCOUNTS_ADDRESS_OFFSET)?,
                    m1_candles_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_M1_CANDLES_ADDRESS_OFFSET)?,
                    m15_candles_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_M15_CANDLES_ADDRESS_OFFSET)?,
                    day_candles_account: read_pubkey(&i.account.data, INSTR_STATIC_ACCOUNT_DAY_CANDLES_ADDRESS_OFFSET)?,
                    ..Default::default()
                },
                derivatives,
                ..Default::default()
            });
        }

        Ok(())
    }
}