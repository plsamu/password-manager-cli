use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::time::Duration;

use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit};
use constants::*;
use crossterm::style::Color;
use sha2::digest::generic_array::GenericArray;
use sha2::digest::typenum::bit::{B0, B1};
use sha2::digest::typenum::{UInt, UTerm};
use sha2::Sha512;

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use terminal_menu::{button, label, menu, TerminalMenuStruct};

use crate::{constants, Keystore};

pub fn run_and_get_mut_menu(
    menu: &Arc<RwLock<TerminalMenuStruct>>,
) -> RwLockWriteGuard<'_, TerminalMenuStruct> {
    terminal_menu::run(menu);
    let mut_menu = terminal_menu::mut_menu(&menu);
    mut_menu
}

pub fn create_new_password(message: &str, confirm_message: &str) -> String {
    loop {
        clear_screen();
        let password1 = rpassword::prompt_password(message).unwrap();
        std::io::stdout().flush().unwrap();
        let password2 = rpassword::prompt_password(confirm_message).unwrap();
        if password1 == password2 {
            return password1;
        } else {
            println!("Passwords must match.");
            std::thread::sleep(Duration::from_millis(800));
        }
    }
}

pub fn read_password() -> std::string::String {
    clear_screen();
    print!("Type your master password: ");
    std::io::stdout().flush().unwrap();
    rpassword::read_password().unwrap()
}

/**
 * usage:
 * 	let app_name = read_user_input("Insert App Name: ");
 */
pub fn read_user_input(msg: &str) -> std::string::String {
    std::io::stdout().flush().unwrap();
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    std::io::stdout().flush().unwrap();
    input
}

pub fn show_msg_to_user(msg: &str) {
    terminal_menu::run(&menu(vec![label(msg).colorize(Color::Red), button(OK)]));
}

pub fn exit_without_save(exit_code: i32) {
    println!("Exit Without Saving");
    std::thread::sleep(Duration::from_millis(800));
    std::process::exit(exit_code);
}

pub fn save(pwd: &String, keystore: &Keystore) {
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .open(FILENAME)
        .unwrap();
    let text = serde_json::to_string(&keystore).unwrap();
    let ciphertext = crypt(&pwd, text);
    match file.write(&ciphertext.unwrap()) {
        Ok(_) => {}
        Err(err) => {
            panic!("Couldn't write in file, {}", err)
        }
    }
}

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
