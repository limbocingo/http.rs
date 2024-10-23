use std::sync::Arc;

use tokio::net::TcpStream;

use rustls::pki_types::ServerName;

use rustls::ClientConfig;
use rustls::RootCertStore;

use tokio_rustls::client::TlsStream;
use tokio_rustls::TlsConnector;

/// Builder for setting up all the TLS encryption asynchronously
/// , for TCP connections, using the `rustls` library.
///
/// The encryption isn't made from zero and a library is needed
/// because it's too time consuming to do it at this moment.
/// Maybe in the future will be convinient.
///
/// ## Example
///
/// Create a TLS connection that can be used for TCP connections.
///
/// ```rust
/// # use http::TlsBuilder
///
/// # fn main() {
///     let mut stream = TlsBuilder::new("example.com").build(stream).await?;
///     stream.write_all("Hello, World!");
/// # }
/// ```
pub struct TlsBuilder {
    hostname: String,
}

impl TlsBuilder {
    /// Construct a new builder for TLS only by giving the 
    /// hostname, everthing else is managed by the implementation.
    pub fn new(hostname: String) -> Self {
        Self { hostname }
    }

    /// A container for root certificates able to provide a root-of-trust for connection authentication.
    pub fn certificates(&self) -> RootCertStore {
        let mut certificate: RootCertStore = RootCertStore::empty();
        certificate.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        return certificate;
    }

    /// Common configuration for all connections made by the stream.
    pub fn configuration(&self) -> ClientConfig {
        return ClientConfig::builder()
            .with_root_certificates(self.certificates())
            .with_no_client_auth();
    }

    /// Encodes ways a client can know the expected name of the server.
    pub fn name(&self) -> ServerName {
        return ServerName::try_from(self.hostname.clone()).unwrap();
    }

    // Use the given DNS and the stream to return a TLS encrypted connection.
    pub async fn build(
        &self,
        stream: TcpStream,
    ) -> Result<TlsStream<TcpStream>, Box<dyn std::error::Error>> {
        Ok(    
            TlsConnector::from(Arc::new(self.configuration()))
                .connect(self.name().to_owned(), stream)
                .await?
        )
    }
}
