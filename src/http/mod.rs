//! Module occupied when making requests to a HTTP server and
//! handling the data and all the information that is sent.
//! 
//! Everything in this module is made from scratch expect from the
//! TLS, basically because its very difficult and time consuming to
//! handle. Because of that the crate `rustls` is used as an temporal
//! alternative.
//! 
//! ## Example
//! 
//! Do a GET request and get the content of the response as JSON and
//! serialize it into a struct.
//! 
//! ```rust
//! # use http::request::Request
//! # use http::request::get
//! 
//! # async fn main() {
//!     let request: Request = get("example.com", "/api/webhooks/test/test")
//!         .header("Authentication", "Token ...")
//!         .send().await?;
//!     let data_jsonified: request.json();
//! # }
//! ```

pub mod header;
pub mod tls;
pub mod request;
pub mod util;

pub use self::header::*;
pub use self::tls::*;
pub use self::util::*;

/// All the methods that can be used when making a HTTP
/// request to a server.
pub enum Method {
    /// Indicates the `DELETE` method only.
    Delete,
    /// Indicates the `GET` method only.
    Get,
    /// Indicates the `PATCH` method only.
    Patch,
    /// Indicates the `POST` method only.
    Post,
    /// Indicates the `PUT` method only.
    Put,
}

impl Method {
    /// Get each off the methods as a [String].
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Method::Delete => "DELETE".to_string(),
            Method::Get => "GET".to_string(),
            Method::Patch => "PATCH".to_string(),
            Method::Post => "POST".to_string(),
            Method::Put => "PUT".to_string(),
        }
    }
}

/// Handle the creation of the HTTP response that you should
/// be giving to the server when making a request.
/// 
/// ## Example
/// 
/// Set the method, body and headers of your request and get 
/// the response using the method `as_str`.
/// 
/// ```rust
/// # use http::Http
/// # use http::Header
///
/// # fn main() {
///     let mut headers: Header = http::header::Header::new()
///         .add("Origin", "example.com");
///
///     let http: String = Http::new("https://example.com/api/webhooks/52345/6da2355rf".to_string())
///         .method(http::Method::Get)
///         .headers(headers)
///         .body("{\"test\": 45}".to_string())
///         .as_str();
/// # }
/// ```
pub struct Http {
    body: Option<String>,
    headers: Option<Header>,
    url: String,
    path: String,
    hostname: String,
    method: Option<String>,
}


impl Http {
    /// Construct the HTTP handler.
    pub fn new(url: &str) -> Self {
        Self {
            body: Default::default(),
            headers: Default::default(),
            url: url.to_string(),
            path: format!("/{}", split_path_from_hostname(url).unwrap()[1].to_string()),
            hostname: split_path_from_hostname(url).unwrap()[0].to_string(),
            method: Default::default(),
        }
    }

    /// Sets the headers of the HTTP content.
    pub fn headers(mut self, headers: Header) -> Self {
        self.headers = Some(headers);
        return self;
    }

    /// Sets the body of the HTTP content.
    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        return self;
    }

    /// Sets the method of the HTTP content.
    pub fn method(mut self, method: Method) -> Self {
        self.body = Some(method.as_str().to_string());
        return self;
    }


    /// Format a string into the class [Http] to handle
    /// the HTTP more easily.
    pub fn from_str(url: &str, text: &str) -> HttpParser {
        return HttpParser::new(text);
    }

    /// Get a [String] formatted in the HTTP language.
    pub fn as_str(self) -> String {
        return format!(
            "{} {} HTTP/1.1\r\n{}\r\n\r\n", 
            self.method.unwrap_or("GET".to_string()), self.path,
            self.headers.unwrap().as_str()
        );
    }
}
