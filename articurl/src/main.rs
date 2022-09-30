use arti_client::{TorClient, TorClientConfig};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use clap::{Arg, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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
    println!("The host passed is: {}", path);

    let config = TorClientConfig::default();
    let tor_client = TorClient::create_bootstrapped(config).await?;

    let mut stream = tor_client.connect((path, 80)).await?;
    let pathb = format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path);
    stream
        .write_all(pathb.as_bytes())
        .await?;
    stream.flush().await?;

    let mut buf = vec![];
    stream.read_to_end(&mut buf).await?;

    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
} 