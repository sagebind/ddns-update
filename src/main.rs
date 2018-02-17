extern crate chttp;
extern crate env_logger;
#[macro_use] extern crate json;
#[macro_use] extern crate log;
#[macro_use] extern crate structopt;

use std::env;
use structopt::StructOpt;

mod ip;
mod providers;


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Options {
    #[structopt(short = "d", long = "domain")]
    domain: String,

    #[structopt(short = "p", long = "provider", default_value = "do")]
    provider: providers::Provider,

    #[structopt(short = "a", long = "auth")]
    auth: Option<String>,
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
    let options = Options::from_args();

    if let Some(addr) = ip::find_public_address() {
        info!("public IP is {}", addr);

        if let Err(e) = options.provider.update(options.domain, addr, options.auth) {
            warn!("error updating DNS: {}", e);
        }
    } else {
        warn!("failed to find public IP");
    }
}
