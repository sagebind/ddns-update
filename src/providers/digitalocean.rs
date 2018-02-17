use chttp;
use json;
use std::net::IpAddr;
use std::error::Error;


pub fn update(domain: String, addr: IpAddr, auth: Option<String>) -> Result<(), Box<Error>> {
    if auth.is_none() {
        return Err("auth token is required".into());
    }

    let mut record_name = String::from("@");
    let mut domain_name = domain.clone();

    let parts = domain.split('.').collect::<Vec<&str>>();
    if parts.len() > 2 {
        record_name = parts[..parts.len()-2].join(".");
        domain_name = parts[parts.len()-2..].join(".");
    }

    let request = chttp::http::Request::builder()
        .uri(format!("https://api.digitalocean.com/v2/domains/{}/records", &domain_name).as_str())
        .header("Authorization", format!("Bearer {}", auth.as_ref().unwrap()).as_str())
        .body(chttp::Body::Empty)?;

    let mut response = chttp::send(request)?;
    let json = response.body_mut().json()?;
    let type_str = if addr.is_ipv4() {
        "A"
    } else {
        "AAAA"
    };

    for record in json["domain_records"].members() {
        if record["type"].as_str() == Some(type_str) && record["name"].as_str() == Some(record_name.as_str()) {
            let id = record["id"].as_u32().unwrap();
            debug!("found matching record ID: {}", id);

            let addr_string = format!("{}", addr);
            if record["data"].as_str() != Some(addr_string.as_str()) {
                info!("IP changed, updating DNS record ({} -> {})", record["data"], addr_string);
                update_record(&domain_name, id, addr, auth.as_ref().unwrap())?;
            }

            return Ok(())
        }
    }

    Err(format!("DNS record for {} not found", domain).into())
}

fn update_record(domain_name: &str, id: u32, addr: IpAddr, auth: &str) -> Result<(), Box<Error>> {
    let body = chttp::Body::from(json::stringify(object! {
        "data" => format!("{}", addr),
    }));

    let request = chttp::http::Request::builder()
        .method("PUT")
        .uri(format!("https://api.digitalocean.com/v2/domains/{}/records/{}", domain_name, id).as_str())
        .header("Authorization", format!("Bearer {}", auth).as_str())
        .body(body)?;

    chttp::send(request)?;

    Ok(())
}
