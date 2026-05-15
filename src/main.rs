use config::Config;
use local_ip_address::local_ip;
use netstat::*;
use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}};
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
    // Set up args from parsed CLI
    let args = Args::parse();

    // Set up read from config
    let configurations = Config::builder()
        .add_source(config::File::with_name("src/config"))
        .build()
        .unwrap();

    let ports: Vec<u16>  = configurations.get_array("ports").unwrap()
        .into_iter()
        .map(|v| v.into_int().unwrap() as u16)
        .collect();    

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

    // Time constraints so the survey runs for a predetermined period
    let between_survey = time::Duration::from_secs(args.t_between as u64);
    let survey_duration = time::Duration::from_secs(args.survey);
    let start_time = std::time::Instant::now(); 

    // The main progression of the loop here
    while start_time.elapsed() < survey_duration {
        // Placeholder for the time being -- map the sockets to a structure the draw_state function can use to make a table
        socket_map(&ports);
    
        // Placeholder for the time being -- draw table of current socket state
        draw_state();

        // Clear the terminal after a wait
        thread::sleep(between_survey);
        clearscreen::clear().expect("Failed to clear screen");
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

// Match values that are useful according to our configuration
fn fine_value (si: &SocketInfo, ports: &Vec<u16>) -> bool{

    // Match socket info value to Tcp or Udp variants
    match &si.protocol_socket_info {
        ProtocolSocketInfo::Tcp(tcp_si) if ports.contains(&tcp_si.local_port) => return true,
            //"TCP {}:{} -> {}:{} {:?} - {}",
            //tcp_si.local_addr,
            //tcp_si.local_port,
            //tcp_si.remote_addr,
            //tcp_si.remote_port,
            //si.associated_pids,
            //tcp_si.state
        
        ProtocolSocketInfo::Tcp(_) => return false,

        //ProtocolSocketInfo::Udp(udp_si) => println!(
        //    "UDP {}:{} -> *:* {:?}",
        //    udp_si.local_addr, udp_si.local_port, si.associated_pids
        //),

        ProtocolSocketInfo::Udp(_) => return false,
    }
}

// Use netstat to list active network sockets (TCP/UDP) on the local machine, filtered by applicable ports
fn socket_map(ports: &Vec<u16>){
    println!("Socket map!");

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();

    // Set up vectors for TCP and UDP info
    let mut tcp_info: Vec<SocketInfo> = Vec::new();
    //let mut udp_info: Vec<UdpSocketInfo> = Vec::new();

    for si in sockets_info {
        if fine_value(&si, &ports) == true {
            //append to proper vector
            println!("Appending to proper socket vector.");
            tcp_info.push(si); 
        }
    }

    println!("{:#?}", tcp_info);

    // Return a vector compatible with the table drawing function
    return;  
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
