use log::debug;

use hex;
use sha2::{Sha256, Digest};

use crate::source::{Engine};
use crate::resolver::{
    ResolverItem,
    Digest as ResolverDigest,
    Signature,
    ResolverError,
};
use crate::validator::{
    Validator,
};

pub fn engine() -> Engine {
    return "web2".to_string();
}

/// The default web2 implementation of `ResolverItem` is the `Sha256ImmutableResolver`. This
/// resolver holds the `sha256` hash of the content, which is used as both the pointer and the
/// integrity check of a resource located at a normal web2 HTTP endpoint.
pub struct Sha256ImmutableResolverItem {
    /// The `sha256` digest of the resource.
    pub key: Vec<u8>,
    /// The verbatim content of the resource.
    pub content: Option<Vec<u8>>,
}

impl Sha256ImmutableResolverItem {
    pub fn new(key: &Vec<u8>, content: Option<Vec<u8>>) -> Self {
        Sha256ImmutableResolverItem{
            key: key.clone(),
            content: content,
        }
    }
}

impl ResolverItem for Sha256ImmutableResolverItem {
    fn digest(&self) -> &ResolverDigest {
        return &self.key;
    }
    fn signature(&self) -> Result<ResolverDigest, ResolverError> {
        Ok(Vec::new())     
    }
    fn pointer(&self) -> String {
        let v = &self.key;
        return hex::encode(v);
    }
}

pub struct Sha256ImmutableValidator {}

    impl Validator for Sha256ImmutableValidator {
        fn verify(&self, digest: &ResolverDigest, content: Option<&Vec<u8>>, _signature: Option<&Signature>) -> bool {
            let r: bool;

            match content {
                Some(v) => {
                    let mut h = Sha256::new();
                    h.update(v);
                    let z = h.finalize();
                    r = digest.as_slice() == z.as_slice();
                    debug!("verify sha256 digest {:?}: {}", digest, r);
                },
                _ => { 
                    r = true;
                    debug!("no content for sha256 verify digest {:?}", digest);
                },
            };
            r
        }
    }

