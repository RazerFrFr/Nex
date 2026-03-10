use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub redirect: Option<String>,

    #[serde(default)]
    pub backend: Option<String>,

    #[serde(default)]
    pub gameserver: Option<String>,

    #[serde(default)]
    pub clientdll: Option<String>,
}
