use solana_sdk::pubkey::Pubkey;

/// Helper function to read an u32 from an immutable byte slice
pub fn read_u32(data: &[u8], offset: usize) -> u32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[offset..offset + 4]);
    u32::from_le_bytes(bytes)
}

/// Helper function to read an i64 from an immutable byte slice
pub fn read_i64(data: &[u8], offset: usize) -> i64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[offset..offset + 8]);
    i64::from_le_bytes(bytes)
}

pub fn read_pubkey(data: &[u8], offset: usize) -> Result<Pubkey, Box<dyn std::error::Error>> {
    if data.len() < offset + 32 {
        return Err("Insufficient data for Pubkey".into());
    }
    Ok(Pubkey::new_from_array(data[offset..offset + 32].try_into()?))
}