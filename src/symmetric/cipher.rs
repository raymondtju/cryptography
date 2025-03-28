
fn reverse_cipher(input: &str) -> String {
    let mut reversed = String::new();
    let mut char_len = input.len();

    while char_len > 0 {
        reversed += &input.chars().nth(char_len - 1).unwrap().to_string();
        char_len -= 1;
    }
    reversed
}

fn reverse_cipher_unicode(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_cipher() {
        assert_eq!(reverse_cipher("hello"), "olleh");
        assert_eq!(reverse_cipher("rust"), "tsur");
        assert_eq!(reverse_cipher(""), "");
    }

    #[test]
    fn test_reverse_cipher_unicode() {
        assert_eq!(reverse_cipher_unicode("hello🤣"), "🤣olleh");
        assert_eq!(reverse_cipher_unicode("rust"), "tsur");
        assert_eq!(reverse_cipher_unicode(""), "");
    }
}
