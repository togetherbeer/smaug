extern crate dotenv;

use dotenv::dotenv;
use smaug_registry::app;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let address = format!("{}:{}", "127.0.0.1", "8080");
    let listener = TcpListener::bind(address)?;

    app(listener)?.await?;
    Ok(())
}
