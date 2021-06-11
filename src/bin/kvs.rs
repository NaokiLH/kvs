#[macro_use]
extern crate clap;
use std::process::exit;

use clap::App;

fn main() {
    let yaml = load_yaml!("kvs.yml");
    let m = App::from_yaml(yaml).get_matches();

    match m.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
