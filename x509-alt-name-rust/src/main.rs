use clap::{Arg, App};
use dns_lookup::{lookup_host};
use std::net::IpAddr;
use std::fs::File;
use x509_parser::pem::Pem;

// Resolves host
fn resolve_altnames(hosts: Option<(bool, &x509_parser::extensions::SubjectAlternativeName)>) -> Result<(), std::io::Error>{
  let unwp = hosts.unwrap();

  Ok(for host in &unwp.1.general_names {
    let stringed_host = format!("{:?}",host).to_string();
    let stringed_host_len = &stringed_host.len() - 2;
    let parsed_host = &stringed_host[9..stringed_host_len];

    let ips: Vec<std::net::IpAddr> = lookup_host(parsed_host).unwrap();
    format!("The hostname {:?}, resolves to these IPs: {:?}", parsed_host, ips);
  })
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
  let file = std::fs::File::open(path).unwrap();

  //Pass Pem and Parse Pem
  match resolve_altnames(x509_parser::pem::Pem::read(std::io::BufReader::new(file))
    .unwrap().0
    .parse_x509().unwrap()
    .tbs_certificate.subject_alternative_name()) {
        Ok(ok_message) => println!("{:?}", ok_message),
        Err(err) => println!("Error: {:?}", err)
    }
}

