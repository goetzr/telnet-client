use std::net::Ipv4Addr;
use std::net::TcpStream;
use std::io;

pub fn connect(ip_addr: &Ipv4Addr) -> io::Result<TcpStream> {
    TcpStream::connect((ip_addr.clone(), 23))
}