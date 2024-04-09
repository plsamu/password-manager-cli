use super::views::load_remove_profile;
use crate::{
    models::{App, Profile},
    utils::{read_user_input, run_and_get_mut_menu, show_msg_to_user},
    ADD_PROFILE, REMOVE_PROFILE,
};
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
                show_msg_to_user("Profile Already Exists.")
            } else {
                // pwd length: https://bitwarden.com/blog/how-long-should-my-password-be/
                let pg = PasswordGenerator {
                    length: 16,
                    numbers: true,
                    lowercase_letters: true,
                    uppercase_letters: true,
                    symbols: true,
                    spaces: true,
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
            // TODO copy to clipboard
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
    app.profiles
        .retain(|profile| profile.profile_name.ne(selected))
}
