use super::resolver::{Resolver, Digest, Signature};

/// The [Validator] is optionally used with a result from a [Source](crate:.source::Source) to verify the origin of
/// content.
pub trait Validator {

    /// Returns `true` if [Signature] can be verified against the [Digest] of content.
    fn verify(&self, digest: &Digest, signature: &Signature) -> bool;
}

pub struct Sha256ImmutableValidator<'r> {
    pub resolver: &'r Resolver<'r>,
}

impl<'a> Validator for Sha256ImmutableValidator<'a> {
    fn verify(&self, digest: &Digest, signature: &Signature) -> bool {
        true
    }
}

/// The default value of [Validator], which performs no validation.
pub struct NoopValidator {
}

impl Validator for NoopValidator {
    fn verify(&self, digest: &Digest, signature: &Signature) -> bool {
        true
    }
}

pub const noopValidator: NoopValidator = NoopValidator{};
