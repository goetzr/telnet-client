use std::net::TcpStream;
use std::io::{Read, BufReader};

pub struct Client {
    reader: BufReader<TcpStream>,
}

impl Client {
    pub fn new(server_stream: TcpStream) -> Self {
        Client { reader: BufReader::new(server_stream) }
    }

    pub fn log_rcvd_data(&mut self) {
        let mut buf = [0u8; 100];
        let num_bytes_rcvd = match self.reader.read(&mut buf) {
            Ok(n) => n,
            Err(e) => {
                error!("Failed to log received data: {}", e);
                return;
            },
        };
        let mut hex_string = String::new();
        for i in 0..num_bytes_rcvd {
            let hex_digits = format!("{:02X}", buf[i]);
            hex_string.push_str(&hex_digits);
        }
        info!("Received data: {}", hex_string);
    }
}
