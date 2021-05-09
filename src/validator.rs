use super::resolver::{Resolver, Digest, Signature};

pub trait Validator {
    fn verify(&self, digest: &Digest, signature: &Signature) -> bool;
}

pub struct Sha256ImmutableValidator<'a> {
    pub resolver: &'a dyn Resolver,
}

impl<'a> Validator for Sha256ImmutableValidator<'a> {
    fn verify(&self, digest: &Digest, signature: &Signature) -> bool {
        true
    }
}

pub struct NoopValidator {
}

impl Validator for NoopValidator {
    fn verify(&self, digest: &Digest, signature: &Signature) -> bool {
        true
    }
}

pub const noopValidator: NoopValidator = NoopValidator{};
