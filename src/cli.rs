use clap::Parser;
use std::net::Ipv4Addr;

#[derive(Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(help("The IP address of the telnet server."))]
    pub ip_addr: Ipv4Addr,
}