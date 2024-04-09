use crate::app::views;
use crate::profile;
use crate::{models::App, ADD_APP, REMOVE_APP, SAVE_AND_EXIT};
use crate::{
    models::Keystore,
    utils::{self},
};

pub fn handle_app_selection(selection: &str, keystore: &mut Keystore, password: &String) {
    match selection {
        ADD_APP => {
            handle_add_app(keystore);
        }
        REMOVE_APP => {
            views::load_remove_apps_menu(&keystore.apps);
        }
        SAVE_AND_EXIT => {
            utils::save(password, &keystore);
            std::process::exit(0);
        }
        _ => match keystore.apps.iter_mut().find(|app| app.name.eq(selection)) {
            Some(mut_app) => profile::load_menu_profiles(mut_app),
            None => {}
        },
    }
}

pub fn handle_add_app(keystore: &mut Keystore) {
    let user_input_app_name = utils::read_user_input("Insert App Name: ");
    if keystore
        .apps
        .iter()
        .any(|app| app.name.eq(&user_input_app_name))
    {
        utils::show_msg_to_user("App Already Exists");
    } else {
        keystore.apps.push(App {
            name: user_input_app_name,
            profiles: vec![],
        });
    }
}
