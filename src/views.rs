use std::io::Write;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crossterm::style::Color;
use terminal_menu::{button, label, menu};
use terminal_menu::{string, TerminalMenuStruct};

use crate::utils::clear_screen;
use crate::{Keystore, ADD_APP, EXIT, REMOVE_APP};

pub fn create_new_password() -> std::string::String {
    clear_screen();
    println!("Keystore not found. Creating master password:");
    std::io::stdout().flush().unwrap();
    let password1 = rpassword::prompt_password("Write your new master password: ").unwrap();
    std::io::stdout().flush().unwrap();
    let password2 = rpassword::prompt_password("Confirm: ").unwrap();
    if password1 != password2 {
        println!("password should be the same");
        std::process::exit(1);
    }
    password1
}

pub fn read_password() -> std::string::String {
    clear_screen();
    print!("Type your master password: ");
    std::io::stdout().flush().unwrap();
    rpassword::read_password().unwrap()
}

pub fn load_menu_apps(keystore: &Keystore) -> Arc<RwLock<TerminalMenuStruct>> {
    println!("load_menu_apps");
    std::thread::sleep(Duration::from_millis(1500));
    let mut vec_menu = vec![
        label("Password Manager CLI").colorize(Color::Blue),
        string(ADD_APP, "", /* allow empty string */ false),
        button(REMOVE_APP),
    ];
    let mut apps_buttons = vec![];
    let apps_name: Vec<&str> = keystore
        .apps
        .iter()
        .map(|app| {
            println!("{}", app.name);
            std::thread::sleep(Duration::from_millis(1500));
            app.name.as_ref()
        })
        .collect();
    apps_name
        .iter()
        .for_each(|app_name| apps_buttons.push(button(app_name.to_string())));
    vec_menu.append(&mut apps_buttons);
    vec_menu.push(button(EXIT));
    menu(vec_menu)
}
