use local_ip_address::local_ip;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {

    // Get localhost addresses for IPv6 and IPv4
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1));
    println!("Localhost IPV4 address: {}", localhost_v4);
    println!("Localhost IPv6 address: {}", localhost_v6); 

    // Get local machine's IP address so initial location is known at runtime
    let local_ip = local_ip().unwrap(); 
    println!("The local IP address is {:?}", local_ip);

    pinpoint(local_ip);

    let bad_ip = "192.168.200.254".parse().unwrap();
    pinpoint(bad_ip);

}

// Pinpoint that an IP exists on the accessible subnet (needs better error handling)
fn pinpoint(ip: IpAddr) {
    let target_ip = ip;

    match ping::new(target_ip).send() {
        Ok(_) => println!("The IP address has been found! {}", ip),
        Err(e) => println!("The IP address {} has not been found: {}", ip, e),
    } 
}