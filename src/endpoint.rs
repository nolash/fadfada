use std::fmt;
use super::validator::{Validator, noopValidator};

pub struct Endpoint<'a> {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub path: String,
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
    use crate::resolver::{Sha256ImmutableResolver};

    #[test]
    fn create() {
        let key: Vec<u8> = Vec::new();
        let content: Vec<u8> = Vec::new();
        let r: Sha256ImmutableResolver = Sha256ImmutableResolver{key: &key, content: &content};
        let v: Sha256ImmutableValidator = Sha256ImmutableValidator{resolver: &r};
        let p: u16 = 8080;
        let e: Endpoint = Endpoint::new("https", "localhost", &p, Some("foo"), None);
        assert_eq!(format!("{}", e), "https://localhost:8080/foo");
    }
}
