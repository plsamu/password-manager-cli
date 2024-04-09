use crate::app::views;
use crate::{models::App, profile, ADD_APP, REMOVE_APP, SAVE_AND_EXIT};
use crate::{
    models::Keystore,
    utils::{self},
};
use std::time::Duration;

pub fn handle_app_selection(selection: &str, mut keystore: &mut Keystore, password: &String) {
    match selection {
        ADD_APP => {
            handle_add_app(&mut keystore);
        }
        REMOVE_APP => {
            views::load_remove_apps_menu(&keystore.apps);
        }
        SAVE_AND_EXIT => {
            utils::save(password, &keystore);
            std::process::exit(0);
        }
        _ => keystore.apps.iter().for_each(|app| {
            if app.name == selection {
                profile::load_menu_profiles(app);
            }
        }),
    }
}

pub fn handle_add_app(keystore: &mut Keystore) {
    let user_input_app_name = utils::read_user_input("Insert App Name: ");
    let mut already_exists = false;
    keystore.apps.iter().for_each(|app| {
        if user_input_app_name == app.name {
            println!("App Already Exists");
            std::thread::sleep(Duration::from_millis(800));
            already_exists = true;
        }
    });
    if already_exists == false {
        keystore.apps.push(App {
            name: user_input_app_name,
            profiles: vec![],
        });
    }
}
