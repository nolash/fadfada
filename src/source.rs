use std::fmt;

use super::keystore::{Fingerprint};
use super::timing::{Scheduler};
use super::endpoint::{Endpoint};

/// The [Engine] is use as a lookup key for resources for a particular [Source] backend.
///
/// For example it is used to retrieve a pointers from a [Resolver](crate::resolver::Resolver) to
/// build [Endpoint]  URLs.
pub type Engine = String;

/// [Source] encapsulates one or more endpoints to access content using a particular storage
/// backend [Engine].
///
/// A [Scheduler] may also be included to define with what delay endpoints should be queried in
/// sequence. 
///
/// Lastly, an array of public keys or public key digests may be provided to verify the origin of
/// the content.
pub struct Source<'a> {
    pub trusted_keys: Vec<Fingerprint>,
    pub endpoints: Vec<Endpoint<'a>>,
    pub timing: Option<Scheduler>,
    pub engine: Engine,
}

impl<'a> Source<'a> {
    pub fn new(engine: Engine) -> Self {
        Source {
            trusted_keys: vec![],
            endpoints: vec![],
            timing: None,
            engine: engine,
        }
    }
}

impl<'a> fmt::Debug for Source<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::write(f, format_args!("registering source engine {}", self.engine))
//        self.endpoints.iter().for_each(|endpoint| {
//            let r = fmt::write(f, format_args!("source engine {} registered endpoint {}", self.engine, endpoint));
//            match r {
//                Err(e) => {
//                    error!("debug render error for source engine {}", self.engine);
//                },
//                _ => {},
//            }
//        });
//        Ok(())
    }
}

//#[cfg(test)]
//mod tests {
//    use super::Source;
//    use super::Scheduler;
//    use super::Endpoint;
//    use super::Engine;
//    use crate::validator::{Sha256ImmutableValidator};
//
//    use super::test::resolver::{TestResolverItem};
//    use crate::resolver::{Resolver};
//
//    #[test]
//    fn create_source() {
//        let key: Vec<u8> = Vec::new();
//        let content: Vec<u8> = Vec::new();
//        let mut r: Resolver = Resolver::new();
//        let ri = TestResolverItem{key: &key, content: None};
//        let engine_string: Engine = "sha256".to_string();
//        r.add(engine_string, &ri);
//        let v: Sha256ImmutableValidator = Sha256ImmutableValidator{resolver: &r};
//        let p: u16 = 8080;
//        let e: Endpoint = Endpoint::new("https", "localhost", &p, Some("foo"), None);
//        let h: Scheduler = Scheduler{
//            delay: 42,
//            timeout: 13,
//            };
//        let mut ep: Vec<Endpoint> = Vec::new();
//        ep.push(e);
//        let s: Source = Source{
//            trusted_keys: Vec::new(),
//            endpoints: ep,
//            timing: Some(h),
//            engine: "foo".to_string(),
//        };
//    }
//}
