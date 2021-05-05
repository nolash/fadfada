use super::validator::{Validator};

pub struct Endpoint {
    pub protocol: String,
    pub validator: Validator,
}
