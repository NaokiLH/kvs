#[macro_use]
extern crate clap;
use log::warn;
use std::net::SocketAddr;

use clap::App;
use kvs::{KvsClient, Result};

fn main() -> Result<()> {
    let yaml = load_yaml!("kvs-client.yml");
    let m = App::from_yaml(yaml).get_matches();

    match m.subcommand() {
        ("set", Some(matches)) => {
            warn!("dsadas");
            let key = matches.value_of("KEY").unwrap().to_string();
            let value = matches.value_of("VALUE").unwrap().to_string();
            let addr: SocketAddr = matches.value_of("addr").unwrap().parse().unwrap();
            let mut client = KvsClient::connect(addr)?;
            client.set(key, value)?;
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap().to_string();
            let addr: SocketAddr = matches.value_of("addr").unwrap().parse().unwrap();
            let mut client = KvsClient::connect(addr)?;
            if let Some(value) = client.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap().to_string();
            let addr: SocketAddr = matches.value_of("addr").unwrap().parse().unwrap();
            let mut client = KvsClient::connect(addr)?;
            client.remove(key)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
