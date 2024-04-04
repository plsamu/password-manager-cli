use utils::clear_screen;
use views::create_new_password;
use views::run_main_menu;

use crate::{models::Keystore, utils, views, FILENAME};

fn run_app(password: String, mut keystore: Keystore) {
    loop {
        clear_screen();
        run_main_menu(&mut keystore, &password);
    }
}

pub fn open_with_keystore() {
    let pwd = views::read_password();
    let ciphertext: Vec<u8> = std::fs::read(FILENAME).expect("Unable to read file");
    let content = utils::decrypt(&pwd, ciphertext);
    let keystore: Result<Keystore, serde_json::Error> = serde_json::from_str::<Keystore>(&content);
    if let Err(err) = keystore {
        panic!(
            "Could't read keystore, maybe it's corrupted. I'm sorry :( \n{}",
            err
        )
    }
    run_app(pwd, keystore.unwrap());
}

pub fn open_without_keystore() {
    println!("Keystore not found. Creating master password:");
    let pwd = create_new_password(
        "Write your new master password: ",
        "Confirm master password: ",
    );
    let keystore: Keystore = Keystore { apps: vec![] };
    run_app(pwd, keystore)
}
