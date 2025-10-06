use crate::CauldronModMetadata;
use serde::de::Error;
use std::fmt::Formatter;
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

/// A [serde::Deserializer] for a [semver::Version].
pub fn deserialize_version<'de, D>(deserializer: D) -> std::result::Result<semver::Version, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_str(VersionVisitor)
}

/// A [serde::Deserializer] for a [semver::VersionReq].
pub fn deserialize_version_req<'de, D>(deserializer: D) -> std::result::Result<semver::VersionReq, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_str(VersionReqVisitor)
}

struct VersionVisitor;
impl<'de> serde::de::Visitor<'de> for VersionVisitor {
    type Value = semver::Version;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a semver-compliant version string")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: Error,
    {
        semver::Version::parse(v).map_err(E::custom)
    }
}

struct VersionReqVisitor;
impl<'de> serde::de::Visitor<'de> for VersionReqVisitor {
    type Value = semver::VersionReq;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a semver-compliant constraint string")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: Error,
    {
        semver::VersionReq::parse(v).map_err(E::custom)
    }
}
