fn main() {
    for i in 0..26 {
        println!("With key {i}");
        cesar_decrypt("vcfgrwqwoizcuwgtowbsobhgoizcuwsgsghqseisqsghoixcifrviwxcifrstshsqcaasbhsghqseisjcigbsgojsndogeishobhrsgofhwgobgjcigbsrsjsndogjcigacbhfsfibxcifcijfwsfgobgojcwfzsgwubsgrsjcgdfctsggwcbgdofzshcweiszsghhcbashwsf", i);
        println!("\n")
    }
}

fn cesar_encrypt(pt_text: &str, key: u8) {
    /// Function which handles the Cesar encryption according to a certain key
    let text = pt_text.to_uppercase();
    let mut cipher_text = String::new();
    for c in text.bytes() {
        if c.is_ascii_alphanumeric() {
            let encrypted_char = String::from(((c - 65 + key)%26 + 65) as char);
            cipher_text = format!("{cipher_text}{encrypted_char}");
        }
        else {
            let spe = c as char;
            cipher_text = format!("{cipher_text}{spe}");
        }
    }
    println!("{}", cipher_text);
}

fn cesar_decrypt(pt_text: &str, key: i8) {
    /// Function which handles the Cesar decryption according to a certain key
    let text = pt_text.to_uppercase();
    let mut cipher_text = String::new();
    for c in text.bytes() {
        if c.is_ascii_alphanumeric() {
            let mut result = (c as i8 - 65 - key)%26;
            if result < 0 {
                result += 26;
            }
            let encrypted_char = String::from(((result + 65) as u8) as char);
            cipher_text = format!("{cipher_text}{encrypted_char}");
        }
        else {
            let spe = c as char;
            cipher_text = format!("{cipher_text}{spe}");
        }
    }
    println!("{}", cipher_text);
}
