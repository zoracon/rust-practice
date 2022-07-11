// Command line boilerplate
use clap::{Arg, Command};
use hex::{decode, encode};

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

pub fn fixed_xor(hex1: &str, hex2: &str) -> String {
    let bytes1 = decode(hex1).unwrap();
    let bytes2 = decode(hex2).unwrap();

    let xor_bytes: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();

   
    let hex_string = encode(xor_bytes);
    println!("{}", hex_string); // Prints XOR'd Bytes from Hex string
    return hex_string;

}

fn main() -> Result<()> {
    let matches = Command::new("XOR")
    .version("0.1.0")
    .author("zoracon <zoonarc@duck.com>")
    .about("XOR 2")
    .arg(Arg::new("STRING1")
            .long("string1")
            .required(true)
            .takes_value(true)
            .help("hex string 1"))
    .arg(Arg::new("STRING2")
            .long("string2")
            .required(true)
            .takes_value(true)
            .help("hex string 2"))
    .get_matches();

    let string1 = matches.value_of("STRING1").unwrap(); // Unwrap okay because argument is required
    let string2 = matches.value_of("STRING2").unwrap(); // Unwrap okay because argument is required

    fixed_xor(string1, string2);

    // Contains the success value
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor() {
        let str1 = "48656c6c6f20776f726c6421";
        let str2 = "48656c6c6f20776f726c6425";
        let hexstr = "000000000000000000000004";
        assert_eq!(hexstr, fixed_xor(str1, str2));
    }
}

