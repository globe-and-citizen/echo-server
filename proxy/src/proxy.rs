use async_trait::async_trait;
use bytes::Bytes;
use std::net::ToSocketAddrs;
use log::{info};
use pingora::upstreams::peer::HttpPeer;
use pingora::{Result};
use pingora::http::{StatusCode};
use pingora::proxy::{ProxyHttp, Session};
use crate::crypto::EchoCrypto;
use crate::handler::{Handler};

pub struct EchoProxy<T: EchoCrypto> {
    addr: std::net::SocketAddr,
    handler: Handler<T>,
}

impl<T: EchoCrypto> EchoProxy<T> {
    pub(crate) fn new(upstream_host: String, upstream_port: u16, handler: Handler<T>) -> Self {
        let addr = (upstream_host, upstream_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();

        EchoProxy { addr, handler }
    }
}

#[async_trait]
impl<T: EchoCrypto + Sync> ProxyHttp for EchoProxy<T> {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer: Box<HttpPeer> = Box::new(HttpPeer::new(self.addr, false, self.addr.ip().to_string()));
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        let mut response_body_bytes = Vec::new();

        // validate request
        let mut response_status = self.handler.validate_request(session);
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
