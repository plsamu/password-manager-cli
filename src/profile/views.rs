use crate::models::Profile;
use crate::profile::App;
use crate::{ADD_PROFILE, EXIT, ORANGE, REMOVE_PROFILE, USE_KEY_TO_EXIT};
use crossterm::style::Color;
use std::sync::{Arc, RwLock};
use terminal_menu::{button, label};
use terminal_menu::{TerminalMenuItem, TerminalMenuStruct};

fn load_profiles_with_title(app: &App, title: &str, color: Color) -> Vec<TerminalMenuItem> {
    let mut menu_items = render_profiles_menu(&app.profiles, color);
    menu_items.insert(
        0,
        label(format!("{} {}", title, USE_KEY_TO_EXIT)).colorize(color),
    );
    menu_items
}

pub fn load_remove_profile(app: &App) -> Arc<RwLock<TerminalMenuStruct>> {
    let items = load_profiles_with_title(app, "Select Profiles To Remove", ORANGE);
    terminal_menu::menu(items)
}

pub fn load_menu(app: &App) -> Arc<RwLock<TerminalMenuStruct>> {
    let mut items = load_profiles_with_title(app, "App Profiles", Color::Green);
    items.append(&mut vec![
        button(ADD_PROFILE),
        button(REMOVE_PROFILE),
        button(EXIT),
    ]);
    terminal_menu::menu(items)
}

pub fn render_profiles_menu(profiles: &Vec<Profile>, color: Color) -> Vec<TerminalMenuItem> {
    let mut profiles_buttons = vec![];
    profiles_buttons.append(&mut vec![label(" --- Profiles --- ").colorize(color)]);
    profiles
        .iter()
        .for_each(|profile| profiles_buttons.push(button(profile.profile_name.to_string())));
    profiles_buttons.push(label(" --- ").colorize(color));
    profiles_buttons
}
