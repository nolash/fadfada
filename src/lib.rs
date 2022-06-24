#![crate_name = "fadfada"]

//! fadfada generates URL request graphs for different types of content sources.
//!
//! Its primary use-case is bridging the dependency problem between web2 and web3 resources, enabling the
//! client to specify sources from multiple protocol backends, to be tried at particular offsets in
//! time.
//!
//! The `fadfada` package is agnostic about what backend types is uses, and can just as well be used 
//! to fetch a resource from a list of web2 mirrors.
//!
//! The execution of `fadfada` is described by registering resources and schedules with the
//! `fadfada::control::Controller` object.

/// A single endpoint in a source structure.
pub mod endpoint;

/// Resolves a content resource to the pointer depending on source context.
pub mod resolver;

/// Validates content after delivery.
pub mod validator;

/// Manages keys used for content verification.
pub mod keystore;

/// Define query schedules for content sources.
pub mod timing;

/// Represents a storage engine backend, e.g. web2-sha256, swarm, IPFS etc.
pub mod source;

/// Entry-point object that orchestrates order and timing of requests.
pub mod control;

#[cfg(feature = "web2")]
pub mod web2;

#[cfg(feature = "yaml")]
pub mod yaml;

#[cfg(test)]
pub mod mock;
