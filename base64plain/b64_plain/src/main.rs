use b64_plain::*;

fn main() {
    let url = "https://docs.rs/base64/0.21.2/base64/index.html#using-predefined-engines";
    let s = b"hello internet!";
    println!("general: {:?}", b64_encode("test"));
    println!("URL: {:?}", b64_url_encode(url));
    println!("Bytes: {:?}", b64_slice_encode(s));
}