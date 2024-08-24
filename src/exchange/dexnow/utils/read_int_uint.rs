/// Helper function to read an i32 from a byte slice
pub fn read_i32(data: &[u8], offset: usize) -> i32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[offset..offset + 4]);
    i32::from_le_bytes(bytes)
}

/// Helper function to read an u32 from a byte slice
pub fn read_u32(data: &[u8], offset: usize) -> u32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[offset..offset + 4]);
    u32::from_le_bytes(bytes)
}

/// Helper function to read an i64 from a byte slice
pub fn read_i64(data: &[u8], offset: usize) -> i64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[offset..offset + 8]);
    i64::from_le_bytes(bytes)
}

/// Helper function to read an u64 from a byte slice
pub fn read_u64(data: &[u8], offset: usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[offset..offset + 8]);
    u64::from_le_bytes(bytes)
}

/// Helper function to read a f32 from a byte slice
pub fn read_f32(data: &[u8], offset: usize) -> f32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[offset..offset + 4]);
    f32::from_le_bytes(bytes)
}

/// Helper function to read a f64 from a byte slice
pub fn read_f64(data: &[u8], offset: usize) -> f64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[offset..offset + 8]);
    f64::from_le_bytes(bytes)
}