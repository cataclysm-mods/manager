use app_dirs2::{AppDataType, AppInfo, get_app_root};
use std::path::PathBuf;

pub mod http_client;
pub mod log;
pub mod models;

pub const NAME: &'static str = "cdda-manager";

// Metadata provided by Cargo
// See: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const DIR_CONFIG: AppInfo = AppInfo{
    name: NAME,
    author: "cataclysm-mods"
};

/// This const is used to give us a simple way to iterate over
/// the kinds of configuration directories defined by the XDG
/// directory specification or their equivilants
/// present across multiple operating systems.
const DIR_CONFIG_TYPES: [AppDataType; 3] = [
    AppDataType::UserCache,
    AppDataType::UserConfig,
    AppDataType::UserData,
];

#[derive(Debug)]
pub struct Config {
    pub user_cache: Option<PathBuf>,
    pub user_config: Option<PathBuf>,
    pub user_data: Option<PathBuf>,
}

impl Config {}

impl Default for Config {
    /// Creates a new Config struct using default values.
    /// Detected configuration files and folders will be populated.
    fn default() -> Self {
        let mut config = Config {
            user_cache: None,
            user_config: None,
            user_data: None,
        };

        for dir_type in DIR_CONFIG_TYPES.iter() {
            let path = match get_app_root(*dir_type, &DIR_CONFIG) {
                Ok(p) => Some(p),
                Err(_e) => None
            };

            match *dir_type {
                AppDataType::UserCache  => config.user_cache = path,
                AppDataType::UserConfig => config.user_config = path,
                AppDataType::UserData   => config.user_data = path,
                _ => unreachable!(),
            }
        }

        config
    }
}
