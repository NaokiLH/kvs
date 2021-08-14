#[macro_use]
extern crate clap;
use clap::App;
use kvs::*;
use log::LevelFilter;
use log::{error, info, warn};
use std::env::current_dir;
use std::fs;
use std::net::SocketAddr;
use std::process::exit;
const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Engine {
        Kvs,
        Sled
    }
}
fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let yaml = load_yaml!("kvs-server.yml");
    let m = App::from_yaml(yaml).get_matches();

    let addr = match m.value_of("addr") {
        Some(addr) => addr,
        _ => DEFAULT_LISTENING_ADDRESS,
    };
    let opt_engine = match m.value_of("engine") {
        Some(str) if str == "sled" => Engine::Sled,
        _ => Engine::Kvs,
    };

    let engine = match current_engine() {
        Ok(Some(engine)) if engine != opt_engine => {
            error!("Wrong engine!");
            exit(1);
        }
        _ => opt_engine,
    };

    info!("sadasd");
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {}", engine);
    info!("Listening on {}", addr);
    fs::write(current_dir()?.join("engine"), format!("{}", engine))?;
    let addr: SocketAddr = addr.parse().unwrap();

    match engine {
        Engine::Kvs => run_with_engine(KvStore::open(current_dir()?)?, addr),
        Engine::Sled => run_with_engine(SledKvsEngine::new(sled::open(current_dir()?)?), addr),
    }
}
fn run_with_engine<E: KvsEngine>(engine: E, addr: SocketAddr) -> Result<()> {
    let server = KvsServer::new(engine);
    server.run(addr)
}
fn current_engine() -> Result<Option<Engine>> {
    let engine = current_dir()?.join("engine");
    if !engine.exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine) {
        Ok(str) if str.as_str() == "kvs" => Ok(Some(Engine::Kvs)),
        Ok(str) if str.as_str() == "sled" => Ok(Some(Engine::Sled)),
        _ => {
            warn!("the content of engine file is invalid");
            Ok(None)
        }
    }
}
