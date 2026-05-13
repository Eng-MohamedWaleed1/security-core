mod engine;
use engine::SecureBuffer;
use std::io::{self, Write};

fn main() {
    let key: u8 = 0x5E;

    println!("════════════════════════════════════════");
    println!("  WALEED-CORE — Secure Message Receiver");
    println!("════════════════════════════════════════");
    print!("Paste the encrypted hex here: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let cipher_bytes: Vec<u8> = input
        .trim()
        .split_whitespace()
        .filter_map(|h| u8::from_str_radix(h, 16).ok())
        .collect();

    if cipher_bytes.is_empty() {
        eprintln!("[ERROR] No valid hex input detected.");
    } else {
        let mut buf = SecureBuffer::new(key);
        buf.load_ciphertext(cipher_bytes).unwrap();

        match buf.decrypt() {
            Ok(_) => {
                let msg = std::str::from_utf8(buf.view().data).unwrap_or("[binary data]");
                println!("\n════════════════════════════════════════");
                println!("  DECRYPTED MESSAGE:");
                println!("════════════════════════════════════════");
                println!("  {}", msg);
                println!("════════════════════════════════════════");
            }
            Err(e) => eprintln!("[FAILED] {}", e),
        }
    }

    println!("\nPress ENTER to exit...");
    let mut pause = String::new();
    io::stdin().read_line(&mut pause).unwrap();
}