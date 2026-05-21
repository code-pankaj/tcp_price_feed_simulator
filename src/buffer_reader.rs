use std::net::TcpStream;
use std::io::{BufRead, BufReader};



pub fn buffer_reader(reader: &mut BufReader<&mut TcpStream>) -> Result<String, Box<dyn std::error::Error>>{
    
    let mut message = String::new();
    reader.read_line(&mut message)?;

    Ok(message)
}