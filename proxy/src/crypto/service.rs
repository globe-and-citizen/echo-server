use ring::digest;
use ring::signature::{Ed25519KeyPair, KeyPair};
use std::sync::OnceLock;
use crate::crypto::{EchoCrypto};

#[derive(Default, Debug)]
pub struct CryptoService {
    locked_keypair: OnceLock<Ed25519KeyPair>,
}

impl CryptoService {
    pub fn new(secret: &[u8]) -> Self {
        // Hash the secret using SHA256
        let seed_bytes = digest::digest(&digest::SHA256, secret).to_owned();

        // Create the keypair using the hashed secret
        let keypair = Ed25519KeyPair::from_seed_unchecked(seed_bytes.as_ref()).expect("Failed to create keypair");

        // Store the keypair in the OnceLock
        let once_lock_keypair = OnceLock::new();
        once_lock_keypair.get_or_init(|| { keypair });

        Self {
            locked_keypair: once_lock_keypair,
        }
    }
}

impl EchoCrypto for CryptoService {

    fn sign_message(&self, message: &[u8]) -> Vec<u8> {
        let keypair = self.locked_keypair.get().expect("Keypair not initialized");
        let signature = keypair.sign(message);
        signature.as_ref().to_vec()
    }

    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> bool {
        let keypair = self.locked_keypair.get().expect("Keypair not initialized");
        let public_key = keypair.public_key();
        let peer_public_key = ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key.as_ref());
        peer_public_key.verify(message, signature).is_ok()
    }
}


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
