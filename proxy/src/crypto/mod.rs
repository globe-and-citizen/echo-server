pub mod service;

pub trait EchoCrypto {
    fn sign_message(&self, message: &[u8]) -> Vec<u8>;
    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> bool;
}