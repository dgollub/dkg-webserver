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

fn get_contenttype_from_file(file: &PathBuf) -> String {
    let ext = if let Some(ext) = file.extension() {
        // TODO(dkg): error handling
        OsStr::to_str(ext)
    } else {
        Some("fallback to default")
    };

    let ext = ext.unwrap().to_lowercase();
    let content_type = match &ext[..] {
        "html" | "htm" => "text/html",
        "css" | "scss" => "text/css",
        "js" | "es6" => "text/javascript",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "mp3" => "application/mp3",
        "ico" => "image/x-icon",
        "png" => "image/png",
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        "jpg" | "jpeg" => "image/jpeg",
        "txt" | "text" | "md" => "text/plain",
        // TODO(dkg): add more mimetypes or find a crate that has everything
        _ => "application/octet-stream",
    };

    content_type.to_owned()
}

pub fn get_response_headers_from_file(filepath: &PathBuf) -> String {
    let metadata = metadata(filepath).unwrap();
    let size = metadata.len();
    // https://docs.rs/chrono/0.4.10/chrono/format/strftime/index.html
    // example: Mon, 27 Jul 2009 12:28:53 GMT
    let format = "%a, %d %b %Y %H:%M:%S %Z";
    let now = Utc::now().format(format);
    let modified = DateTime::<Utc>::from(metadata.modified().unwrap()).format(format);
    let content_type = get_contenttype_from_file(&filepath);

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
    fn get_contenttype_from_file_test() {
        let file = PathBuf::from("test/index.html");
        let expected = "text/html";
        let result = get_contenttype_from_file(&file);
        assert_eq!(expected, result);
    }

    #[test]
    fn get_contenttype_from_file_test_upper() {
        let file = PathBuf::from("index.HTML");
        let expected = "text/html";
        let result = get_contenttype_from_file(&file);
        assert_eq!(expected, result);
    }

    #[test]
    fn get_contenttype_from_file_test_htm() {
        let file = PathBuf::from("index.HTm");
        let expected = "text/html";
        let result = get_contenttype_from_file(&file);
        assert_eq!(expected, result);
    }

    #[test]
    fn get_contenttype_from_file_test_unknown() {
        let file = PathBuf::from("index.xxx");
        let expected = "application/octet-stream";
        let result = get_contenttype_from_file(&file);
        assert_eq!(expected, result);
    }

    #[test]
    fn get_contenttype_from_file_test_noext() {
        let file = PathBuf::from("some file name");
        let expected = "application/octet-stream";
        let result = get_contenttype_from_file(&file);
        assert_eq!(expected, result);
    }

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
