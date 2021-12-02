#![crate_name = "fadafada"]

//! fadafada generates URL request graphs for different types of content sources.
//!
//! Its primary use-case is bridging the dependency problem between web2 and web3 resources, enabling the
//! client to specify sources from multiple protocol backends, to be tried at particular offsets in
//! time.
//!
//! The `fadafada` package is agnostic about what backend types is uses, and can just as well be used 
//! to fetch a resource from a list of web2 mirrors.
//!
//! The execution of `fadafada` is described by registering resources and schedules with the
//! `fadafada::control::Controller` object.

/// A single endpoint in a source structure.
pub mod endpoint;

/// Resolves a content resource to the pointer depending on source context.
pub mod resolver;


mod validator;
mod keystore;
mod timing;

pub mod source;
mod control;
