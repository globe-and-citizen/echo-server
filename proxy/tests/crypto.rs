use proxy::crypto::{EchoCrypto, service::CryptoService};

#[test]
fn sign_ok() {
    let secret = b"my_secret_key";
    let message = b"Hello, world!";
    let crypto_service = CryptoService::new(secret);
    let signature = crypto_service.sign_message(message);

    assert_eq!(signature.len(), 64);
    assert_eq!(hex::encode(&signature), "2990e2f07f205f76e04331b257a0c8369302ffc26b573d96b9100e3f4433b28bb427b1c62050aa49e5b8f1f7a25d9cb1df60b64001ee2a909a9c04e66042a905".to_string());
}

#[test]
#[should_panic]
fn sign_empty_secret() {
    let secret = b"";
    let message = b"Hello, world!";
    let crypto_service = CryptoService::new(secret);
    crypto_service.sign_message(message);
}

#[test]
fn verify_ok() {
    let secret = b"my_secret_key";
    let message = b"Hello, world!";
    let crypto_service = CryptoService::new(secret);
    let signature = hex::decode("2990e2f07f205f76e04331b257a0c8369302ffc26b573d96b9100e3f4433b28bb427b1c62050aa49e5b8f1f7a25d9cb1df60b64001ee2a909a9c04e66042a905").unwrap();
    assert!(crypto_service.verify_signature(message, &signature));
}

#[test]
#[should_panic]
fn verify_empty_secret() {
    let secret = b"";
    let message = b"Hello, world!";
    let crypto_service = CryptoService::new(secret);
    let signature = hex::decode("2990e2f07f205f76e04331b257a0c8369302ffc26b573d96b9100e3f4433b28bb427b1c62050aa49e5b8f1f7a25d9cb1df60b64001ee2a909a9c04e66042a905").unwrap();
    crypto_service.verify_signature(message, &signature);
}

#[test]
fn verify_wrong_signature() {
    let secret = b"my_secret_key";
    let message = b"Hello, world!";
    let crypto_service = CryptoService::new(secret);
    let signature = hex::decode("6883452843fa7e6a58cf13509507bd36bc345bfff8bc97e226998eabb5e400fa2a5ec6bd65d2cd27f9a1ebafc3786028d802d7d58eca66bede0db7767e03ea09").unwrap();
    assert!(!crypto_service.verify_signature(message, &signature));
}

#[test]
#[should_panic]
fn verify_invalid_signature_size() {
    let secret = b"my_secret_key";
    let message = b"Hello, world!";
    let crypto_service = CryptoService::new(secret);
    let signature = hex::decode("2990e2f07f205f76e04331b257a0c8369302ffc26b573d96b9100e3f4433b28bb427b1c62050aa49e5b8f1f7a25d9cb1df60b64001ee2a909a9c04e66042a90").unwrap();
    crypto_service.verify_signature(message, &signature);
}