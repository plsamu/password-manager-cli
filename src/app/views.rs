use crate::models::Keystore;
use crate::REMOVE_APP;
use crate::{models::App, ADD_APP, SAVE_AND_EXIT};
use crossterm::style::Color;
use std::sync::{Arc, RwLock};
use terminal_menu::TerminalMenuItem;
use terminal_menu::TerminalMenuStruct;
use terminal_menu::{button, label, menu};

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

fn render_apps_menu(apps: &Vec<App>, color: Color) -> Vec<TerminalMenuItem> {
    let mut apps_buttons = vec![];
    apps_buttons.append(&mut vec![label(" --- Apps --- ").colorize(color)]);
    apps.iter()
        .for_each(|app| apps_buttons.push(button(app.name.to_string())));
    apps_buttons.push(label(" --- ").colorize(color));
    apps_buttons
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
