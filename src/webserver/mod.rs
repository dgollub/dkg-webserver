use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};


const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let socket = SocketAddrV4::new(config.address.parse::<Ipv4Addr>().unwrap(), config.port);

    let listener = TcpListener::bind(socket)?;

    println!("Running DKG Web Server at http://{}:{}/", config.address, config.port);

    for stream in listener.incoming() {
        println!("Connection established!");

        // TODO(dkg): fix error handling to not crash the whole server....
        let stream = stream?;

        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 256];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut index_file = env::current_exe().unwrap();
    index_file.pop();
    index_file.push(format!("www/{}", filename));
    
    if !index_file.exists() {
        index_file.pop();
        index_file.push("www/404.html");
    }

    let (status_line, index_file) = if !index_file.exists() {
        eprintln!("File {} not found", index_file.display());
        (Some("HTTP/1.1 404 NOT FOUND\r\n\r\n"), None)
    } else {
        (Some(status_line), Some(index_file))
    };

    let contents = match index_file {
        Some(f) => fs::read_to_string(&f).unwrap(),
        None => String::from("")
    };
    let response = format!("{}\r\n\r\n{}", status_line.unwrap(), contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap_or_else(|err| {
        eprintln!("Could not flush the response stream: {}", err);
    })
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
            // TODO(dkg): this parse should be bubbled up if it fails....
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
