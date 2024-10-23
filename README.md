# Http 🔎
Make very basic HTTP requests. Simple project only to add up to the protfolio.

## How it works? ⚙️
```rust
use http::request::Request
use http::request::get;

async fn main() {
    let request: Request = get("example.com", "/api/webhooks/test/test")
        .header("Authentication", "Token ...")
        .send().await?;
    let data_jsonified: request.json();
}
```

## Support 🍃
Fork the project and send a pull request.
If the content inside the request is usefull and helps the project will be added.
