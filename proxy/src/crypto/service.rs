use ring::digest;
use ring::signature::{Ed25519KeyPair, KeyPair};
use crate::crypto::{EchoCrypto};

#[derive(Debug)]
pub struct CryptoService {
    keypair: Option<Ed25519KeyPair>, // Keypair is optional for testing purpose
}

impl CryptoService {
    pub fn new(secret: &[u8]) -> Self {
        // This should throw a panic if the secret is empty but we let it be for now (testing purpose)
        if secret.len() == 0 {
            return Self {
                keypair: None,
            }
        }
        // Hash the secret using SHA256
        let seed_bytes = digest::digest(&digest::SHA256, secret).to_owned();
        Self {
            keypair: Some(Ed25519KeyPair::from_seed_unchecked(seed_bytes.as_ref()).expect("Failed to create keypair"))
        }
    }

    pub fn get_keypair(&self) -> Result<&Ed25519KeyPair, &'static str> {
        match self.keypair {
            Some(ref keypair) => Ok(keypair),
            None => Err("Keypair not initialized"),
        }
    }
}

impl EchoCrypto for CryptoService {

    fn sign_message(&self, message: &[u8]) -> Vec<u8> {
        let keypair = self.get_keypair().expect("Keypair not initialized");
        let signature = keypair.sign(message);
        signature.as_ref().to_vec()
    }

    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> bool {
        let keypair = self.get_keypair().expect("Keypair not initialized");
        let public_key = keypair.public_key();
        let peer_public_key = ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key.as_ref());
        peer_public_key.verify(message, signature).is_ok()
    }
}


// this is only for unittest practice, see proxy/tests/crypto.rs for more test coverage
#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::EchoCrypto;

    #[test]
    fn test_sign_and_verify() {
        let secret = b"my_secret_key";
        let message = b"Hello, world!";
        let crypto_service = CryptoService::new(secret);
        let signature = crypto_service.sign_message(message);

        assert!(crypto_service.verify_signature(message, &signature));
    }
}
