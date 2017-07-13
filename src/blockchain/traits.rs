
pub trait Hashable {
    fn to_sha3_hash(&self) -> [u8; 32];
}

