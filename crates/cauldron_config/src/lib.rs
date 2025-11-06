use crate::config::{CauldronConfigVersionOnly, Config};
use serde::{Deserialize, Serialize};
use std::fs;
use toml_edit::de::from_str;

pub mod config;

pub mod prelude {
    pub use crate::config::CauldronConfigVersionOnly;
    pub use crate::config::v1::CauldronConfig;

    pub use crate::VersionedConfig;
    pub use crate::load_config;
    pub use crate::load_config_or_default;
}

/// An intermediary log level used for config files.
#[derive(Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Toml Error: {0}")]
    Toml(#[from] toml_edit::TomlError),

    #[error("Toml Deserialization Error: {0}")]
    TomlDe(#[from] toml_edit::de::Error),

    #[error("Toml Serialization Error: {0}")]
    TomlSe(#[from] toml_edit::ser::Error),

    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown Config Version: {0}")]
    UnknownConfigVersion(u32),

    #[error("Unknown Error: {0}")]
    Unknown(String),
}

pub enum VersionedConfig {
    V1(config::v1::CauldronConfig),
}

impl Default for VersionedConfig {
    fn default() -> Self {
        VersionedConfig::V1(prelude::CauldronConfig::default())
    }
}

pub fn load_config() -> Result<VersionedConfig> {
    let file = fs::read_to_string("cauldron/cauldron.toml")?;
    let version = from_str::<CauldronConfigVersionOnly>(file.as_str())?;

    match version.config_version {
        1 => {
            let config = from_str::<config::v1::CauldronConfig>(file.as_str())?;
            Ok(VersionedConfig::V1(config))
        }
        unknown => Err(Error::UnknownConfigVersion(unknown)),
    }
}

pub fn load_config_or_default() -> VersionedConfig {
    load_config().unwrap_or_else(|_| VersionedConfig::default())
}

pub fn load_config_or_write_default() -> VersionedConfig {
    let Ok(config) = load_config() else {
        let default_config = prelude::CauldronConfig::default();
        let config_document = default_config
            .as_annotated_toml::<prelude::CauldronConfig>()
            .expect("failed to create default annotated config file");

        fs::write(
            "cauldron/cauldron.toml",
            config_document.to_string().as_bytes(),
        )
        .expect("failed to write default config file");

        return VersionedConfig::V1(default_config);
    };

    config
}
