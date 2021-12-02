use std::fmt;
use super::validator::{Validator, noopValidator};

/// Endpoint represents a single access point of a specific source, to be accessed according to the
/// source-specific schedule.
/// 
/// An endpoint also includes a validator, which will verify that content retrieved from the
/// endpoint is valid.
pub struct Endpoint<'a> {
    /// The protocol corresponding to the endpoint
    pub protocol: String,
    /// Endpoint host
    pub host: String,
    /// Endpoint port
    pub port: u16,
    /// Endpoint path
    pub path: String,
    /// Content validator for content returned from the endpoint. Enabling endpoint-specific
    /// validation allows for different signatories for different locations.
    pub validator: &'a (dyn Validator + 'a),
}

impl<'a> Endpoint<'a> {
    pub  fn new(protocol: &str, host: &str, port: &u16, path: Option<&str>, validator: Option<&dyn Validator>) -> Endpoint<'a> {
        let mut e: Endpoint = Endpoint{
            protocol: String::from(protocol),
            host: String::from(host),
            port: *port,
            path: String::from(""),
            validator: &noopValidator,
        };
        match path {
            Some(p) => {
                e.path = String::from(p);
            },
            _ => (),
        }
        e
    }

    /// Calculates the URL of a resource in the context of the specific endpoint.
    ///
    /// The endpoint will typically be the string representation of a digest.
    ///
    /// TODO: pointer should probably be of digest, or a dedicated type for reference,
    pub fn url_for(&self, pointer: &String) -> String {
        match &self.path {
            x if x.is_empty() => {
                format!("{}://{}:{}/{}", self.protocol, self.host, self.port, pointer)
            },
            _ => {
                format!("{}://{}:{}/{}/{}", self.protocol, self.host, self.port, self.path, pointer)
            },
        }
    }
}

impl<'a> fmt::Display for Endpoint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.path {
            //Some(ref v) => {
            ref v if self.path.len() > 0 => { 
                fmt::write(f, format_args!("{}://{}:{}/{}", self.protocol, self.host, self.port, v));
            },
            _ => {
                fmt::write(f, format_args!("{}://{}:{}", self.protocol, self.host, self.port));
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Endpoint;
    use crate::validator::{Sha256ImmutableValidator};
    use crate::resolver::{Sha256ImmutableResolver, Resolver};

    #[test]
    fn create() {
        let key: Vec<u8> = Vec::new();
        let content: Vec<u8> = Vec::new();
        let r: Resolver = Resolver::new();
        //let r: Sha256ImmutableResolver = Sha256ImmutableResolver{key: &key, content: &content};
        let v: Sha256ImmutableValidator = Sha256ImmutableValidator{resolver: &r};
        let p: u16 = 8080;
        let e: Endpoint = Endpoint::new("https", "localhost", &p, Some("foo"), None);
        assert_eq!(format!("{}", e), "https://localhost:8080/foo");
    }
}
