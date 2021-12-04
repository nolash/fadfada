use crate::source::{Engine};
use crate::resolver::{
    ResolverItem,
    Digest,
    ResolverError,
};

pub fn engine() -> Engine {
    return "web2".to_string();
}

/// The default web2 implementation of `ResolverItem` is the `Sha256ImmutableResolver`. This
/// resolver holds the `sha256` hash of the content, which is used as both the pointer and the
/// integrity check of a resource located at a normal web2 HTTP endpoint.
pub struct Sha256ImmutableResolverItem<'a> {
    /// The `sha256` digest of the resource.
    pub key: &'a Vec<u8>,
    /// The verbatim content of the resource.
    pub content: Option<Vec<u8>>,
}

impl<'a> ResolverItem for Sha256ImmutableResolverItem<'a> {
    fn digest(&self) -> &Digest {
        return self.key;
        //Ok(Vec::new())     
    }
    fn signature(&self) -> Result<Digest, ResolverError> {
        Ok(Vec::new())     
    }
    fn pointer(&self) -> String {
        let v = self.key;
        return hex::encode(v);
    }
}
