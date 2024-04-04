pub mod constants;
pub mod keystore_manager;
pub mod models;
pub mod profile_manager;
pub mod utils;
pub mod views;

use crate::views::load_remove_apps_menu;
use std::path::Path;
use std::process::exit;
use std::vec;

use constants::*;
use keystore_manager::open_keystore;
use models::Keystore;
use utils::clear_screen;
use utils::exit_without_save;
use views::create_new_password;
use views::load_add_app;
use views::load_mut_main_menu;
use views::load_profiles_menu;

fn main() {
    ctrlc::set_handler(move || {
        exit_without_save(0);
    })
    .expect("Error setting Ctrl-C handler");
    let path = Path::new(FILENAME);
    if path.exists() {
        open_keystore();
    } else {
        println!("Keystore not found. Creating master password:");
        let pwd = create_new_password(
            "Write your new master password: ",
            "Confirm master password: ",
        );
        let keystore: Keystore = Keystore { apps: vec![] };
        run_app(pwd, keystore)
    }
}

fn run_app(password: String, mut keystore: Keystore) {
    loop {
        clear_screen();
        let selected_item = load_mut_main_menu(&keystore);
        handle_app_selection(&selected_item, &mut keystore, &password);
    }
}

fn handle_app_selection(selection: &str, keystore: &mut Keystore, password: &String) {
    match selection {
        ADD_APP => {
            load_add_app(keystore);
        }
        REMOVE_APP => {
            load_remove_apps_menu(&keystore.apps);
        }
        SAVE_AND_EXIT => {
            utils::save(password, &keystore);
            exit(0);
        }
        _ => keystore.apps.iter().for_each(|app| {
            if app.name == selection {
                load_profiles_menu(app);
            }
        }),
    }
}
