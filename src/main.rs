use std::fs::File;
use std::io::Write;
use std::path::Path;

use chacha20poly1305::aead::{Aead, Buffer};
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit};
use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::typenum::bit::{B0, B1};
use sha2::digest::typenum::{UInt, UTerm};
use sha2::Sha512;
use terminal_menu::{button, label, menu};

const SALT: &[u8] = "bel-salt-bro".as_bytes(); // must be 12 bit long
const FILENAME: &str = "keystore";
const ROUNDS: u32 = 30_000;

fn get_key(
    pwd: &String,
) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>> {
    let mut buf = [0u8; 32];
    pbkdf2::pbkdf2_hmac::<Sha512>(pwd.as_bytes(), SALT, ROUNDS, &mut buf);
    Key::from(buf)
}

fn crypt(pwd: &String, text: String) -> Result<Vec<u8>, chacha20poly1305::Error> {
    let cipher = ChaCha20Poly1305::new(&get_key(pwd));
    cipher.encrypt(SALT.into(), text.as_bytes().as_ref())
}

fn decrypt(pwd: &String, ciphertext: Vec<u8>) -> String {
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
    res_utf8.unwrap().to_string()
}

fn main() {
    let path = Path::new(FILENAME);
    if !path.exists() {
        println!("Keystore not found. Creating master password:");
        std::io::stdout().flush().unwrap();
        let password1 = rpassword::prompt_password("Write your new master password: ").unwrap();
        std::io::stdout().flush().unwrap();
        let password2 = rpassword::prompt_password("Confirm: ").unwrap();
        if password1 != password2 {
            println!("password should be the same");
            std::process::exit(1);
        }
        let ciphertext = crypt(&password1, "".to_string());
        let _data_file = File::create(path).expect("creation failed");
        run_app(password1, ciphertext.unwrap())
    } else {
        print!("Type your master password: ");
        std::io::stdout().flush().unwrap();
        let password = rpassword::read_password().unwrap();
        let ciphertext: Vec<u8> = std::fs::read(FILENAME).expect("Unable to read file");
        run_app(password, ciphertext)
    }
}

#[derive(Serialize, Deserialize)]
struct Pwd {
    profile_name: String,
    pwd: String,
}

#[derive(Serialize, Deserialize)]
struct App {
    name: String,
    pwds: Vec<Pwd>,
}

#[derive(Serialize, Deserialize)]
struct Keystore {
    apps: Vec<App>,
}

fn run_app(password: String, ciphertext: Vec<u8>) {
    let content = decrypt(&password, ciphertext);
    println!("{}", content);
    if !content.is_empty() {
        let keystore: Keystore = serde_json::from_str(&content).unwrap();
    }
    let menu = menu(vec![
        label("COLOR!").colorize(Color::Red),
        label("Green").colorize(Color::Green),
        label("Blue").colorize(Color::Blue),
        button("Cyan"),
    ]);
    terminal_menu::run(&menu);
}
