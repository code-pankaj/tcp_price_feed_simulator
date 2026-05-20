use std::net::TcpStream;
use std::io::{Read, Write};

pub fn client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let data = String::from("BTCUSD");

    stream.write_all(data.as_bytes())?;

    Ok(())
}

// write() may write only partial bytes
// write_all() guarantees full buffer transmission or error