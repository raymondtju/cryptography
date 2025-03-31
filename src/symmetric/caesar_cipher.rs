pub fn encrypt(text: &str, s: i32) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_uppercase() {
            let encrypted_char = (((c as i32 - 65 + s) % 26) + 65) as u8 as char;
            result.push(encrypted_char);
        } else if c.is_ascii_lowercase() {
            let encrypted_char = (((c as i32 - 97 + s) % 26) + 97) as u8 as char;
            result.push(encrypted_char);
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt("BDn", 1), "CE ")
    }
}
