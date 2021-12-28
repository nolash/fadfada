use hex;
use crate::resolver::{
        ResolverItem,
        ResolverError,
        Digest,
        Signature,
    };

pub struct TestResolverItem {
    pub key: Digest,
}

impl<'r> ResolverItem for TestResolverItem {
    fn digest(&self) -> &Digest {
        return &self.key;
    }

    fn pointer(&self) -> String {
        return hex::encode(&self.key);
    }

    fn signature(&self) -> Result<Signature, ResolverError> {
        Ok(vec![])
    }
}
