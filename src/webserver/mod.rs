use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

mod utils;

use utils::{parse_filename_from_request, get_response_headers_from_file};


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

    let buffer_string = String::from_utf8_lossy(&buffer[..]);
    // println!("Request: {}", &buffer_string);

    if !buffer.starts_with(b"GET ") {
        let response = "HTTP/1.1 501 Not Implemented\r\n\r\nRequested method is not implemented";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap_or_else(|err| {
            eprintln!("Could not flush the response stream: {}", err);
        });

        return
    }

    let filename = parse_filename_from_request(&buffer_string).or_else(|| Some("index.html")).unwrap();
    let mut index_file = env::current_exe().unwrap();
    index_file.pop();
    index_file.push(format!("www/{}", filename));

    if !index_file.exists() {
        eprintln!("Requested file {} not found", filename);
        index_file.pop();
        index_file.push("404.html");
    }

    let (status_line, index_file) = if !index_file.exists() {
        eprintln!("File {} not found", index_file.display());
        (Some("HTTP/1.1 404 NOT FOUND\r\n\r\nFile not found"), None)
    } else {
        (Some("HTTP/1.1 200 OK"), Some(index_file))
    };

    let (contents, headers) = match index_file {
        Some(f) => {
            println!("FILE {} requested.", &f.display());
            match get_response_headers_from_file(&f) {
                Some(headers) => {
                    // TODO(dkg): crashes if file content is not valid utf-8....
                    let content: io::Result<String> = fs::read_to_string(&f).or_else(|_| {
                        Ok(String::from("TODO"))
                    });
                    (content.unwrap(), headers)
                },
                None => {
                    // TODO(dkg): crashes if file content is not valid utf-8....
                    let content: io::Result<String> = fs::read_to_string(&f).or_else(|_| {
                        Ok(String::from("TODO"))
                    });
                    (content.unwrap(), String::from(""))
                }
            }
            
        },
        None => (String::from(""), String::from(""))
    };

    let response = format!("{}{}\r\n\r\n{}", status_line.unwrap(), headers, contents);

    if let Err(err) = stream.write(response.as_bytes()) { 
        eprintln!("Could write to the response stream: {}", err);
    }
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
