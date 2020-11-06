use clap::{Arg, App};
use dns_lookup::{lookup_host};
use std::net::IpAddr;
use std::fs::File;
use x509_parser::pem::Pem;

// Resolves host
fn resolve(host: &str) -> std::io::Result<String> {
  let ips: Vec<std::net::IpAddr> = lookup_host(host)?;
  Ok(format!("The common name {:?}, resolves to these IPs: {:?}", host, ips))
}

// Main process
fn main() {
  // Clap boilerplate for command line args
  let matches = App::new("Parse X509")
    .version("0.1.0")
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
  let subject = Pem::read(std::io::BufReader::new(file))
    .unwrap().0
    .parse_x509().unwrap()
    .tbs_certificate.subject.to_string();

  //Extract Host
  let host = &subject[3..];

  // Validate Host
  match resolve(host) {
    Ok(ok_message) => println!("{:?}", ok_message),
    Err(err) => println!("Error: {:?}", err)
  }
}
