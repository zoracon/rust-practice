use arti_client::{TorClient, TorClientConfig};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use clap::{Arg, App};
// Great compliments for error handling
use thiserror::Error;
use anyhow::{Context, Result};

// Custom Error Type
#[derive(Error, Debug)]
pub enum ArtiError {
    // Argument was empty
    #[error("No value given for host")]
    EmptyHost,

    // Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    // Represents all other cases of `arti_client::Error`.
    #[error(transparent)]
    ArtiError(#[from] arti_client::Error),

    // Represents all other cases of `anyhow::Error`.
    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),
}

#[tokio::main]
async fn main() -> Result<(), ArtiError> {

    // Sorry I like Clap.
    let matches = App::new("Arti Curl")
    .version("0.1.0")
    .author("zoonarc <zooanrc@duck.com>")
    .about("HTTP request on Arti")
    .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .help("host name"))
    .get_matches();

    let path = matches.value_of("host").unwrap_or(""); // May not need to touch this since there's an appropriate default value

    if path.is_empty() {
        return Err(ArtiError::EmptyHost);
    } else {
        println!("The host passed is: {}", path);
    }

    // Bootstrap Tor
    let config = TorClientConfig::default();
    let tor_client = TorClient::create_bootstrapped(config).await.context(format!("Tor failed to bootstrap"))?;

    // Tor client connect to host
    let mut stream = tor_client.connect((path, 80)).await.context(format!("Unable to connect to domain, possibly incorrect or inactive domain"))?;
    let pathb = format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path);
    stream
        .write_all(pathb.as_bytes())
        .await
        .context(format!("Stream failed to write"))?;
    stream.flush().await.context(format!("Stream failed to flush"))?;

    // Format and print back response from host
    let mut buf = vec![];
    stream.read_to_end(&mut buf).await.context(format!("Stream failed to read to the end"))?;

    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
}