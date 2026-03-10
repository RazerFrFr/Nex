use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub email: String,
    pub password: String,
    pub gs_email: String,
    pub gs_password: String,
    pub path: String,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            name: String::new(),
            email: String::new(),
            password: String::new(),
            gs_email: String::new(),
            gs_password: String::new(),
            path: String::new(),
        }
    }
}
