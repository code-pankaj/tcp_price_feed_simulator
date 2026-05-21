use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use rand;
use std::fmt::Error;

use crate::buffer_reader::buffer_reader;

pub fn server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let coin = handle_stream(&mut stream).expect("Error retrieving coin details.");
                send_price(&coin, &mut stream);
            },
            Err(e) => println!("{e}")
        }
    }
    Ok(())
}

// will be replaced by buffer_reader module
fn handle_stream(mut stream: &mut TcpStream) -> Result<String, Error> {
    
   let coin = buffer_reader(&mut stream).expect("Error reading stream");
   println!("Client requested : {coin}");
   Ok(coin)
}

fn send_price(coin : &String, stream: &mut TcpStream) {
    loop {
        let random_number = rand::random_range(40000..90000).to_string();
        let price = format!("{coin} : {random_number}\n");
        let _ = stream.write_all(price.as_bytes());

        thread::sleep(Duration::from_millis(1500));
    }
}   