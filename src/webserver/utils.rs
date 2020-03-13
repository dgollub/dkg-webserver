use std::ffi::OsStr;
use std::fs::metadata;
use std::path::PathBuf;

extern crate chrono;
use chrono::{DateTime, Utc};

const SERVER_NAME: &str = "DKG WebServer";
const SERVER_VERSION: &str = "0.1 alpha";

/// Parse "GET / HTTP/1.1\r\n" to get the path and file name.
/// If the path is equal to / return None.
pub fn parse_filename_from_request(line: &str) -> Option<&str> {
    let mut parts = line.split_ascii_whitespace();
    if let Some(file) = parts.nth(1) {
        if file == "/" {
            return None;
        }
        return Some(&file[1..]);
    }

    None
}

pub fn is_html_file(filepath: &PathBuf) -> bool {
    if let Some(ext) = filepath.extension() {
        let lower = ext.to_string_lossy().to_ascii_lowercase();
        lower.eq("html") || lower.eq("htm")
    } else {
        false
    }
}

pub fn get_default_response_headers() -> String {
    // https://docs.rs/chrono/0.4.10/chrono/format/strftime/index.html
    // example: Mon, 27 Jul 2009 12:28:53 GMT
    let format = "%a, %d %b %Y %H:%M:%S %Z";
    let now = Utc::now().format(format);

    format!(
        "Date: {}\n\
    Server: {}/{}\n\
    Last-Modified: {}\n\
    Connection: Closed",
        now, SERVER_NAME, SERVER_VERSION, now
    )
}

pub fn get_response_headers_from_file(filepath: &PathBuf) -> String {
    let metadata = metadata(filepath).unwrap();
    let size = metadata.len();
    // https://docs.rs/chrono/0.4.10/chrono/format/strftime/index.html
    // example: Mon, 27 Jul 2009 12:28:53 GMT
    let format = "%a, %d %b %Y %H:%M:%S %Z";
    let now = Utc::now().format(format);
    let modified = DateTime::<Utc>::from(metadata.modified().unwrap()).format(format);

    let ext = if let Some(ext) = filepath.extension() {
        // TODO(dkg): error handling
        OsStr::to_str(ext)
    } else {
        Some("txt")
    };

    let content_type = match ext {
        Some("html") | Some("htm") => "text/html",
        Some("css") | Some("scss") => "text/css",
        Some("js") | Some("es6") => "text/javascript",
        Some("pdf") => "application/pdf",
        Some("zip") => "application/zip",
        Some("mp3") => "application/mp3",
        Some("ico") => "image/x-icon",
        Some("png") => "image/png",
        Some("bmp") => "image/bmp",
        Some("gif") => "image/gif",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        // TODO(dkg): add more mimetypes or find a crate that has everything
        _ => "text/plain",
    };

    format!(
        "Date: {}\n\
    Server: {}/{}\n\
    Last-Modified: {}\n\
    Content-Length: {}\n\
    Content-Type: {}\n\
    Connection: Closed",
        now, SERVER_NAME, SERVER_VERSION, modified, size, content_type
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_filename_from_request_none() {
        let line = "GET / HTTP/1.1\r\n";
        assert_eq!(None, parse_filename_from_request(&line));
    }

    #[test]
    fn parse_filename_from_request_pathtofile() {
        let line = "GET /path/to/file.html HTTP/1.1\r\n";
        assert_eq!(
            Some("path/to/file.html"),
            parse_filename_from_request(&line)
        );
    }

    #[test]
    fn parse_filename_from_request_other() {
        let line = "GET /other.css HTTP/1.1\r\n";
        assert_eq!(Some("other.css"), parse_filename_from_request(&line));
    }

    #[test]
    fn parse_filename_from_request_nonsense() {
        let line = "GETindex.htmlHTTP/1.1\r\n";
        assert_eq!(None, parse_filename_from_request(&line));
    }
}
