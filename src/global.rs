use parking_lot::RwLock;
use std::{collections::HashMap, env, fs, fs::OpenOptions};

use crate::{consts::CONFIG_NAME, models::config::Config};

lazy_static! {
    pub static ref CONFIG: Config = {
        let path = env::current_dir().unwrap().join(CONFIG_NAME);
        let contents = fs::read_to_string(path).unwrap();

        let config: Config = toml::from_str(&contents).unwrap();

        config
    };
    pub static ref SAVES_OPEN_OPTIONS: RwLock<HashMap<u32, OpenOptions>> =
        RwLock::new(HashMap::new());
}
