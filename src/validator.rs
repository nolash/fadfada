use log::debug;

use super::resolver::{Resolver, Digest, Signature};

/// The [Validator] is optionally used with a result from a [Source](crate:.source::Source) to verify the origin of
/// content.
pub trait Validator {

    /// Returns `true` if [Signature] can be verified against the [Digest] of content.
    fn verify(&self, digest: &Digest, content: Option<&Vec<u8>>, signature: Option<&Signature>) -> bool;
}

pub struct Sha256ImmutableValidator {}

impl Validator for Sha256ImmutableValidator {
    fn verify(&self, digest: &Digest, _content: Option<&Vec<u8>>, _signature: Option<&Signature>) -> bool {
        debug!("verifying digest {:?}", digest);
        true
    }
}

/// The default value of [Validator], which performs no validation.
pub struct NoopValidator {
}

impl Validator for NoopValidator {
    fn verify(&self, _digest: &Digest, _content: Option<&Vec<u8>>, _signature: Option<&Signature>) -> bool {
        true
    }
}

pub const NOOPVALIDATOR: NoopValidator = NoopValidator{};
