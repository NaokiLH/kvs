#[macro_use]
extern crate clap;
use clap::App;
const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
fn main() {
    let yaml = load_yaml!("kvs-server.yml");
    let m = App::from_yaml(yaml).get_matches();
}
