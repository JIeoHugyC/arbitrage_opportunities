use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};
use crate::exchange::dexnow::data_structures::instrument::Instrument;
use crate::exchange::dexnow::data_structures::token::Token;

pub struct Engine {
    version: u8,
    connection: RpcClient,
    program_id: Pubkey,
    dexnow_authority: Pubkey,
    root_account: Pubkey,
    distrib_account: Pubkey,
    community_account: Pubkey,
    lut_account: Option<Pubkey>,
    wallet: Option<Keypair>,
    original_client_id: Option<u64>,
    client_primary_account: Option<Pubkey>,
    client_drv_account: Option<Pubkey>,
    client_dexnow_account: Option<Pubkey>,
    client_lut_account: Option<Pubkey>,
    tokens: std::collections::HashMap<u64, Token>,
    instruments: std::collections::HashMap<u64, Instrument>,
}

impl Engine {
    pub fn new(connection: RpcClient, root_account: Pubkey, program_id: Pubkey) -> Self {
        let dexnow_authority = Pubkey::find_program_address(&[b"ndxnt"], &program_id).0;

        Engine {
            version: 1,
            connection,
            program_id,
            dexnow_authority,
            root_account,
            distrib_account: Pubkey::default(),
            community_account: Pubkey::default(),
            lut_account: None,
            wallet: None,
            original_client_id: None,
            client_primary_account: None,
            client_drv_account: None,
            client_dexnow_account: None,
            client_lut_account: None,
            tokens: std::collections::HashMap::new(),
            instruments: std::collections::HashMap::new(),
        }
    }
}



