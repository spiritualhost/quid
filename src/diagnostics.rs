// Separation of logic for connection diagnostics
use dns_lookup::lookup_addr;

#[allow(dead_code)] //So warnings are suppressed for unused fields

// Return DNS hostname information
pub fn get_dns(ip: &str) -> String {
    let address: std::net::IpAddr = ip.parse().unwrap();
    let host = lookup_addr(&address).unwrap();
    return host; 
}