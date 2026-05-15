use local_ip_address::local_ip;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tabled::{Table, Tabled};
use std::{thread, time};
use clap::Parser; 

// Parse command line arguments
#[derive(Parser, Debug)]
#[command(name = "quid", version = "0.1.0", about = "Perform helpful network survey for QuickBooks server.")]
struct Args {
    // Amount of time to survey for
    #[arg(short, long)]
    survey: u64,

    // Amount of time between scans
    #[arg(short, long, default_value_t = 1)]
    t_between: u8,
}

fn main() {
    let _args = Args::parse();

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

    // Eventually add time constraints here to only run for a certain period
    let mut counter = 0; 
    while counter < 10 {
        // Clear the terminal
        let sleeptime = time::Duration::from_secs(5);
        thread::sleep(sleeptime);
        clearscreen::clear().expect("Failed to clear screen");

        // Placeholder for the time being
        socket_map();
        
        println!("Counter: {}", counter); 
        draw_state();
        
        counter += 1; 
    }
}

// Pinpoint that an IP exists on the accessible subnet (needs better error handling)
fn pinpoint(ip: IpAddr) {
    let target_ip = ip;

    match ping::new(target_ip).send() {
        Ok(_) => println!("The IP address has been found! {}", ip),
        Err(e) => eprintln!("The IP address {} has not been found: {}", ip, e),
    } 
}

// Use netstat to list active network sockets (TCP/UDP) on the local machine, filtered by applicable ports
fn socket_map(){
    println!("Socket map!");
}

// Draw a table of the current state
fn draw_state(){
    #[derive(Tabled)]
    struct State {
        destination_ip: &'static str,
        source_ip: &'static str,  
        port: u16,
    }
    
    // Actual live data populates here
    let curr_state = vec![
        State { destination_ip: "192.168.0.0", source_ip: "10.0.0.0", port: 80 },
        State { destination_ip: "192.168.0.0", source_ip: "10.0.0.1", port: 443 },
        State { destination_ip: "192.168.0.0", source_ip: "10.0.0.2", port: 80 },
    ];

    let table = Table::new(curr_state);

    println!("{}", table);
}
