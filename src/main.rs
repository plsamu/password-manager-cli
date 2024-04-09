pub mod app;
pub mod constants;
pub mod models;
pub mod profile;
pub mod utils;

use std::path::Path;

use constants::*;
use models::Keystore;
use utils::exit_without_save;

fn main() {
    ctrlc::set_handler(move || {
        exit_without_save(0);
    })
    .expect("Error setting Ctrl-C handler");
    let path = Path::new(FILENAME);
    if path.exists() {
        open_with_keystore();
    } else {
        open_without_keystore();
    }
}

fn open_with_keystore() {
    let pwd = utils::read_password();
    let ciphertext: Vec<u8> = std::fs::read(FILENAME).expect("Unable to read file");
    let content = utils::decrypt(&pwd, ciphertext);
    let keystore: Result<Keystore, serde_json::Error> = serde_json::from_str::<Keystore>(&content);
    if let Err(err) = keystore {
        panic!("{}\n{}", OPEN_KEYSTORE_ERROR, err)
    }
    app::run_app(pwd, keystore.unwrap());
}

fn open_without_keystore() {
    println!("Keystore not found. Creating master password:");
    let pwd = utils::create_new_password(
        "Write your new master password: ",
        "Confirm master password: ",
    );
    let keystore: Keystore = Keystore { apps: vec![] };
    app::run_app(pwd, keystore)
}
