use std::net::SocketAddrV4;
use std::net::TcpStream;
use std::io;

pub fn connect(server_addr: SocketAddrV4) -> io::Result<TcpStream> {
    TcpStream::connect(server_addr)
}