use local_ip_address::local_ip;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {

    // Get localhost addresses for IPv6 and IPv4
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 1, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1));
    println!("Localhost IPV4 address: {}", localhost_v4);
    println!("Localhost IPv6 address: {}", localhost_v6); 

    // Get local machine's IP address so initial location is known at runtime
    let local_ip = local_ip().unwrap(); 
    println!("The local IP address is {:?}", local_ip);

    pinpoint();

}

// Pinpoint the connections being made to a specific IP address
fn pinpoint() {
    println!("The IP address has been found!");
}