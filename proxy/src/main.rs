mod proxy;

use clap::Parser;
use env_logger;
use chrono::Local;
use log::*;
use std::net::ToSocketAddrs;
use std::fs::OpenOptions;
use std::io::Write;

use pingora::server::configuration::Opt;
use pingora::server::Server;

fn log_init() {
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
}

// RUST_LOG=INFO cargo run proxy
// curl 127.0.0.1:6191
fn main() {
    log_init();

    let opt = Opt::parse();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let mut my_proxy = pingora::proxy::http_proxy_service(
        &my_server.configuration,
        proxy::EchoProxy::new(),
    );

    my_proxy.add_tcp("127.0.0.1:6191");
    my_server.add_service(my_proxy);
    my_server.run_forever();
}