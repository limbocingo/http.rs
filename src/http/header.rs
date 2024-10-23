use std::{borrow::Borrow, collections::HashMap};

/// Simple manager for headers to make it more readable and easy
/// to manage when working with them instead of ugly plain strings.
/// 
/// Very useful with the `Http` module.
/// 
/// ## Example
/// 
/// ```rust
/// # use http::Http
/// # use http::Header
///
/// # fn main() {
///     let mut headers: Header = http::header::Header::new()
///         .add("Origin", "example.com");
/// # }
/// ```
pub struct Header {
    lines: Vec<String>,
    mapped_lines: HashMap<String, String>,
}

impl Header {
    /// Construct the header manager.
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            mapped_lines: HashMap::new(), 
        }
    }

    /// Add a new header to the current stack.
    pub fn add(mut self, key: &str, value: &str) -> Self {
        self.mapped_lines.insert(key.to_string(), value.to_string());
        self.lines.push(format!("{}: {}", 
                                  key, value));
        return self;
    }

    /// Get a header from the stack.
    pub fn get(self, key: &str) -> String {
        return self.mapped_lines.get(key)
            .unwrap_or(key.to_string().borrow())
            .to_owned();
    }

    /// Get all the headers in a [HashMap].
    pub fn all(self) -> HashMap<String, String> {
        return self.mapped_lines;
    }

    /// Get all the headers in a big string with the HTTP format.
    pub fn as_str(&mut self) -> String {
        return self.lines.join("\r\n"); 
    }
}
