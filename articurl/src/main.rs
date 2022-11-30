use arti_client::{TorClient, TorClientConfig};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use clap::{Arg, App};
use thiserror::Error;
use anyhow::{Context, Result};

// Custom Error Types
#[derive(Error, Debug)]
pub enum ArtiError {
    // Argument was empty
    #[error("No value given for host")]
    EmptyHost,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ArtiError(#[from] arti_client::Error),

    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),
}

#[tokio::main]
async fn main() -> Result<(), ArtiError> {

    let matches = App::new("Arti Curl")
    .version("0.1.0")
    .author("zoonarc <zooanrc@duck.com>")
    .about("HTTP request on Arti")
    .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .help("host name"))
    .get_matches();

    let path = matches.value_of("host").unwrap_or("");

    if path.is_empty() {
        return Err(ArtiError::EmptyHost);
    } else {
        println!("The host passed is: {}", path);
    }

    let config = TorClientConfig::default();
    let tor_client = TorClient::create_bootstrapped(config).await.context(format!("Tor failed to bootstrap"))?;

    let mut stream = tor_client.connect((path, 80)).await.context(format!("Unable to connect to domain, possibly incorrect or inactive domain"))?;
    let pathb = format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path);
    stream
        .write_all(pathb.as_bytes())
        .await
        .context(format!("Stream failed to write"))?;
    stream.flush().await.context(format!("Stream failed to flush"))?;

    let mut buf = vec![];
    stream.read_to_end(&mut buf).await.context(format!("Stream failed to read to the end"))?;

    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
} 