use local_ip_address::local_ip;

fn main() {

    // Get local machine's IP address so initial location is known at runtime
    let local_ip = local_ip().unwrap(); 
    println!("The local IP address is {:?}", local_ip);

    pinpoint();

}

// Pinpoint the connections being made to a specific IP address
fn pinpoint() {
    println!("The IP address has been found!");
}