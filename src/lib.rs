// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    match hex::decode(raw_tx_hex) {
        Ok(decoded) => {
            // Check if the decoded data is long enough to be a valid tx hash
            if decoded.len() < 128 {
                return Err("Transaction data too short".to_string());
            }

            // Select the first 4 bytes for the version
            let version_bytes = &decoded[0..4];

            // Convert the first 4 bytes to a u32 (little-endian)
            let version = u32::from_le_bytes([
                version_bytes[0],
                version_bytes[1],
                version_bytes[2],
                version_bytes[3],
            ]);

            Ok(version)
        }
        Err(_) => Err("Hex decode error".to_string()),
    }
}
