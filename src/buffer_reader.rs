use std::fmt::Error;
use std::net::TcpStream;
use std::io::Read;
use std::str::from_utf8;

pub fn buffer_reader(stream: &mut TcpStream) -> Result<String, Error>{
    let mut buffer: [u8;128] = [0;128];
    let buffer_size = stream.read(&mut buffer).expect("Error writing stream to buffer.");
    
    let message = from_utf8(&buffer[..buffer_size]).expect("Error converting buffer to string slice.");

    Ok(message.to_string())
}