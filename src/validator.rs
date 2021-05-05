pub trait Resolver {
    fn digest(&self) -> Vec<u8>;
    fn signature(&self) -> Vec<u8>;
    fn verify(&self, digest: &Vec<u8>, signature: &Vec<u8>) -> bool;
}

pub struct Validator {
    pub resolver: Resolver,
}
