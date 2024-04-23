fn encrypt(plaintext: &str, key: u16) -> String {
    let mut encryptpt = String::new();
    for c in plaintext.chars() {
        let mut new_char = c as u16;
        if c.is_ascii_alphabetic() {
            new_char += key;
            // Ensure it stays within uppercase or lowercase ASCII ranges
            if c.is_ascii_uppercase() {
                if new_char > 'Z' as u16 {
                    new_char -= 26;  // Wrap around the alphabet
                }
            } else if c.is_ascii_lowercase() {
                if new_char > 'z' as u16 {
                    new_char -= 26;  // Wrap around the alphabet
                }
            }
        }
        encryptpt.push(char::from_u32(new_char as u32).unwrap_or(c));  // Convert back to char safely
    }
    encryptpt
}

fn decrypt(encrypted: &str, key: u16) -> String {
    let mut decryptpt = String::new();
    for c in encrypted.chars() {
        let mut new_char = c as u16;
        if c.is_ascii_alphabetic() {
            new_char -= key;
            // Ensure it stays within uppercase or lowercase ASCII ranges
            if c.is_ascii_uppercase() {
                if new_char < 'A' as u16 {
                    new_char += 26;  // Wrap around the alphabet
                }
            } else if c.is_ascii_lowercase() {
                if new_char < 'a' as u16 {
                    new_char += 26;  // Wrap around the alphabet
                }
            }
        }
        decryptpt.push(char::from_u32(new_char as u32).unwrap_or(c));  // Convert back to char safely
    }
    decryptpt
}

fn main() {
    let plaintext = "//66363999993???=0300399938¿¿¿¡¿¿¿¿¿++``++++";
    let key = 5;
    let encrypted = encrypt(plaintext, key);
    println!("Encrypted: {}", encrypted);
    let decrypted = decrypt(&encrypted, key);
    println!("Decrypted: {}", decrypted);
}
