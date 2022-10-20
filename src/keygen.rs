use rand_core::{OsRng, RngCore};

#[derive(Hash)]
pub struct SecretKey {
    key: [u8; 32],
}

impl SecretKey {
    pub fn new() -> Self {
        SecretKey {
            key: OsRng.fill_bytes([0; 32]),
        }
    }
}
