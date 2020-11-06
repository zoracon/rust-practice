use clap::{Arg, App};
use dns_lookup::{lookup_host};
use std::fs::File;
use x509_parser::pem::Pem;

// Resolves host
fn resolve_altnames(hosts: Option<(bool, &x509_parser::extensions::SubjectAlternativeName)>) -> std::io::Result<String> {
  let unwp = hosts.unwrap();
  let mut _parsed_host = String::from("Init");
  let mut parsed_host_concat = Vec::<String>::new();
  let mut ips = Vec::new();

  for host in &unwp.1.general_names {
    let stringed_host = format!("{:?}",host).to_string();
    let stringed_host_len = &stringed_host.len() - 2;
    _parsed_host = stringed_host[9..stringed_host_len].to_string();
    parsed_host_concat.push(String::from(&_parsed_host));
    ips.append(&mut lookup_host(&_parsed_host)?);
  }

  Ok(format!("The alt names {:?}, resolves to these IPs: {:?}", parsed_host_concat, ips))
}

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
  match resolve_altnames(Pem::read(std::io::BufReader::new(file))
    .unwrap().0
    .parse_x509().unwrap()
    .tbs_certificate.subject_alternative_name()) {
        Ok(ok_message) => println!("{:?}", ok_message),
        Err(err) => println!("Error: {:?}", err)
    }

  // #[cfg(test)]
  // mod tests {
  //     use super::*;

  //     #[test]
  //     fn test_file() {
  //       let mut test_pem = File::open("foo.pem").unwrap();
  //       assert_eq!(test_pem[..4], ".pem");
  //     }
  // }
}

