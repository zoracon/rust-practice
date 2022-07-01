#[cfg(test)]
mod tests {
    #[test]
    fn it_converts() {
        let b64string = "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIDEzIGxhenkgZG9ncy4=";
        assert_eq!(b64string, convert_hex_to_base64("54686520717569636B2062726F776E20666F78206A756D7073206F766572203133206C617A7920646F67732E"));
    }
}