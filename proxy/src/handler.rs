use std::panic::panic_any;
use log::{debug, error};
use pingora::http::{Method, ResponseHeader, StatusCode};
use pingora::prelude::Session;
use serde::{Deserialize, Serialize};
use crate::crypto::EchoCrypto;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    data: String,
    signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    result: String,
}

pub(crate) struct Handler<T: EchoCrypto> {
    routes: Vec<String>,
    crypto_service: T,
}

impl<T: EchoCrypto> Handler<T> {
    pub(crate) fn new(crypto_service: T) -> Self {

        // todo routes are hardcoded for now
        let routes = vec![
            "sign".to_string(),
            "verify".to_string(),
        ];

        Handler { routes, crypto_service }
    }

    fn extract_request_summary(session: &Session) -> (String, String) {
        let request_summary = session.request_summary();
        let parts: Vec<&str> = request_summary.split_whitespace().collect();

        if parts.len() > 1 {
            let method = parts[0].to_string();
            let path = parts[1]
                .split('/')
                .collect::<Vec<&str>>()
                .get(1) // todo
                .unwrap_or(&"")
                .trim_end_matches(',')
                .to_string();
            (method, path)
        } else {
            error!("Invalid request summary: {}", request_summary);
            (String::new(), String::new())
        }
    }

    pub(crate) fn validate_request(&self, session: &Session) -> StatusCode {
        let (method, path) = Handler::<T>::extract_request_summary(session);

        // only POST method is allowed for now
        if method == Method::POST.to_string() {
            // check if path is allowed
            if self.routes.contains(&path) {
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            }
            // browser always sends an OPTIONS request along with POST for 'application/json' content-type
        } else if method == Method::OPTIONS.to_string() {
            StatusCode::NO_CONTENT
        } else {
            StatusCode::METHOD_NOT_ALLOWED
        }
    }

    async fn get_request_body(session: &mut Session) -> Option<RequestBody> {
        // read request body
        let mut body = Vec::new();
        loop {
            match session.read_request_body().await {
                Ok(option) => {
                    match option {
                        Some(chunk) => body.extend_from_slice(&chunk),
                        None => break,
                    }
                }
                Err(err) => {
                    error!("ERROR: {err}");
                    break;
                }
            }
        }

        // convert to json
        match serde_json::de::from_slice::<RequestBody>(&body) {
            Ok(request_body) => {
                debug!("Request body: {:?}", request_body);
                Some(request_body)
            }
            Err(err) => {
                error!("ERROR: {err}");
                None
            }
        }
    }

    fn handle_sign_request(&self, request_body: &RequestBody) -> pingora::Result<Option<ResponseBody>> {
        // sign the data
        let signature = self.crypto_service.sign_message(&request_body.data.as_bytes());
        let hex_signature = hex::encode(signature);

        Ok(Some(ResponseBody { result: hex_signature }))
    }

    fn handle_verify_request(&self, request_body: &RequestBody) -> pingora::Result<Option<ResponseBody>> {
        match &request_body.signature {
            Some(signature) => {
                debug!("Signature: {:?}", signature);
                // convert hex signature to bytes
                let signature = hex::decode(signature).unwrap_or_else(|_| {
                    error!("Failed to decode hex signature");
                    vec![]
                });

                // verify the data
                let is_valid = self.crypto_service.verify_signature(
                    &request_body.data.as_bytes(),
                    signature.as_ref(),
                );

                if is_valid {
                    Ok(Some(ResponseBody { result: "valid".to_string() }))
                } else {
                    Ok(Some(ResponseBody { result: "invalid".to_string() }))
                }
            }
            None => {
                error!("Signature is missing");
                Ok(None)
            }
        }
    }

    pub(crate) async fn handle_request(&self, session: &mut Session) -> pingora::Result<Option<ResponseBody>> {
        // read request body
        match Handler::<T>::get_request_body(session).await {
            Some(request_body) => {
                debug!("Request body: {:?}", request_body);

                let (_, route) = Handler::<T>::extract_request_summary(session);
                match route.as_str() {
                    "verify" => {
                        self.handle_verify_request(&request_body)
                    }
                    "sign" => {
                        self.handle_sign_request(&request_body)
                    }
                    _ => {
                        panic_any("this line shouldn't be reached because of the validate_request method");
                    }
                }
            }
            None => {
                Ok(None)
            }
        }
    }

    pub(crate) async fn set_headers(response_status: StatusCode, body_bytes: &Vec<u8>, session: &mut Session) -> pingora::Result<()> {
        let mut header = ResponseHeader::build(response_status, None)?;
        header.append_header("Content-Length", body_bytes.len().to_string()).unwrap();
        // access headers below are needed to pass browser's policy
        header.append_header("Access-Control-Allow-Origin", "*".to_string()).unwrap();
        header.append_header("Access-Control-Allow-Methods", "POST".to_string()).unwrap();
        header.append_header("Access-Control-Allow-Headers", "Content-Type".to_string()).unwrap();
        header.append_header("Access-Control-Max-Age", "86400".to_string()).unwrap();
        session.write_response_header_ref(&header).await
    }
}