use std::sync::{Arc, RwLock};

use crossterm::style::Color;
use terminal_menu::{button, label, TerminalMenuItem, TerminalMenuStruct};

use crate::models::{App, Profile};
use crate::{ADD_PROFILE, EXIT, ORANGE, REMOVE_PROFILE};

pub fn run_menu(app: &App) {
    let menu_profiles = load_menu(&app);
    terminal_menu::run(&menu_profiles);
    let mut_profile_menu = terminal_menu::mut_menu(&menu_profiles);
    let profile_selected = mut_profile_menu.selected_item_name();
    handle_profile_selected(profile_selected);
}

fn load_menu(app: &App) -> Arc<RwLock<TerminalMenuStruct>> {
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
    terminal_menu::menu(apps_buttons)
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

pub fn handle_profile_selected(profile_selected: &str) {
    match profile_selected {
        ADD_PROFILE => {
            // create new profile
        }
        REMOVE_PROFILE => {}
        EXIT => {}
        _ => {}
    }
}
