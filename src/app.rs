use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use crossterm::style::Color;
use terminal_menu::{button, label, menu, TerminalMenuItem, TerminalMenuStruct};

use crate::{
    models::{App, Keystore},
    profile, utils, ADD_APP, FILENAME, REMOVE_APP, SAVE_AND_EXIT,
};

fn run_app(password: String, mut keystore: Keystore) {
    loop {
        utils::clear_screen();
        run_main_menu(&mut keystore, &password);
    }
}

pub fn run_main_menu(mut keystore: &mut Keystore, password: &String) {
    let menu = load_menu_apps(keystore);
    terminal_menu::run(&menu);
    let mut_menu = terminal_menu::mut_menu(&menu);
    if mut_menu.canceled() {
        utils::exit_without_save(0);
    }
    let selected = mut_menu.selected_item_name().to_string();
    handle_app_selection(&selected, &mut keystore, &password);
}

fn handle_app_selection(selection: &str, mut keystore: &mut Keystore, password: &String) {
    match selection {
        ADD_APP => {
            handle_add_app(&mut keystore);
        }
        REMOVE_APP => {
            load_remove_apps_menu(&keystore.apps);
        }
        SAVE_AND_EXIT => {
            utils::save(password, &keystore);
            std::process::exit(0);
        }
        _ => keystore.apps.iter().for_each(|app| {
            if app.name == selection {
                profile::run_menu(app);
            }
        }),
    }
}

fn handle_add_app(keystore: &mut Keystore) {
    let user_input_app_name = utils::read_user_input("Insert App Name: ");
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

pub fn load_remove_apps_menu(apps: &Vec<App>) {
    let mut apps_buttons = render_apps_menu(apps, Color::Red);
    apps_buttons.insert(
        0,
        label("Remove App (use 'q' or esc to exit)").colorize(Color::Red),
    );
    let menu = menu(apps_buttons);
    terminal_menu::run(&menu);
}

pub fn open_with_keystore() {
    let pwd = utils::read_password();
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
    let pwd = utils::create_new_password(
        "Write your new master password: ",
        "Confirm master password: ",
    );
    let keystore: Keystore = Keystore { apps: vec![] };
    run_app(pwd, keystore)
}

fn render_apps_menu(apps: &Vec<App>, color: Color) -> Vec<TerminalMenuItem> {
    let mut apps_buttons = vec![];
    apps_buttons.append(&mut vec![label(" --- Apps --- ").colorize(color)]);
    apps.iter()
        .for_each(|app| apps_buttons.push(button(app.name.to_string())));
    apps_buttons.push(label(" --- ").colorize(color));
    apps_buttons
}

pub fn load_menu_apps(keystore: &Keystore) -> Arc<RwLock<TerminalMenuStruct>> {
    let mut apps_buttons = render_apps_menu(&keystore.apps, Color::Magenta);
    apps_buttons.insert(
        0,
        label("Password Manager CLI (use 'q' or esc to exit)").colorize(Color::Magenta),
    );
    apps_buttons.append(&mut vec![
        button(ADD_APP),
        button(REMOVE_APP),
        button(SAVE_AND_EXIT),
    ]);
    menu(apps_buttons)
}
