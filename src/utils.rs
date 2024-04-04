use std::io::Write;

use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit};
use constants::*;
use sha2::digest::generic_array::GenericArray;
use sha2::digest::typenum::bit::{B0, B1};
use sha2::digest::typenum::{UInt, UTerm};
use sha2::Sha512;

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::constants;

pub fn get_key(
    pwd: &String,
) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>> {
    let mut buf = [0u8; 32];
    pbkdf2::pbkdf2_hmac::<Sha512>(pwd.as_bytes(), SALT, ROUNDS, &mut buf);
    Key::from(buf)
}

pub fn crypt(pwd: &String, text: String) -> Result<Vec<u8>, chacha20poly1305::Error> {
    println!("{}", "encrypting keystore");
    let cipher = ChaCha20Poly1305::new(&get_key(pwd));
    cipher.encrypt(SALT.into(), text.as_bytes().as_ref())
}

pub fn decrypt(pwd: &String, ciphertext: Vec<u8>) -> String {
    println!("{}", "decrypting keystore");
    let cipher = ChaCha20Poly1305::new(&get_key(pwd));
    if ciphertext.is_empty() {
        return "".to_string();
    }
    let text_binary_res: Result<Vec<u8>, chacha20poly1305::Error> =
        cipher.decrypt(SALT.into(), &*ciphertext);
    if let Err(_err) = text_binary_res {
        panic!("Wrong password")
    }
    let binding = text_binary_res.unwrap();
    let res_utf8 = std::str::from_utf8(&binding);
    if let Err(err) = res_utf8 {
        panic!("Invalid UTF-8 sequence: {}", err)
    }
    println!("{}", "keystore decrypted");
    res_utf8.unwrap().to_string()
}

pub fn clear_screen() {
    let mut out = std::io::stdout();
    // out.queue(Hide).unwrap(); // Optionally hide the cursor
    out.queue(Clear(ClearType::All)).unwrap(); // Clear the screen
    out.queue(MoveTo(0, 0)).unwrap(); // Move the cursor to the top-left corner
    out.flush().unwrap(); // Flush the output to the terminal
}
