use sha2::{Digest, Sha256};

pub fn hash<T: AsRef<[u8]>>(data: T) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);

    hasher.finalize().as_slice().into()
}
