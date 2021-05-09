use super::keystore::{Fingerprint};
use super::timing::{Scheduler};
use super::endpoint::{Endpoint};

pub struct Source<'a> {
    pub trusted_keys: Vec<Fingerprint>,
    pub endpoints: Vec<Endpoint<'a>>,
    pub timing: Option<Scheduler>,
}


#[cfg(test)]
mod tests {
    use super::Source;
    use super::Scheduler;
    use super::Endpoint;
    use crate::validator::{Sha256ImmutableValidator};
    use crate::resolver::{Sha256ImmutableResolver};

    #[test]
    fn create() {
        let key: Vec<u8> = Vec::new();
        let content: Vec<u8> = Vec::new();
        let r: Sha256ImmutableResolver = Sha256ImmutableResolver{key: &key, content: &content};
        let v: Sha256ImmutableValidator = Sha256ImmutableValidator{resolver: &r};
        let p: u16 = 8080;
        let e: Endpoint = Endpoint::new("https", "localhost", &p, Some("foo"), None);
        let h: Scheduler = Scheduler{
            delay: 42,
            timeout: 13,
            };
        let mut ep: Vec<Endpoint> = Vec::new();
        ep.push(e);
        let s: Source = Source{
            trusted_keys: Vec::new(),
            endpoints: ep,
            timing: Some(h),
            };
    }
}
