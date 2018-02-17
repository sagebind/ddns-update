use chttp;
use std::net::IpAddr;


/// Attempt to find the public IP address of the current network.
pub fn find_public_address() -> Option<IpAddr> {
    if let Ok(mut response) = chttp::get("https://api.ipify.org") {
        if let Ok(text) = response.body_mut().text() {
            if let Ok(addr) = text.parse() {
                return Some(addr);
            }
        }
    }

    None
}
