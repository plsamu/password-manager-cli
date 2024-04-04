pub mod constants;
pub mod models;
pub mod utils;
pub mod views;

use crate::models::App;
use crate::utils::crypt;
use crate::utils::decrypt;
use crate::views::load_remove_apps_menu;
use crate::views::read_user_input;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::time::Duration;
use std::vec;

use constants::*;
use models::Keystore;
use utils::exit_without_save;
use views::create_new_password;
use views::load_menu_apps;

use crate::utils::clear_screen;

fn main() {
    ctrlc::set_handler(move || {
        exit_without_save(0);
    })
    .expect("Error setting Ctrl-C handler");
    let path = Path::new(FILENAME);
    if !path.exists() {
        let pwd = create_new_password();
        let keystore: Keystore = Keystore { apps: vec![] };
        let text = serde_json::to_string(&keystore).unwrap();
        let ciphertext = crypt(&pwd, text);
        let mut data_file = File::create(path).expect("creation failed");
        match data_file.write(&ciphertext.clone().unwrap()) {
            Ok(_) => {}
            Err(err) => {
                panic!("Couldn't write in file, {}", err)
            }
        }
        run_app(pwd, keystore)
    } else {
        let pwd = views::read_password();
        let ciphertext: Vec<u8> = std::fs::read(FILENAME).expect("Unable to read file");
        let content = decrypt(&pwd, ciphertext);
        match serde_json::from_str::<Keystore>(&content) {
            Ok(keystore) => run_app(pwd, keystore),
            Err(err) => panic!(
                "Could't read keystore, maybe it's corrupted. I'm sorry :( \n{}",
                err
            ),
        }
    }
}

fn run_app(password: String, mut keystore: Keystore) {
    println!("{}", "Running App");
    loop {
        clear_screen();
        let menu = load_menu_apps(&keystore);
        terminal_menu::run(&menu);
        let mut_menu = terminal_menu::mut_menu(&menu);
        if mut_menu.canceled() {
            exit_without_save(0);
        }
        match mut_menu.selected_item_name() {
            ADD_APP => {
                let user_input_app_name = read_user_input("Insert App Name: ");
                let mut already_exists = false;
                keystore.apps.iter().for_each(|app| {
                    if user_input_app_name == app.name {
                        println!("App Already Exists");
                        std::thread::sleep(Duration::from_millis(800));
                        already_exists = true;
                    }
                });
                if already_exists == false {
                    keystore.apps.push(App {
                        name: user_input_app_name,
                        profiles: vec![],
                    });
                }
            }
            REMOVE_APP => {
                let menu = load_remove_apps_menu(&keystore.apps);
                terminal_menu::run(&menu);
            }
            SAVE_AND_EXIT => {
                utils::save(&password, &keystore);
                exit(0);
            }
            _ => keystore.apps.iter().for_each(|app| {
                if app.name == mut_menu.selected_item_name() {
                    let menu_profiles = views::load_menu_profiles(&app);
                    terminal_menu::run(&menu_profiles);
                }
            }),
        }
    }
}
