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
        let data = &buf[..num_bytes_rcvd];
        let data: Vec<_> = data.iter().map(|b| format!("{}", b)).collect();
        let data_str = data.join(",");
        info!("Received data: {}", data_str);
    }
}
