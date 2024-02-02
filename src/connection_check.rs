use std::io::prelude::*;
use std::net::TcpStream;
pub fn check_internet_connection() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("google.com:443")?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
}