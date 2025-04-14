// Copyright 2025 Cloudflare, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use async_trait::async_trait;
use bytes::Bytes;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::ToSocketAddrs;

use pingora::server::configuration::Opt;
use pingora::server::Server;
use pingora::upstreams::peer::HttpPeer;
use pingora::Result;
use pingora::http::ResponseHeader;
use pingora::proxy::{ProxyHttp, Session};

const UPSTREAM_HOST: &str = "localhost";
const UPSTREAM_IP: &str = "0.0.0.0"; //"125.235.4.59"
const UPSTREAM_PORT: u16 = 8000;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    data: String,
}

pub struct EchoProxy {
    addr: std::net::SocketAddr,
}

pub struct MyCtx {
    buffer: Vec<u8>,
}

#[async_trait]
impl ProxyHttp for EchoProxy {
    type CTX = MyCtx;
    fn new_ctx(&self) -> Self::CTX {
        MyCtx { buffer: vec![] }
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer = Box::new(HttpPeer::new(self.addr, false, UPSTREAM_HOST.to_owned()));
        Ok(peer)
    }


    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        let mut body = Vec::new();
        loop {
            match session.read_request_body().await? {
                Some(chunk) => body.extend_from_slice(&chunk),
                None => break,
            }
        }

        let json_body: RequestBody = serde_json::de::from_slice(&body).unwrap();
        println!("{:?}", json_body);

        // TODO manipulate body
        let response_body = ResponseBody{data: format!("Hello from echo server! - {}", json_body.data)};
        let response_bytes = serde_json::ser::to_vec(&response_body).unwrap();

        let mut header = ResponseHeader::build(200, None)?;
        header.append_header("Content-Length", response_bytes.len().to_string());

        session.write_response_header_ref(&header).await?;
        session
            .write_response_body(Some(Bytes::from(response_bytes)), true)
            .await?;

        Ok(true)
    }
}

// RUST_LOG=INFO cargo run proxy
// curl 127.0.0.1:6191
fn main() {
    env_logger::init();

    let opt = Opt::parse();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let mut my_proxy = pingora::proxy::http_proxy_service(
        &my_server.configuration,
        EchoProxy {
            addr: (UPSTREAM_IP.to_owned(), UPSTREAM_PORT)
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap(),
        },
    );

    my_proxy.add_tcp("127.0.0.1:6191");

    my_server.add_service(my_proxy);
    my_server.run_forever();
}