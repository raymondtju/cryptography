pub fn encrypt_columnar_transposition(plaintext: &str, key: &str) -> String {
    let plaintext = plaintext.replace(" ", ""); // Remove spaces for simplicity
    let key = key.to_uppercase();
    let num_cols = key.len();
    let num_rows = (plaintext.len() as f64 / num_cols as f64).ceil() as usize;
    let mut grid: Vec<Vec<char>> = vec![vec!['\0'; num_cols]; num_rows]; // /0 is NULL

    let mut plain_iter = plaintext.chars();
    for i in 0..num_rows {
        for j in 0..num_cols {
            if let Some(c) = plain_iter.next() {
                grid[i][j] = c;
            } else {
                grid[i][j] = ' '; // use space as filler
            }
        }
    }

    println!("{:?}", grid);

    let mut key_order: Vec<(char, usize)> = key.chars().enumerate().map(|(i, c)| (c, i)).collect();
    key_order.sort_by_key(|&(c, _)| c);

    println!("{:?}", key_order);

    let mut ciphertext = String::new();
    for (index, (_, original_index)) in key_order.iter().enumerate() {
        for i in 0..num_rows {
            if grid[i][*original_index] != '\0' && grid[i][*original_index] != ' ' {
                ciphertext.push(grid[i][*original_index]);
            }
        }
        if index != key_order.len() - 1 {
            ciphertext.push(' ');
        }
    }

    ciphertext
}

pub fn decrypt_columnar_transposition(ciphertext: &str, key: &str) -> String {
    let key = key.to_uppercase();
    let num_cols = key.len();
    let num_rows = (ciphertext.len() as f64 / num_cols as f64).ceil() as usize;
    let mut grid: Vec<Vec<char>> = vec![vec!['\0'; num_cols]; num_rows];

    let mut key_order: Vec<(char, usize)> = key.chars().enumerate().map(|(i, c)| (c, i)).collect();
    key_order.sort_by_key(|&(c, _)| c);

    let mut cipher_iter = ciphertext.chars();
    for (_, original_index) in key_order {
        for i in 0..num_rows {
            if let Some(c) = cipher_iter.next() {
                grid[i][original_index] = c;
            }
        }
    }

    println!("{:?}", grid);

    let mut plaintext = String::new();
    for i in 0..num_rows {
        for j in 0..num_cols {
            if grid[i][j] != '\0' && grid[i][j] != ' ' {
                plaintext.push(grid[i][j]);
            }
        }
    }

    plaintext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        assert_eq!(
            encrypt_columnar_transposition("WE ARE DISCOVERED FLEE AT ONCE", "ZEBRAS"),
            "EVLN ACDT ESEA ROFO DEEC WIREE"
        );
    }

    #[test]
    fn test_decryption() {
        assert_eq!(
            decrypt_columnar_transposition("EVLN ACDT ESEA ROFO DEEC WIREE", "ZEBRAS"),
            "WE ARE DISCOVERED FLEE AT ONCE"
        );
    }

    #[test]
    fn test_encryption_with_spaces() {
        assert_eq!(
            encrypt_columnar_transposition("HELLO   WORLD", "KEY"),
            "EOWRLLHDL"
        );
    }

    #[test]
    fn test_decryption_with_padding() {
        assert_eq!(
            decrypt_columnar_transposition("EOWRLLHDL", "KEY"),
            "HELLOWORLD"
        );
    }
}
