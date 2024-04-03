pub mod utils;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::time::Duration;

use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit};
use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::typenum::bit::{B0, B1};
use sha2::digest::typenum::{UInt, UTerm};
use sha2::Sha512;
use terminal_menu::{button, label, menu};

use crate::utils::clear_screen;

const SALT: &[u8] = "bel-salt-bro".as_bytes(); // must be 12 bit long
const FILENAME: &str = "keystore";
const ROUNDS: u32 = 30_000;

const ADD_APP: &str = "Add App";
const REMOVE_APP: &str = "Remove App";
const GET_PASSWORD: &str = "Get Password";
const ADD_PASSWORD: &str = "Add Password";
const REMOVE_PASSWORD: &str = "Remove Password";
const EXIT: &str = "Exit";

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
        let keystore: Keystore = Keystore { apps: vec![] };
        let text = serde_json::to_string(&keystore).unwrap();
        let ciphertext = crypt(&password1, text);
        let mut data_file = File::create(path).expect("creation failed");
        match data_file.write(&ciphertext.clone().unwrap()) {
            Ok(_) => {}
            Err(err) => {
                panic!("Couldn't write in file, {}", err)
            }
        }
        run_app(password1, keystore)
    } else {
        print!("Type your master password: ");
        std::io::stdout().flush().unwrap();
        let password = rpassword::read_password().unwrap();
        let ciphertext: Vec<u8> = std::fs::read(FILENAME).expect("Unable to read file");
        let content = decrypt(&password, ciphertext);
        match serde_json::from_str::<Keystore>(&content) {
            Ok(keystore) => run_app(password, keystore),
            Err(err) => panic!(
                "Could't read keystore, maybe it's corrupted. I'm sorry :( \n{}",
                err
            ),
        }
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

/**
 * > Add app
 * > Remove app
 * 	> app1 (with confirmations)
 * 	> ...
 * > app1
 *  > add_pwd
 *  > remove_pwd
 * 	> profile1
 * 	> profile2
 * 	> ...
 * > app2
 * > ...
 */
fn run_app(password: String, keystore: Keystore) {
    loop {
        let apps_name: Vec<&str> = keystore.apps.iter().map(|app| app.name.as_ref()).collect();
        let mut menu_choices = vec![
            label("Password Manager CLI").colorize(Color::Blue),
            button(ADD_APP),
            button(REMOVE_APP),
        ];
        for app_name in apps_name {
            menu_choices.push(button(app_name))
        }
        menu_choices.push(button(EXIT));
        let menu = menu(menu_choices);
        clear_screen();
        terminal_menu::run(&menu);
        match terminal_menu::mut_menu(&menu.clone()).selected_item_name() {
            ADD_APP => {
                clear_screen();
                println!("{}", ADD_APP);
            }
            REMOVE_APP => {
                clear_screen();
                println!("{}", REMOVE_APP)
            }
            EXIT => {
                println!("{}", EXIT);
                exit(0)
            }
            _ => {}
        }
    }
}
