pub mod app;
pub mod constants;
pub mod models;
pub mod profile;
pub mod utils;
pub mod views;

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
        app::open_with_keystore();
    } else {
        app::open_without_keystore();
    }
}
