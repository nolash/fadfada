use std::{
    fmt,
    path,
};
use crate::validator::{
    Validator,
    NOOPVALIDATOR,
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
    pub fn new(endpoint_url_src: &str, _validator: Option<&dyn Validator>) -> Endpoint<'a> {
        let endpoint_url = Url::parse(endpoint_url_src).unwrap();
        Endpoint{
            url: endpoint_url,
            validator: &NOOPVALIDATOR,
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

    #[test]
    fn test_endpoint_create() {
        let orig_url = "https://localhost:8521/foo";
        let e: Endpoint = Endpoint::new(orig_url, None);
        assert_eq!(format!("{}", e), "https://localhost:8521/foo");
    }

    #[test]
    fn test_endpoint_pointer() {
        let orig_url = "https://localhost:8521/foo";
        let e: Endpoint = Endpoint::new(orig_url, None);
        let endpoint_url = e.url_for("deadbeef");
        assert_eq!(format!("{}", endpoint_url), "https://localhost:8521/foo/deadbeef");
    }

    #[test]
    fn test_endpoint_file() {
        let orig_url = "file:///tmp/foobar";
        let e: Endpoint = Endpoint::new(orig_url, None);
        let endpoint_url = e.url_for("deadbeef");
        assert_eq!(format!("{}", endpoint_url), "file:///tmp/foobar/deadbeef");
    }

}
