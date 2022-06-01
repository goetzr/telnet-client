use clap::{command, Arg};
use std::net::{Ipv4Addr, SocketAddrV4, AddrParseError};
use std::str::FromStr;
use thiserror::Error;
use std::num::ParseIntError;

pub fn parse_arguments() -> Result<CliArgs, CliError> {
    const SERVER_ADDR_ARG: &'static str = "SERVER-ADDR";
    let matches = command!()
        .arg(
            Arg::new(SERVER_ADDR_ARG)
                .takes_value(true)
                .required(true)
                .help("The address of the TELNET server. Specified as <IP>[:<PORT>]. If the port is not specified, the default TELNET port 23 is used.")
        ).get_matches();

    let server_addr = matches.value_of(SERVER_ADDR_ARG).unwrap().parse::<ServerAddr>()?.0;
    Ok(CliArgs { server_addr })
}

pub struct CliArgs {
    pub server_addr: SocketAddrV4,
}

struct ServerAddr(SocketAddrV4);

impl FromStr for ServerAddr {
    type Err = ServerAddrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        const TELNET_PORT: u16 = 23;
        let port = match parts.len() {
            1 => TELNET_PORT,
            2 => parts[1].parse::<u16>()?,
            _ => return Err(ServerAddrError::new("Invalid format. Expected <IP>:<PORT>.")),
        };
        let ip: Ipv4Addr = parts[0].parse()?;
        let sock_addr = SocketAddrV4::new(ip, port); 
        Ok(ServerAddr(sock_addr))
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct ServerAddrError(String);

impl ServerAddrError {
    fn new(s: &str) -> Self {
        ServerAddrError(String::from(s))
    }
}

impl From<AddrParseError> for ServerAddrError {
    fn from(e: AddrParseError) -> Self {
        let mut msg = String::from("Invalid IP address: ");
        msg += &e.to_string();
        ServerAddrError(msg)
    }
}

impl From<ParseIntError> for ServerAddrError {
    fn from(e: ParseIntError) -> Self {
        let mut msg = String::from("Invalid port: ");
        msg += &e.to_string();
        ServerAddrError(msg)
    }
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Server address error: {0}")]
    ServerAddrError(#[from] ServerAddrError),
}