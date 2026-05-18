// Separation of logic for connection diagnostics
use dns_lookup::lookup_addr;

// Get round trip time for the connection
fn get_rtt(){
    return; 
}

// Return DNS hostname information
pub fn get_dns(ip: &str) -> String {
    let address: std::net::IpAddr = ip.parse().unwrap();
    let host = lookup_addr(&address).unwrap();
    return host; 
}