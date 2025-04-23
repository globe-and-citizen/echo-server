use proxy::crypto::{EchoCrypto, service::CryptoService};

#[test]
fn test_sign_and_verify() {
    let secret = b"my_secret_key";
    let message = b"Hello, world!";
    let crypto_service = CryptoService::new(secret);
    let signature = crypto_service.sign_message(message);

    assert!(crypto_service.verify_signature(message, &signature));
}