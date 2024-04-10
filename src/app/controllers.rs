use crate::utils::{run_and_get_mut_menu, yes_no_blocking_user_decision};
use crate::{models::App, ADD_APP, REMOVE_APP, SAVE_AND_EXIT};
use crate::{
    models::Keystore,
    utils::{self},
};
use crate::{profile, YES};

use super::views::load_remove_app;

pub fn handle_app_selection(selection: &str, keystore: &mut Keystore, password: &String) {
    match selection {
        ADD_APP => handle_add_app(keystore),
        REMOVE_APP => handle_remove_app(keystore),
        SAVE_AND_EXIT => {
            match utils::save(password, keystore) {
                Ok(_) => {}
                Err(err) => panic!("Error while saving file: {}", err),
            }
            std::process::exit(0);
        }
        _ => {
            if let Some(mut_app) = keystore.apps.iter_mut().find(|app| app.name.eq(selection)) {
                profile::load_menu_profiles(mut_app)
            }
        }
    }
}

pub fn handle_add_app(keystore: &mut Keystore) {
    let user_input_app_name = utils::read_user_input("Insert App Name: ", true);
    if keystore
        .apps
        .iter()
        .any(|app| app.name.eq(&user_input_app_name))
    {
        utils::show_blocking_msg_to_user("App Already Exists");
    } else {
        keystore.apps.push(App {
            name: user_input_app_name,
            profiles: vec![],
        });
    }
}

fn handle_remove_app(keystore: &mut Keystore) {
    let menu = load_remove_app(&keystore.apps);
    let mut_menu = run_and_get_mut_menu(&menu);
    if mut_menu.canceled() {
        return;
    }
    let selected = mut_menu.selected_item_name();
    if yes_no_blocking_user_decision(&format!("Sure you want to remove {}?", selected)) == YES {
        keystore.apps.retain(|app| app.name.ne(selected))
    }
}
