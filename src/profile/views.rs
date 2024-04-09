use crate::models::Profile;
use crate::profile::App;
use crate::{ADD_PROFILE, EXIT, ORANGE, REMOVE_PROFILE, USE_KEY_TO_EXIT_WITHOUT_SAVE};
use crossterm::style::Color;
use std::sync::{Arc, RwLock};
use terminal_menu::{button, label};
use terminal_menu::{TerminalMenuItem, TerminalMenuStruct};

pub fn load_menu(app: &App) -> Arc<RwLock<TerminalMenuStruct>> {
    let mut apps_buttons = render_profiles_menu(&app.profiles, ORANGE);
    apps_buttons.insert(
        0,
        label(format!("App Profiles {}", USE_KEY_TO_EXIT_WITHOUT_SAVE)).colorize(ORANGE),
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
