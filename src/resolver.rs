use std::fmt;

pub type Digest = Vec<u8>;
pub type Signature = Vec<u8>;

pub struct ResolverError;

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resolver error display")
    }
}

impl fmt::Debug for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resolver error debug")
    }
}

pub trait Resolver {
    fn digest(&self) -> Result<Digest, ResolverError>;
    fn signature(&self) -> Result<Signature, ResolverError>;
}

pub struct Sha256ImmutableResolver<'a> {
    pub key: &'a Vec<u8>,
    pub content: &'a Vec<u8>,
}

impl<'a> Resolver for Sha256ImmutableResolver<'a> {
    fn digest(&self) -> Result<Digest, ResolverError> {
        Ok(Vec::new())     
    }
    fn signature(&self) -> Result<Digest, ResolverError> {
        Ok(Vec::new())     
    }
}
