pub fn hexsum(s: impl AsRef<[u8]>) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(s);
    let hash = hasher.finalize();
    let hex = format!("{hash:X}");
    hex
}
