pub mod controllers;
pub mod views;
use self::controllers::handle_profile_selected;
use self::views::load_menu;
use crate::models::App;
use crate::utils::run_and_get_mut_menu;

pub fn load_menu_profiles(app: &App) {
    let menu_profiles = load_menu(&app);
    let mut_menu = run_and_get_mut_menu(&menu_profiles);
    handle_profile_selected(mut_menu.selected_item_name());
}
