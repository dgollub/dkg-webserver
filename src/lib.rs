use std::env;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let socket = SocketAddrV4::new(config.address.parse::<Ipv4Addr>().unwrap(), config.port);

    let listener = TcpListener::bind(socket)?;

    println!("Running DKG Web Server at http://{}:{}/", config.address, config.port);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }

    Ok(())
}

pub struct Config {
    pub port: u16,
    pub address: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // ignore self reference

        let address = match args.next() {
            Some(arg) => arg,
            None => DEFAULT_ADDRESS.to_string(),
        };

        let port = match args.next() {
            Some(arg) => arg.parse::<u16>().expect("Please provide a valid port."),
            None => DEFAULT_PORT,
        };

        Ok(Config {
            address,
            port,
        })
    }
}


#[cfg(test)]
mod tests {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
}
