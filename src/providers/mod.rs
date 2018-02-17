use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;

mod digitalocean;


#[derive(Clone, Copy, Debug)]
pub enum Provider {
    DigitalOcean,
}

impl FromStr for Provider {
    type Err = String;

    fn from_str(s: &str) -> Result<Provider, String> {
        match s.to_lowercase().as_str() {
            "digitalocean" | "do" => Ok(Provider::DigitalOcean),
            _ => Err("unknown provider".into()),
        }
    }
}

impl Provider {
    pub fn update(&self, domain: String, addr: IpAddr, auth: Option<String>) -> Result<(), Box<Error>> {
        match self {
            &Provider::DigitalOcean => digitalocean::update(domain, addr, auth),
        }
    }
}
