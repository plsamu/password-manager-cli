use super::views::load_remove_profile;
use crate::{
    models::{App, Profile},
    utils::{
        read_user_input, run_and_get_mut_menu, show_blocking_msg_to_user, show_msg_to_user,
        yes_no_blocking_user_decision,
    },
    ADD_PROFILE, REMOVE_PROFILE, YES,
};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use passwords::PasswordGenerator;

pub fn handle_profile_selected(app: &mut App, profile_selected: &str) {
    match profile_selected {
        ADD_PROFILE => {
            let profile_name = read_user_input("Insert Profile Name: ", true);
            if app
                .profiles
                .iter()
                .any(|profile| profile.profile_name.eq(&profile_name))
            {
                show_blocking_msg_to_user("Profile Already Exists.")
            } else {
                // pwd length: https://bitwarden.com/blog/how-long-should-my-password-be/
                let pg = PasswordGenerator {
                    length: 16,
                    numbers: true,
                    lowercase_letters: true,
                    uppercase_letters: true,
                    symbols: true,
                    spaces: false,
                    exclude_similar_characters: false,
                    strict: true,
                };
                app.profiles.push(Profile {
                    profile_name,
                    pwd: pg.generate_one().unwrap(),
                })
            }
        }
        REMOVE_PROFILE => handle_remove_profile(app),
        _ => {
            if let Some(profile) = app
                .profiles
                .iter()
                .find(|profile| profile.profile_name.eq(&profile_selected))
            {
                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(profile.pwd.clone()).unwrap();
                show_msg_to_user("Password copied to clipboard", 800, true);
            }
        }
    }
}

fn handle_remove_profile(app: &mut App) {
    let menu = load_remove_profile(app);
    let mut_menu = run_and_get_mut_menu(&menu);
    if mut_menu.canceled() {
        return;
    }
    let selected = mut_menu.selected_item_name();
    if YES == yes_no_blocking_user_decision(&format!("Sure you want to remove {}?", selected)) {
        app.profiles
            .retain(|profile| profile.profile_name.ne(selected))
    }
}
