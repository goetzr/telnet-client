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

    let args = may_fail!(cli::parse_arguments(), "Failed to parse command line");
    info!("Connecting to TELNET server at {}...", args.server_addr);
    let _server_sock = may_fail!(net::connect(args.server_addr), "Failed to connect to the server");
}
