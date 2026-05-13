mod engine;
use engine::SecureBuffer;
use std::io::{self, Write};

fn main() {
    let key: u8 = 0x5E;

    print!("Enter your secret message: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let message = input.trim().as_bytes().to_vec();

    let mut buf = SecureBuffer::new(key);
    buf.load(message).unwrap();
    buf.encrypt().unwrap();

    let hex: String = buf
        .view()
        .data
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ");

    println!("\n════════════════════════════════════════");
    println!("  ENCRYPTED — Send this to the receiver:");
    println!("════════════════════════════════════════");
    println!("{}", hex);
    println!("════════════════════════════════════════");
    println!("Key used: 0x5E (share privately with receiver)");
    println!("\nPress ENTER to exit...");
    let mut pause = String::new();
    io::stdin().read_line(&mut pause).unwrap();
}