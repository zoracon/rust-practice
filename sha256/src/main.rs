// SHA256 Digest Hash of File

//Error Structure
use error_chain::error_chain;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};

// Command line boilerplate
use clap::{Arg, App};

// Error boilerplate
error_chain! {
    foreign_links {
        Io(std::io::Error);
        Decode(data_encoding::DecodeError);
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    // A context for multi-step digest calculations.
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024]; // A buffer of 1024 bytes

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn main() -> Result<()> {
    let matches = App::new("SHA Program")
    .version("0.1.0")
    .author("zoracon <notrealemail@hack.things>")
    .about("File Name to SHA256")
    .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("Some file"))
    .get_matches();

    let path = matches.value_of("file").unwrap_or("");
    println!("The file passed is: {}", path);

    let input = File::open(path)?;
    let reader = BufReader::new(input); //stream reading of file
    let digest = sha256_digest(reader)?;

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    // Contains the success value
    Ok(())
}
