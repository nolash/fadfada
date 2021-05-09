pub type Key = Vec<u8>; 
pub type Fingerprint = Vec<u8>;

pub trait Keystore {
    fn add(key: Key) -> usize;
    fn have(fp: Fingerprint) -> bool;
    fn get(fp: Fingerprint) -> Result<Key, ()>;
}
