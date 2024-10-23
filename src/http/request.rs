use core::str;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio_rustls::client::TlsStream;

use super::{split_path_from_hostname, Header, Http, HttpParser, Method, TlsBuilder};

/// Represents a HTTP requests, adds to the [Http] implementation
/// handling of TCP sockets so you can send and recieve formated HTTP.
///     
/// ## Example
/// 
/// How to create a GET request to a API.
/// 
/// ```rust
/// # use http::request::Request
/// # use http::request::get
/// 
/// # async fn main() {
///     let request: Request = Request::new("https://api.com/api/webhooks/test/test")
///         .header("Authentication", "Token ...")
///         .send().await?;
///     let data_jsonified: request.json();
/// # }
/// ```
pub struct Request {
    pub http: Http
}

impl Request {
    /// Constructor of a HTTP request.
    pub fn new(url: &str, method: Method) -> Self {
        Self {
            http: Http::new(url).method(method).headers(
                Header::new()
                    .add("Accept", "*/*")
                    .add("Connection", "close")
                    .add("Host", split_path_from_hostname(url).unwrap()[0])
                    .add("User-Agent", "Quantun/0.1")
            )
        }
    }
    
    /// Get the connection with the HTTP server.
    async fn stream(&self) -> Result<TlsStream<TcpStream>, Box<dyn std::error::Error>> {
        return Ok(
            TlsBuilder::new(self.http.hostname.clone())
                .build(
                    TcpStream::connect(
                        (self.http.hostname.clone(), 443)
                    ).await?
                ).await?
        )
    } 


    /// Send the request to the server.
    pub async fn send(self) -> Result<HttpParser, Box<dyn std::error::Error>> {
        let mut stream: TlsStream<TcpStream> = self.stream().await?;
        
        stream.write_all(self.http.as_str().as_bytes()).await?;
        
        let mut response = String::new();
        stream.read_to_string(&mut response).await?;

        return Ok(HttpParser::new(&response.as_str()));
    }
}

/// Do a async HTTP request to a URL.
pub fn get(url: &str) -> Request {
    return Request::new(url, Method::Get);
}
