use log::debug;

use super::resolver::{
    Digest,
    Signature,
};

/// The [Validator] is optionally used with a result from a [Source](crate:.source::Source) to verify the origin of
/// content.
pub trait Validator {

    /// Returns `true` if [Signature] can be verified against the [Digest] of content.
    fn verify(&self, digest: &Digest, content: Option<&Vec<u8>>, signature: Option<&Signature>) -> bool;
}

/// The default value of [Validator], which performs no validation.
pub struct NoopValidator {
}

impl Validator for NoopValidator {
    fn verify(&self, digest: &Digest, _content: Option<&Vec<u8>>, _signature: Option<&Signature>) -> bool {
        debug!("noop validator verify digest {:?}", digest);
        true
    }
}

pub const NOOPVALIDATOR: NoopValidator = NoopValidator{};
