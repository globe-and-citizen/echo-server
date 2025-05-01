mod proxy;
mod crypto;
mod handler;
mod config;

use std::env;
use clap::Parser;
use env_logger;
use chrono::Local;
use log::*;
use std::fs::OpenOptions;
use std::io::Write;

use pingora::server::configuration::Opt;
use pingora::server::Server;

fn log_init(filepath: &String, level: &LevelFilter) {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)
        .expect("Can't create file!");

    let target = Box::new(file);

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, *level)
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
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());

    let echo_config = config::Config::from_file(&config_path);
    println!("{:?}", echo_config);

    log_init(&echo_config.log.path, &echo_config.log.to_level_filter());

    let crypto_service = crypto::service::CryptoService::new(echo_config.crypto.secret.as_bytes());
    let handler = handler::Handler::new(crypto_service);

    let opt = Opt::parse();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let mut my_proxy = pingora::proxy::http_proxy_service(
        &my_server.configuration,
        proxy::EchoProxy::new(echo_config.upstream.host, echo_config.upstream.port, handler),
    );

    my_proxy.add_tcp(echo_config.server.address.as_str());
    my_server.add_service(my_proxy);
    my_server.run_forever();
}