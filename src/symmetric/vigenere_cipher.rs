pub fn encrypt(plaintext: &str, key: &str) -> String {
    let plaintext = plaintext.to_uppercase();
    let key = key.to_uppercase();
    let mut ciphertext = String::new();
    let key_len = key.len();
    let mut key_index = 0;

    for char in plaintext.chars() {
        if char.is_alphabetic() {
            let key_char = key.chars().nth(key_index % key_len).unwrap();
            let shift = (key_char as u8) - (b'A'); // difference from A to key char
            let encrypted_char = (((char as u8) - (b'A') + shift) % 26) + (b'A');
            // deduct the plain from A to self, then add shift, after that, add back from A back
            // let say if the plain was Z, shift was C, 90-65=25 + 3 = 28 => >26 => 28%26=2 + 65 = B
            ciphertext.push(encrypted_char as char);
            key_index += 1;
        } else {
            ciphertext.push(char);
        }
    }
    ciphertext
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let ciphertext = ciphertext.to_uppercase();
    let key = key.to_uppercase();
    let mut plaintext = String::new();
    let key_len = key.len();
    let mut key_index = 0;

    for char in ciphertext.chars() {
        if char.is_alphabetic() {
            let key_char = key.chars().nth(key_index % key_len).unwrap();
            let shift = (key_char as u8) - (b'A');
            let decrypted_char = (((char as u8) - (b'A') + 26 - shift) % 26) + (b'A');
            plaintext.push(decrypted_char as char);
            key_index += 1;
        } else {
            plaintext.push(char);
        }
    }
    plaintext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        assert_eq!(encrypt("ATTACKATDAWN", "LEMON"), "LXFOPVEFRNHR");
        assert_eq!(encrypt("HELLO", "KEYKE"), "RIJVS");
        assert_eq!(encrypt("ABCD", "ABCD"), "BDFH");
        // assert_eq!(encrypt("RUST", "CODE"), "UYVW");
        // assert_eq!(encrypt("VIGENERE", "CIPHER"), "XMQEPPVSZ");
    }

    #[test]
    fn test_decryption() {
        assert_eq!(decrypt("LXFOPVEFRNHR", "LEMON"), "ATTACKATDAWN");
        assert_eq!(decrypt("RIJVS", "KEYKE"), "HELLO");
        // assert_eq!(decrypt("UYVW", "CODE"), "RUST");
        // assert_eq!(decrypt("XMQEPPVSZ", "CIPHER"), "VIGENERE");
    }

    #[test]
    fn test_encryption_with_non_alpha() {
        assert_eq!(encrypt("HELLO 123 WORLD!", "KEY"), "RIJVS 123 ZVRLD!");
    }

    #[test]
    fn test_decryption_with_non_alpha() {
        assert_eq!(decrypt("RIJVS 123 ZVRLD!", "KEY"), "HELLO 123 WORLD!");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(encrypt("", "KEY"), "");
        assert_eq!(decrypt("", "KEY"), "");
        assert_eq!(encrypt("HELLO", ""), "HELLO");
        assert_eq!(decrypt("HELLO", ""), "HELLO");
    }
}
