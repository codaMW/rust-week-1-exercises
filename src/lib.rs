// Implement extract_tx_version function below
//use hex::decode;

pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Decode the hex string into bytes
    let bytes = match hex::decode(raw_tx_hex) {
        Ok(b) => b,
        Err(_) => return Err("Hex decode error".to_string()),
    };

    // Check if the length is at least 4 bytes (needed for version)
    if bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    // Extract the first 4 bytes and convert from little-endian
    let version_bytes = &bytes[0..4];
    let version = u32::from_le_bytes([
        version_bytes[0],
        version_bytes[1],
        version_bytes[2],
        version_bytes[3],
    ]);

    Ok(version)
}
