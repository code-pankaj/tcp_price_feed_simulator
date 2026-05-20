use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use rand;

pub fn server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let coin = handle_stream(&mut stream);
                send_price(&coin, stream);
            },
            Err(e) => println!("{e}")
        }
    }
    Ok(())
}

fn handle_stream(stream: &mut TcpStream) -> String {
    
    let mut buffer = [0; 128];
    let bytes_read = stream.read(&mut buffer);

    let mut coin = String::new();
    if let Ok(bytes_count) = bytes_read {
        if let Ok(ans) = from_utf8(&buffer[..bytes_count]){
            println!("Requested from client : {ans}");
            coin = ans.to_string();
        }
    };
    coin
}

fn send_price(coin : &String, stream: TcpStream) {
    loop {
        let random_number = rand::random_range(40000..90000);

        // stream.write_all(random_number);
    }
}   