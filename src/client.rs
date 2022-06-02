use std::{net::TcpStream, io::BufReader};

pub struct Client {
    server_stream: TcpStream,
}

impl Client {
    pub fn new(server_stream: TcpStream) -> Self {
        Client { server_stream }
    }

    pub fn negotiate_options() {
        let mut reader = BufReader::new(self)
    }
}