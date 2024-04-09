use crossterm::style::Color;

pub const ORANGE: Color = Color::Rgb {
    r: 255,
    g: 165,
    b: 0,
};
pub const SALT: &[u8] = "bel-salt-bro".as_bytes(); // must be 12 bit long
pub const FILENAME: &str = "keystore";
pub const ROUNDS: u32 = 30_000;
pub const ADD_APP: &str = "Add App";
pub const OK: &str = "Ok";
pub const REMOVE_APP: &str = "Remove App";
pub const GET_PASSWORD: &str = "Get Password";
pub const ADD_PASSWORD: &str = "Add Password";
pub const REMOVE_PASSWORD: &str = "Remove Password";
pub const SAVE_AND_EXIT: &str = "Save & Exit";
pub const EXIT: &str = "Exit";
pub const ADD_PROFILE: &str = "Add Profile";
pub const REMOVE_PROFILE: &str = "Remove Profile";
pub const OPEN_KEYSTORE_ERROR: &str = "Could't read keystore, maybe it's corrupted. I'm sorry :(";
pub const USE_KEY_TO_EXIT_WITHOUT_SAVE: &str = "(use 'q' or esc to exit without save)";
