use radix_engine_common::crypto::{blake2b_256_hash, Hash};

/// Computes the hash digest of a message.
pub fn hash<T: AsRef<[u8]>>(data: T) -> Hash {
    blake2b_256_hash(data)
}
