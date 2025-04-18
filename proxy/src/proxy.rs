use async_trait::async_trait;
use bytes::Bytes;
use std::fmt::Error;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::ToSocketAddrs;
use std::panic::panic_any;
use log::{debug, error, info};
use pingora::upstreams::peer::HttpPeer;
use pingora::{Result};
use pingora::http::{ResponseHeader, StatusCode, Method};
use pingora::proxy::{ProxyHttp, Session};
use ring::signature::Signature;
use crate::crypto::EchoCrypto;
use crate::handler::{Handler, RequestBody, ResponseBody};

const UPSTREAM_HOST: &str = "localhost";
const UPSTREAM_IP: &str = "0.0.0.0"; //"125.235.4.59"
const UPSTREAM_PORT: u16 = 8000;

pub struct MyCtx {
    buffer: Vec<u8>,
}

pub struct EchoProxy<T: EchoCrypto> {
    addr: std::net::SocketAddr,
    handler: Handler<T>,
}

impl<T: EchoCrypto> EchoProxy<T> {
    pub(crate) fn new(handler: Handler<T>) -> Self {
        let addr = (UPSTREAM_IP.to_owned(), UPSTREAM_PORT)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();

        EchoProxy { addr, handler }
    }
}

#[async_trait]
impl<T: EchoCrypto + Sync> ProxyHttp for EchoProxy<T> {
    type CTX = MyCtx;
    fn new_ctx(&self) -> Self::CTX {
        MyCtx { buffer: vec![] }
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer: Box<HttpPeer> = Box::new(HttpPeer::new(self.addr, false, UPSTREAM_HOST.to_owned()));
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        let mut response_body_bytes = Vec::new();
        let mut response_status = StatusCode::OK;

        // validate request
        response_status = self.handler.validate_request(session);
        if response_status == StatusCode::OK {
            // handle request
            match self.handler.handle_request(session).await? {
                Some(res) => {
                    response_body_bytes = serde_json::ser::to_vec(&res).unwrap();
                }
                None => {
                    response_status = StatusCode::BAD_REQUEST;
                }
            }
        }

        // convert json response to vec
        Handler::<T>::set_headers(response_status, &response_body_bytes, session).await?;
        session.write_response_body(Some(Bytes::from(response_body_bytes)), true).await?;

        Ok(true)
    }

    async fn logging(
        &self,
        session: &mut Session,
        _e: Option<&pingora::Error>,
        ctx: &mut Self::CTX,
    ) {
        let response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());
        // access log
        info!("{} response code: {response_code}", self.request_summary(session, ctx));
    }
}
