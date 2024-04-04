use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub profile_name: String,
    pwd: String,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub profiles: Vec<Profile>,
}

#[derive(Serialize, Deserialize)]
pub struct Keystore {
    pub apps: Vec<App>,
}
