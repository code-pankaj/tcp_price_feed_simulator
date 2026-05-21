use std::net::TcpStream;
use std::io::Write;
use std::thread;
use std::time::Duration;

use crate::buffer_reader::buffer_reader;

pub fn client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let data = String::from("BTCUSD");

    stream.write_all(data.as_bytes())?;

    loop {
        let message = buffer_reader(&mut stream).expect("Error in reading stream");
        println!("{message}");

        // thread::sleep(Duration::from_millis(1500));
    }
}

// write() may write only partial bytes
// write_all() guarantees full buffer transmission or error