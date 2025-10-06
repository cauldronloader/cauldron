use crate::CauldronModMetadata;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum MetadataReadError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, MetadataReadError>;

impl CauldronModMetadata {
    /// Read and parse a [CauldronModMetadata] file.
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let str = std::fs::read_to_string(path)?;
        let data = toml::de::from_str::<Self>(&str)?;
        Ok(data)
    }
}
