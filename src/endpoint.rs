use std::{
    fmt,
    path,
};
use crate::validator::{
    Validator,
    noopValidator,
};

use url::Url;

/// [Endpoint] represents a single access point of a specific source, to be accessed according to the
/// source-specific schedule.
/// 
/// An endpoint also includes a [Validator], which will verify that content retrieved from the
/// endpoint is valid.
pub struct Endpoint<'a> {
    /// Endpoint url
    pub url: Url,
    /// Content validator for content returned from the endpoint. Enabling endpoint-specific
    /// validation allows for different signatories for different locations.
    pub validator: &'a (dyn Validator + 'a),
}

impl<'a> Endpoint<'a> {
    //pub fn new(protocol: &str, host: Option<String>, port: Option<u16>, path: Option<String>, validator: Option<&dyn Validator>) -> Endpoint<'a> {
    pub fn new(endpoint_url_src: &str, validator: Option<&dyn Validator>) -> Endpoint<'a> {
        let endpoint_url = Url::parse(endpoint_url_src).unwrap();
        Endpoint{
            url: endpoint_url,
            validator: &noopValidator,
        }        
    }

    /// Calculates the URL of a resource in the context of the specific endpoint.
    ///
    /// The endpoint will typically be the string representation of a digest.
    ///
    /// TODO: pointer should probably be of [Digest](crate::resolver::Digest), or a dedicated type for reference,
    /// TODO: enforce zero port for schemes that do not have ports associated with them (file)
    pub fn url_for(&self, pointer: &str) -> String {
        let mut pointer_url = self.url.clone();
        //let pointer_path: String;
//        match pointer_url.path() {
//             => {
//                let new_path = path::Path::new(s)
//                    .join(pointer);
//            },
//            None => {
//                pointer_path = pointer;
//            },
//        }
        let new_path = path::Path::new(self.url.path())
            .join(pointer);
        pointer_url.set_path(new_path.to_str().unwrap());
        return pointer_url.to_string();
    }
}

impl<'a> fmt::Display for Endpoint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          fmt::write(f, format_args!("{}", self.url.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::Endpoint;
    use url::Url;

    #[test]
    fn test_endpoint_create() {
        //let mut orig_url = Url::parse("https://localhost:8521/foo").unwrap();
        let orig_url = "https://localhost:8521/foo";
        //let mut e: Endpoint = Endpoint::new("https", "localhost", &p, Some("foo"), None);
        let mut e: Endpoint = Endpoint::new(orig_url, None);
        assert_eq!(format!("{}", e), "https://localhost:8521/foo");
    }

    #[test]
    fn test_endpoint_pointer() {
        //let mut orig_url = Url::parse("https://localhost:8521/foo").unwrap();
        let orig_url = "https://localhost:8521/foo";
        //let mut e: Endpoint = Endpoint::new("https", "localhost", &p, Some("foo"), None);
        let mut e: Endpoint = Endpoint::new(orig_url, None);
        let endpoint_url = e.url_for("deadbeef");
        assert_eq!(format!("{}", endpoint_url), "https://localhost:8521/foo/deadbeef");
    }

    #[test]
    fn test_endpoint_file() {
        //let mut orig_url = Url::parse("file:///tmp/foobar").unwrap();
        let orig_url = "file:///tmp/foobar";
        //let mut e: Endpoint = Endpoint::new("https", "localhost", &p, Some("foo"), None);
        let mut e: Endpoint = Endpoint::new(orig_url, None);
        let endpoint_url = e.url_for("deadbeef");
        assert_eq!(format!("{}", endpoint_url), "file:///tmp/foobar/deadbeef");
    }

}
