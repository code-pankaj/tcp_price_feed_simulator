use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use rand;

use crate::buffer_reader::buffer_reader;

pub fn server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut reader = BufReader::new(&mut stream);
                let coin = handle_stream(&mut reader)?;
                send_price(&coin, &mut stream);
            },
            Err(e) => println!("{e}")
        }
    }
    Ok(())
}

fn handle_stream(reader: &mut BufReader<&mut TcpStream>) -> Result<String, Box<dyn std::error::Error>> {
    
   let coin = buffer_reader(reader)?;
   println!("Client requested : {coin}");
   Ok(coin)
}

fn send_price(coin : &String, stream: &mut TcpStream) {
    loop {
        let random_number = rand::random_range(40000..90000).to_string();

        // here i used delimiter '\n' so client know where line ends 
        // this is also known as framing or protocol boundary or message framing
        let price = format!("{coin} : {random_number}\n");

        let _ = stream.write_all(price.as_bytes());

        thread::sleep(Duration::from_millis(1500));
    }
}   

// protocols are agreed rules of communication 