use tcpfeed::{client::client, server::server};
use std::thread;
use std::time::Duration;

fn main() {
    println!("TCP Price Feed Simulator...");
    println!("");

    let handle = thread::spawn(|| {
        server().expect("Some error occurred!!");
    });

    thread::sleep(Duration::from_millis(2000));
    client().expect("Client issue");
    handle.join().unwrap();
}


// Ok(TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:64006, fd: 5 })
// addr is the server address
// peer is the client address and the port is assigned by the os 
// ephemeral port - 
// fd is file descriptor 