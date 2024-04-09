use passwords::PasswordGenerator;

use crate::{
    models::{App, Profile},
    utils::{clear_screen, read_user_input, show_msg_to_user},
    ADD_PROFILE, REMOVE_PROFILE,
};

pub fn handle_profile_selected(app: &mut App, profile_selected: &str) {
    match profile_selected {
        ADD_PROFILE => {
            clear_screen();
            let profile_name = read_user_input("Insert Profile Name: ");
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
        REMOVE_PROFILE => {}
        _ => {}
    }
}
