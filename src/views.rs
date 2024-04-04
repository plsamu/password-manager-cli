use std::io::{self, Write};
use std::sync::{Arc, RwLock};

use crossterm::style::Color;
use terminal_menu::{button, label, menu};
use terminal_menu::{TerminalMenuItem, TerminalMenuStruct};

use crate::models::{App, Profile};
use crate::utils::clear_screen;
use crate::{
    Keystore, ADD_APP, ADD_PROFILE, EXIT, ORANGE, REMOVE_APP, REMOVE_PROFILE, SAVE_AND_EXIT,
};

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

pub fn load_remove_apps_menu(apps: &Vec<App>) -> Arc<RwLock<TerminalMenuStruct>> {
    let mut apps_buttons = render_apps_menu(apps, Color::Red);
    apps_buttons.insert(
        0,
        label("Remove App (use 'q' or esc to exit)").colorize(Color::Red),
    );
    menu(apps_buttons)
}

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
