use parking_lot::RwLock;
use std::{collections::HashMap, env, fs, fs::OpenOptions};

use crate::{consts::CONFIG_NAME, models::config::Config};

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_path = env::current_dir().unwrap().to_path_buf().join(CONFIG_NAME);
        let config_data = fs::read_to_string(config_path).unwrap();

        let config: Config = toml::from_str(&config_data).unwrap();

        config
    };
    pub static ref SAVES_OPEN_OPTIONS: RwLock<HashMap<u32, OpenOptions>> =
        RwLock::new(HashMap::new());
}
