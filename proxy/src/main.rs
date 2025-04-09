use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;

// fn main() {
//     let mut my_server = Server::new(None).unwrap();
//     my_server.bootstrap();
//     my_server.run_forever();
// }

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {

    /// For this small example, we don't need context storage
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        println!("upstream peer is: {upstream:?}");

        // Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request.insert_header("Host", "one.one.one.one").unwrap();
        Ok(())
    }
}

fn main() {
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let upstreams =
        LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443"]).unwrap();

    let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
    lb.add_tcp("0.0.0.0:6188");

    my_server.add_service(lb);

    my_server.run_forever();
}

//
// use async_trait::async_trait;
// use clap::Parser;
// use log::info;
// use pingora_core::services::background::background_service;
// use std::{sync::Arc, time::Duration};
//
// use pingora_core::server::configuration::Opt;
// use pingora_core::server::Server;
// use pingora_core::upstreams::peer::HttpPeer;
// use pingora_core::Result;
// use pingora_load_balancing::{health_check, selection::RoundRobin, LoadBalancer};
// use pingora_proxy::{ProxyHttp, Session};
//
// pub struct LB(Arc<LoadBalancer<RoundRobin>>);
//
// #[async_trait]
// impl ProxyHttp for LB {
//     type CTX = ();
//     fn new_ctx(&self) -> Self::CTX {}
//
//     async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
//         let upstream = self
//             .0
//             .select(b"", 256) // hash doesn't matter
//             .unwrap();
//
//         info!("upstream peer is: {:?}", upstream);
//
//         let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
//         Ok(peer)
//     }
//
//     async fn upstream_request_filter(
//         &self,
//         _session: &mut Session,
//         upstream_request: &mut pingora_http::RequestHeader,
//         _ctx: &mut Self::CTX,
//     ) -> Result<()> {
//         upstream_request
//             .insert_header("Host", "one.one.one.one")
//             .unwrap();
//         Ok(())
//     }
// }
//
// // RUST_LOG=INFO cargo run --example load_balancer
// fn main() {
//     env_logger::init();
//
//     // read command line arguments
//     let opt = Opt::parse();
//     let mut my_server = Server::new(Some(opt)).unwrap();
//     my_server.bootstrap();
//
//     // 127.0.0.1:343" is just a bad server
//     let mut upstreams =
//         LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443", "127.0.0.1:343"]).unwrap();
//
//     // We add health check in the background so that the bad server is never selected.
//     let hc = health_check::TcpHealthCheck::new();
//     upstreams.set_health_check(hc);
//     upstreams.health_check_frequency = Some(Duration::from_secs(1));
//
//     let background = background_service("health check", upstreams);
//
//     let upstreams = background.task();
//
//     let mut lb = pingora_proxy::http_proxy_service(&my_server.configuration, LB(upstreams));
//     lb.add_tcp("0.0.0.0:6188");
//
//     let cert_path = format!("{}/tests/keys/server.crt", env!("CARGO_MANIFEST_DIR"));
//     let key_path = format!("{}/tests/keys/key.pem", env!("CARGO_MANIFEST_DIR"));
//
//     let mut tls_settings =
//         pingora_core::listeners::tls::TlsSettings::intermediate(&cert_path, &key_path).unwrap();
//     tls_settings.enable_h2();
//     lb.add_tls_with_settings("0.0.0.0:6189", None, tls_settings);
//
//     my_server.add_service(lb);
//     my_server.add_service(background);
//     my_server.run_forever();
// }