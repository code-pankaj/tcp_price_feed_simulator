use std::net::TcpStream;
use std::io::{BufReader, Write};

use crate::buffer_reader::buffer_reader;

pub fn client() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let data = String::from("BTCUSD\n");

    stream.write_all(data.as_bytes())?;

    let mut reader = BufReader::new(&mut stream);
    loop {
        let message = buffer_reader(&mut reader)?;
        println!("{message}");
    }
}

// write() may write only partial bytes
// write_all() guarantees full buffer transmission or error