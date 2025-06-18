// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    match hex::decode(&raw_tx_hex[0..8]) {
        Ok(version_bytes) => {
            let tx_version = u32::from_le_bytes(
                version_bytes
                    .try_into()
                    .map_err(|_| "Byte to Integer conversion error".to_string())?,
            );
            Ok(tx_version)
        }

        Err(_) => Err("Hex decode error".to_string()),
    }
}
