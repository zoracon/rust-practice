use clap::{Arg, App};
use tracing::info;
// use dns_lookup::{lookup_host};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::net::IpAddr;
use std::fs::File;
use x509_parser::pem::Pem;

// Resolves host
fn resolve(host: &str) -> std::io::Result<String> {
  // Construct a new Resolver with default configuration options
  let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

  let response = resolver.lookup_ip(host).unwrap();

  let address = response.iter().next().expect("no addresses returned!");

  info!("Outputting common name...");
  let ips: IpAddr = address;
  Ok(format!("The common name {:?}, resolves to these IPs: {:?}", host, ips))
}

// Main process
fn main() {
  // Clap boilerplate for command line args
  let matches = App::new("Parse X509")
    .version("0.2.0")
    .author("Alexis Hancock")
    .about("Validate host from x509 Pem File")
    .arg(Arg::with_name("INPUT")
      .help("pem file")
      .required(true)
      .index(1))
    .get_matches();

  let path = matches.value_of("INPUT").unwrap();
  let file = File::open(path).unwrap();

  //Pass Pem and Parse Pem
  info!("Parse pem file...");
  let subject = Pem::read(std::io::BufReader::new(file))
    .unwrap().0
    .parse_x509().unwrap()
    .tbs_certificate.subject.to_string();

  //Extract Host
  let host = &subject[3..];

  // Validate Host
  info!("Validating host...");
  match resolve(host) {
    Ok(ok_message) => println!("{:?}", ok_message),
    Err(err) => println!("Error: {:?}", err)
  }
}
