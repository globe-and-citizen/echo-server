use async_trait::async_trait;
use bytes::Bytes;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::ToSocketAddrs;

use pingora::server::configuration::Opt;
use pingora::server::Server;
use pingora::upstreams::peer::HttpPeer;
use pingora::{Error, Result};
use pingora::http::{ResponseHeader, StatusCode, Method};
use pingora::proxy::{ProxyHttp, Session};
use log::{info, error, debug, LevelFilter};

use env_logger;
use chrono::Local;
use log::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

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

fn get_method(session: &Session) -> String {
    let request_summary = session.request_summary();
    let tmp: Vec<&str> = request_summary.split(" ").collect();
    let method: &str = tmp.get(0).unwrap();
    method.to_string()
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
        let peer: Box<HttpPeer> = Box::new(HttpPeer::new(self.addr, false, UPSTREAM_HOST.to_owned()));
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        let method = get_method(session);

        let mut response_body = ResponseBody { data: String::from("") };
        let mut header = ResponseHeader::build(StatusCode::OK, None)?;

        if method == Method::POST.to_string() {
            let mut body = Vec::new();
            loop {
                match session.read_request_body().await? {
                    Some(chunk) => body.extend_from_slice(&chunk),
                    None => break,
                }
            }

            match serde_json::de::from_slice::<RequestBody>(&body) {
                Ok(request_body) => {
                    debug!("Request body: {:?}", request_body);
                    // TODO manipulate body here
                    response_body = ResponseBody { data: format!("Hello from echo server! - {}", request_body.data) }
                }
                Err(err) => {
                    error!("ERROR: {err}");
                    let _ = header.set_status(StatusCode::BAD_REQUEST);
                }
            };
        } else if method == Method::OPTIONS.to_string() {
            header.set_status(StatusCode::NO_CONTENT).unwrap();
        } else {
            let _ = header.set_status(StatusCode::METHOD_NOT_ALLOWED);
        }

        let response_bytes = serde_json::ser::to_vec(&response_body).unwrap();
        let _ = header.append_header("Content-Length", response_bytes.len().to_string());
        header.append_header("Access-Control-Allow-Origin", "*".to_string()).unwrap();
        header.append_header("Access-Control-Allow-Methods", "POST".to_string()).unwrap();
        header.append_header("Access-Control-Allow-Headers", "Content-Type".to_string()).unwrap();
        header.append_header("Access-Control-Max-Age", "86400".to_string()).unwrap();
        session.write_response_header_ref(&header).await?;
        session.write_response_body(Some(Bytes::from(response_bytes)), true).await?;

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

// RUST_LOG=INFO cargo run proxy
// curl 127.0.0.1:6191
fn main() {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .expect("Can't create file!");

    let target = Box::new(file);

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

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
                .unwrap()
        },
    );

    my_proxy.add_tcp("127.0.0.1:6191");

    my_server.add_service(my_proxy);
    my_server.run_forever();
}