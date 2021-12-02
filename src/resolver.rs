use std::fmt;
use std::collections::HashMap;
use std::error::Error;

pub type Digest = Vec<u8>;
pub type Signature = Vec<u8>;
use crate::source;


/// ErrorDetail adds detail to the `ResolverError` type raised on any error occurring within the
/// resolver package.
pub enum ErrorDetail {
    EngineExistsError,
    UnknownEngineError(String),
}


/// ResolverError encapsulates any error raised within the resolver package.
pub struct ResolverError {
    detail: ErrorDetail,
}

impl ResolverError {
    fn new(e: ErrorDetail) -> ResolverError {
        ResolverError {
            detail: e,
        } 
    }
}

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


/// ResolverItem represents the pointer to a resource for a specific source engine.
///
/// A `Resolver` will hold one or more `ResolverItem`s, which will be visited by the corresponding
/// `Source` object when traversing a request graph.
pub trait ResolverItem {

    /// Return the digest of the content in the context of the particular backend.
    fn digest(&self) -> Result<&Digest, ResolverError>;

    /// Return the signature data for the content.
    ///
    /// It is possible that content will be missing corresponding signature. If this is the case,
    /// it is up to the visitor to decide whether a missing signature constitutes validation by
    /// default or not.
    fn signature(&self) -> Result<Signature, ResolverError>;
}


/// The default web2 implementation of `ResolverItem` is the `Sha256ImmutableResolver`. This
/// resolver holds the `sha256` hash of the content, which is used as both the pointer and the
/// integrity check of a resource located at a normal web2 HTTP endpoint.
pub struct Sha256ImmutableResolver<'a> {
    /// The `sha256` digest of the resource.
    pub key: &'a Vec<u8>,
    /// The verbatim content of the resource.
    pub content: Option<Vec<u8>>,
}

impl<'a> ResolverItem for Sha256ImmutableResolver<'a> {
    fn digest(&self) -> Result<&Digest, ResolverError> {
        return Ok(self.key);
        //Ok(Vec::new())     
    }
    fn signature(&self) -> Result<Digest, ResolverError> {
        Ok(Vec::new())     
    }
}


/// A key-value store of source engine identifiers mapped to `ResolverItem`s.
///
/// If an [source::Engine] to `ResolverItem` mapping exists for a specific resource, then the corresponding
/// `Source` object for that `Engine` will use the digest returned to complete the request using
/// the associated `Endpoint` objects.
pub struct Resolver<'r> {
    resolvers: HashMap<source::Engine, &'r ResolverItem>,
}


impl<'r> Resolver<'r> {
    pub fn new() -> Resolver<'r> {
        Resolver {
            resolvers: HashMap::new(),
        }
    }

    /// Register a [ResolverItem] for an [source::Engine].
    /// 
    /// Will error if a record for [source::Engine] already exists.
    pub fn add(&mut self, e: source::Engine, r: &'r ResolverItem) -> Result<(), ResolverError> {
        if self.resolvers.contains_key(&e) {
            let e = ResolverError::new(ErrorDetail::EngineExistsError);
            return Err(e);
        }
        self.resolvers.insert(e, r);
        Ok(())
    }

    /// Retrieve the [ResolverItem] registered for an [source::Engine].
    /// 
    /// Will error if a record for `Engine` doesn't exist.
    pub fn get_for(&self, e: source::Engine) -> Result<&Digest, ResolverError> {
        let item = self.resolvers.get(&e);
        match item {
            Some(x) => {
                return x.digest();
            },
            None => {
                let err_detail = ErrorDetail::UnknownEngineError(e.to_string());
                let err = ResolverError::new(err_detail);
                return Err(err);
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{
        Sha256ImmutableResolver,
        Resolver,
        ResolverItem,
    };
    use crate::source::{Engine};

    #[test]
    fn create_resolver() {
        let key_one: Vec<u8> = vec![1, 2, 3];
        let key_two: Vec<u8> = vec![4, 5, 6];
        let mut r: Resolver = Resolver::new();
        let ri_one: Sha256ImmutableResolver = Sha256ImmutableResolver{key: &key_one, content: None};
        let ri_two: Sha256ImmutableResolver = Sha256ImmutableResolver{key: &key_two, content: None};
        let engine_string_one: Engine = "one".to_string();
        let engine_string_two: Engine = "two".to_string();
        r.add(engine_string_one.clone(), &ri_one);
        r.add(engine_string_two.clone(), &ri_two);

        let mut ri_returned = r.get_for(engine_string_one).unwrap();
        let mut ri_orig = ri_one.digest().unwrap();
        assert_eq!(ri_orig, ri_returned);

        ri_returned = r.get_for(engine_string_two).unwrap();
        ri_orig = ri_two.digest().unwrap();
        assert_eq!(ri_orig, ri_returned);
    }
}
