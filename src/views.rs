use std::io::{self, Write};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crossterm::style::Color;
use terminal_menu::{button, label, menu};
use terminal_menu::{TerminalMenuItem, TerminalMenuStruct};

use crate::models::{App, Profile};
use crate::profile_manager::handle_profile_selected;
use crate::utils::{clear_screen, exit_without_save};
use crate::{
    Keystore, ADD_APP, ADD_PROFILE, EXIT, ORANGE, REMOVE_APP, REMOVE_PROFILE, SAVE_AND_EXIT,
};

pub fn load_profiles_menu(app: &App) {
    let menu_profiles = load_menu_profiles(&app);
    terminal_menu::run(&menu_profiles);
    let mut_profile_menu = terminal_menu::mut_menu(&menu_profiles);
    let profile_selected = mut_profile_menu.selected_item_name();
    handle_profile_selected(profile_selected);
}

pub fn load_add_app(keystore: &mut Keystore) {
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

pub fn load_mut_main_menu(keystore: &Keystore) -> String {
    let menu = load_menu_apps(keystore);
    terminal_menu::run(&menu);
    let mut_menu = terminal_menu::mut_menu(&menu);
    if mut_menu.canceled() {
        exit_without_save(0);
    }
    mut_menu.selected_item_name().to_string()
}

fn render_apps_menu(apps: &Vec<App>, color: Color) -> Vec<TerminalMenuItem> {
    let mut apps_buttons = vec![];
    apps_buttons.append(&mut vec![label(" --- Apps --- ").colorize(color)]);
    apps.iter()
        .for_each(|app| apps_buttons.push(button(app.name.to_string())));
    apps_buttons.push(label(" --- ").colorize(color));
    apps_buttons
}

fn render_profiles_menu(profiles: &Vec<Profile>, color: Color) -> Vec<TerminalMenuItem> {
    let mut profiles_buttons = vec![];
    profiles_buttons.append(&mut vec![label(" --- Profiles --- ").colorize(color)]);
    profiles
        .iter()
        .for_each(|profile| profiles_buttons.push(button(profile.profile_name.to_string())));
    profiles_buttons.push(label(" --- ").colorize(color));
    profiles_buttons
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

pub fn create_new_password(message: &str, confirm_message: &str) -> String {
    loop {
        clear_screen();
        let password1 = rpassword::prompt_password(message).unwrap();
        std::io::stdout().flush().unwrap();
        let password2 = rpassword::prompt_password(confirm_message).unwrap();
        if password1 == password2 {
            return password1;
        } else {
            println!("Passwords must match.");
            std::thread::sleep(Duration::from_millis(800));
        }
    }
}

pub fn read_user_input(msg: &str) -> std::string::String {
    std::io::stdout().flush().unwrap();
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    std::io::stdout().flush().unwrap();
    input
}

pub fn read_password() -> std::string::String {
    clear_screen();
    print!("Type your master password: ");
    std::io::stdout().flush().unwrap();
    rpassword::read_password().unwrap()
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

pub fn load_menu_profiles(app: &App) -> Arc<RwLock<TerminalMenuStruct>> {
    let mut apps_buttons = render_profiles_menu(&app.profiles, ORANGE);
    apps_buttons.insert(
        0,
        label("App Profiles (use 'q' or esc to exit)").colorize(ORANGE),
    );
    apps_buttons.append(&mut vec![
        button(ADD_PROFILE),
        button(REMOVE_PROFILE),
        button(EXIT),
    ]);
    menu(apps_buttons)
}
