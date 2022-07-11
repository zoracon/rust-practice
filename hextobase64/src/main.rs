use base64;
use hex;

// Command line boilerplate
use clap::{Arg, Command};

//Error Structure
use error_chain::error_chain;

// Error boilerplate
// Provides proper Result object
error_chain! {
    foreign_links {
        Io(std::io::Error);
        Decode(data_encoding::DecodeError);
    }
}

// Hex String to Base64
pub fn convert_hex_to_base64(hex: &str) -> String {
    let c = base64::encode(hex::decode(hex).unwrap());
    println!("Base64 String: {}", c);
    return c;
}

fn main() -> Result<()> {
    let matches = Command::new("Hex to Base64")
    .version("0.1.0")
    .author("zoracon <zoonarc@duck.com>")
    .about("Convert Hex String to base64")
    .arg(Arg::new("HEXSTRING")
            .long("string")
            .required(true)
            .takes_value(true)
            .help("hex string"))
    .get_matches();

    let string = matches.value_of("HEXSTRING").unwrap(); // Unwrap okay because argument is required
    convert_hex_to_base64(string);

    // Contains the success value
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts() {
        let b64string = "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIDEzIGxhenkgZG9ncy4=";
        assert_eq!(b64string, convert_hex_to_base64("54686520717569636B2062726F776E20666F78206A756D7073206F766572203133206C617A7920646F67732E"));
    }
}