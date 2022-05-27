use clap::Parser;
#[macro_use]
extern crate log;

mod cli;
mod net;

macro_rules! may_fail {
    ($expr:expr, $msg:expr) => ({
        match $expr {
            Ok(result) => result,
            Err(err) => {
                error!("{}: {}", $msg, err);
                std::process::exit(1);
            }
        }  
    });
}

fn main() {
    env_logger::init();

    let cli = cli::Cli::parse();
    let _server_sock = may_fail!(net::connect(&cli.ip_addr), "Failed to connect to the server");
}
