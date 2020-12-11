extern crate serde_derive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "Uplay")]
    pub uplay: Uplay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uplay {
    #[serde(rename = "Saves")]
    pub saves: String,

    #[serde(rename = "CdKeys")]
    pub cd_keys: Vec<String>,

    #[serde(rename = "Language")]
    pub language: String,

    #[serde(rename = "OfflineMode")]
    pub offline_mode: bool,

    #[serde(rename = "InstallHooks")]
    pub install_hooks: bool,

    #[serde(rename = "Log")]
    pub log: Log,

    #[serde(rename = "Profile")]
    pub profile: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    #[serde(rename = "Write")]
    pub write: bool,

    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "AccountId")]
    pub account_id: String,

    #[serde(rename = "Email")]
    pub email: String,

    #[serde(rename = "Username")]
    pub username: String,

    #[serde(rename = "Password")]
    pub password: String,
}
