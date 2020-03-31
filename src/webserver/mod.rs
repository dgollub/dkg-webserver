use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

mod utils;
use utils::{
    get_default_response_headers, get_response_headers_from_file, is_html_file,
    parse_filename_from_request,
};

use super::threadpool::ThreadPool;


const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let socket = SocketAddrV4::new(config.address.parse::<Ipv4Addr>().unwrap(), config.port);
    let listener = TcpListener::bind(socket)?;
    let pool = ThreadPool::new(8);

    println!(
        "Running DKG Web Server at http://{}:{}/",
        config.address, config.port
    );

    for stream in listener.incoming() {
        println!("Connection established!");

        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            },
            Err(err) => eprintln!("ERR! Could not get incoming stream because: {}", err),
        };
    }

    println!("Shutting down.");

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    // TODO(dkg): should probably read the whole request string here ...
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let buffer_string = String::from_utf8_lossy(&buffer[..]);

    if !buffer.starts_with(b"GET ") {
        let response = "HTTP/1.1 501 Not Implemented\r\n\r\nRequested method is not implemented";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap_or_else(|err| {
            eprintln!("Could not flush the response stream: {}", err);
        });

        return;
    }

    let filename = parse_filename_from_request(&buffer_string)
        .or_else(|| Some("index.html"))
        .unwrap();
    let mut index_file = env::current_exe().unwrap();
    index_file.pop();
    index_file.push(format!("www/{}", filename));

    // Only for html files we want to replace a not-found file with our custom 404.html.
    if !index_file.exists() && is_html_file(&index_file) {
        index_file.pop();
        index_file.push("404.html");
    }

    let (status_line, index_file) = if !index_file.exists() {
        eprintln!("File {} not found", index_file.display());
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", None)
    } else {
        ("HTTP/1.1 200 OK", Some(index_file))
    };

    let (headers, content) = match index_file {
        None => (get_default_response_headers(), vec![]),
        Some(f) => {
            println!("FILE {} requested.", &f.display());
            // TODO(dkg): this is not a good idea if the file size exceeds a certain size ...
            let content = fs::read(&f).unwrap_or_else(|err| {
                eprintln!("Could not read the file {}: {}", &f.display(), err);
                vec![]
            });
            (get_response_headers_from_file(&f), content)
        }
    };

    let response = format!("{}{}\r\n\r\n", status_line, headers);
    if let Err(err) = stream.write(response.as_bytes()) {
        eprintln!(
            "Could write status line and headers to the response stream: {}",
            err
        );
    }

    // TODO(dkg): (stable) rustc doesn't let me write it in one if expression/statement yet
    if content.len() > 0 {
        if let Err(err) = stream.write(&content) {
            eprintln!(
                "Could not write the file content to the response stream: {}",
                err
            );
        }
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

        Ok(Config { address, port })
    }
}
