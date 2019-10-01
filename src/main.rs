/*
* Function to encrypt a plaintext string using a Caesar cipher-like method.
*
* Parameters:
*   - plaintext: A reference to a string slice (&str) containing the original text to be encrypted.
*   - key: A 16-bit unsigned integer (u16) that specifies the number of positions each letter will be shifted.
*
* Returns:
*   - A `String` containing the encrypted text, where each letter is shifted by the specified key value.
*     If the shift goes beyond the end of the alphabet, it wraps around to the beginning.
*
* Notes:
*   - Non-alphabetic characters are not shifted; they are kept as is.
*   - If the shift results in an invalid character (outside of ASCII), it reverts to the original character.
*/

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
/*
* Function to decrypt an encrypted string using a Caesar cipher-like method.
*
* Parameters:
*   - encrypted: A reference to a string slice (&str) containing the text to be decrypted.
*   - key: A 16-bit unsigned integer (u16) that specifies the number of positions each letter will be shifted backward to restore the original text.
*
* Returns:
*   - A `String` containing the decrypted text, where each letter is shifted back by the specified key value.
*     If the shift goes below the beginning of the alphabet, it wraps around to the end.
*
* Notes:
*   - Non-alphabetic characters are not shifted; they remain unchanged.
*   - If the backward shift results in an invalid character, it reverts to the original character to ensure data integrity.
*/

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
