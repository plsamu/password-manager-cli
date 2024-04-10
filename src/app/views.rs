use crate::models::Keystore;
use crate::{models::App, ADD_APP, SAVE_AND_EXIT};
use crate::{REMOVE_APP, USE_KEY_TO_EXIT, USE_KEY_TO_EXIT_WITHOUT_SAVE};
use crossterm::style::Color;
use std::sync::{Arc, RwLock};
use terminal_menu::TerminalMenuItem;
use terminal_menu::TerminalMenuStruct;
use terminal_menu::{button, label, menu};

pub fn load_menu_apps(keystore: &Keystore) -> Arc<RwLock<TerminalMenuStruct>> {
    let mut apps_buttons = render_apps_menu(&keystore.apps, Color::Magenta);
    apps_buttons.insert(
        0,
        label(format!(
            "{} {}",
            "Password Manager CLI", USE_KEY_TO_EXIT_WITHOUT_SAVE
        ))
        .colorize(Color::Magenta),
    );
    apps_buttons.append(&mut vec![
        button(ADD_APP),
        button(REMOVE_APP),
        button(SAVE_AND_EXIT),
    ]);
    menu(apps_buttons)
}

pub fn load_remove_app(apps: &[App]) -> Arc<RwLock<TerminalMenuStruct>> {
    let mut apps_buttons = render_apps_menu(apps, Color::Red);
    apps_buttons.insert(
        0,
        label(format!("{} {}", "Remove App", USE_KEY_TO_EXIT)).colorize(Color::Red),
    );
    menu(apps_buttons)
}

fn render_apps_menu(apps: &[App], color: Color) -> Vec<TerminalMenuItem> {
    let mut apps_buttons = vec![];
    apps_buttons.append(&mut vec![label(" --- Apps --- ").colorize(color)]);
    apps.iter()
        .for_each(|app| apps_buttons.push(button(app.name.to_string())));
    apps_buttons.push(label(" --- ").colorize(color));
    apps_buttons
}
