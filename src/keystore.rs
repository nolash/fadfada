/// Verbatim public [Key] bytes.
pub type Key = Vec<u8>; 

/// Context-dependent fingerprint of public [Key].
pub type Fingerprint = Vec<u8>;

/// Manages public keys used by [Validator](crate::validator::Validator) to validate content.
pub trait Keystore {

    /// Add a new public [Key]
    ///
    /// Returns the length of the [Key] added.
    fn add(key: Key) -> usize;

    /// Returns true if the public [Key] corresponding to the [Fingerprint] exists in the
    /// [Keystore].
    fn have(fp: Fingerprint) -> bool;

    /// Retrieve the [Key] corresponding to the given [Fingerprint].
    fn get(fp: Fingerprint) -> Result<Key, ()>;
}
