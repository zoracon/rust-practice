use base64::{Engine as _, engine::general_purpose};

// General Purpose engine base64
// not constant-time
pub fn b64_encode(str: &str) -> String {
    let orig = str.as_bytes();
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(orig);
    return encoded;
}

// General Purpose engine base64
// not constant-time
pub fn b64_url_encode(str: &str) -> String {
    let orig = str.as_bytes();
    let encoded_url = general_purpose::URL_SAFE_NO_PAD.encode(orig);
    return encoded_url;
}

// Encode slice
pub fn b64_slice_encode(data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(data.len() * 4 / 3 + 4, 0);
    // Error handling here
    let bytes_written = general_purpose::STANDARD.encode_slice(data, &mut buf).unwrap();
    buf.truncate(bytes_written);
    return buf;
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
        //assert_eq!("base64: AAECAw==", format!("base64: {}", wrapper));
// }
