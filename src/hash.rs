use sha2::{Sha256, Digest};
use blake3;
use xxhash_rust::xxh3;
use img_hash::{HasherConfig, HashAlg};
use img_hash::image::load_from_memory;

pub fn hash_data(algorithm: &str, data: &[u8]) -> Vec<u8>{
    
    match algorithm {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        "blake3" => {
            let mut hasher = blake3::Hasher::new();
            hasher.update(data);
            hasher.finalize().as_bytes().to_vec()
        }
        "xxh3" => {
            let hash = xxh3::xxh3_64(data);
            hash.to_le_bytes().to_vec()
        }
        _=>panic!("algo not provided,{}",algorithm)
    }
}
