use crate::{models::Keystore, run_app, utils, views, FILENAME};

pub fn open_keystore() {
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
