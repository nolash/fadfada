mod resolver;

use resolver::{Resolver, Digest, Signature};

pub trait Validator {
    fn verify(&self, digest: &Digest, signature: &Signature) -> bool;
}

pub struct Sha256ImmutableValidator {
    pub resolver: Resolver,
}
