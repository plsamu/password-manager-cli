use crate::{ADD_PROFILE, EXIT, REMOVE_PROFILE};

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
