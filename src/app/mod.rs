pub mod controllers;
pub mod views;
use self::views::load_menu_apps;
use crate::{
    models::Keystore,
    utils::{self, exit_without_save, run_and_get_mut_menu},
};

pub fn run_app(password: String, mut keystore: Keystore) {
    loop {
        utils::clear_screen();
        let menu = load_menu_apps(&keystore);
        let mut_menu = run_and_get_mut_menu(&menu);
        if mut_menu.canceled() {
            exit_without_save(0);
        }
        controllers::handle_app_selection(mut_menu.selected_item_name(), &mut keystore, &password);
    }
}
