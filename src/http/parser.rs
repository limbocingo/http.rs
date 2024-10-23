use core::str;

use super::Header;

/// Parse HTTP formated text and get all the information that is available
/// in the text gived.
/// 
/// If the text gave has some error the program will just naturally explode.
/// Its really time consuming to add custom handling for that type of things so currently
/// we'll just let it like that.
/// 
/// ## Example
/// 
/// How to parse a basic HTTP text.
/// 
/// ```rust
/// # use http::HttpParser
///
/// # fn main() {
///     let http: String = HttpParser::new("GET /api/example HTTP/1.1\r\n\r\n")
///     http.path() // Give you the path
/// # }
/// ```
pub struct HttpParser {
    pub content: String
}

impl HttpParser {
    /// Http parser constructor.
    pub fn new(text: &str) -> Self {
        HttpParser {
            content: text.to_string()
        }
    }
    
    /// Get the headers from a string in the HTTP format.
    pub fn headers(&self) -> Header {
        let mut header: Header = Header::new();
        
        let lines: Vec<&str>  = self.content.split("\r\n").collect();
        let headers_lines: Vec<&str> = lines[1..][..lines.len() - 2].to_vec();

        let mut splitted_header_line: Vec<&str>;
        for header_line in headers_lines {
            splitted_header_line = header_line.split(": ").collect();
            
            if splitted_header_line.len() == 1 {
                continue;
            }

            header = header.add(splitted_header_line[0], splitted_header_line[1]);
        }

        return header;
    }
    
    /// Get the body from a string in the HTTP format.
    pub fn body(&self) -> String {
        return self.content
            .split("\r\n\r\n").collect::<Vec<&str>>()[1]
            .split("\r\n").collect::<Vec<&str>>()[1].to_string();
    }

    /// Get the code that the server returned.
    pub fn code(&self) -> String {
        return self.content
            // Get the first line
            .split("\r\n")
            .collect::<Vec<&str>>()[0]
            // Get the status
            .splitn(2, " ")
            .collect::<Vec<&str>>()[1].to_owned();
    }
}

/// Separate the path and hostname.
pub fn split_path_from_hostname(url: &str) -> Result<Vec<&str>, &str> {
    let separate_https_from_path: Vec<&str> = url.split("https://").collect();
    let separate_http_from_path: Vec<&str> = url.split("http://").collect();
    let path_from_url: &str;

    if separate_https_from_path.len() == 1 && 
       separate_http_from_path.len() == 1 {
        path_from_url = url;
    } 

    else if separate_https_from_path.len() > 1 {
        path_from_url = separate_https_from_path[1];
    }

    else if separate_http_from_path.len() > 1 {
        path_from_url = separate_http_from_path[1];
    }

    else {
        return Err("Cannot get the path from the URL.");
    }
    
    let separate_path_from_hostname: Vec<&str> = path_from_url.splitn(2, "/").collect();

    return Ok(separate_path_from_hostname);    
}
