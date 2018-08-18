# DDNS Update

A small C# application and Docker image that periodically checks your system's public IP address and updates a DNS record of your choosing with that IP address.

## Usage

Parameters are taken from environment variables:

- `DDNS_PROVIDER`: DNS provider to talk to for updating the DNS record. Required. Currently supported providers:
    - `digitalocean`: This provider also requires the `DIGITALOCEAN_ACCESS_TOKEN` variable to be set.
- `DDNS_DOMAIN`: The domain name to update. Required.
- `DDNS_RECORD`: The name of the DNS record to update. Required.
- `DDNS_INTERVAL`: How often the record should be checked, in seconds. Defaults to 10 minutes.
