//! Cauldron mod metadata spec

/// Utilities for reading and parsing [CauldronModMetadata] files.
pub mod read;

use serde::Deserialize;

/// Rust representation of a `<name>.mod.toml` metadata file.
#[derive(Debug, Clone, Deserialize)]
pub struct CauldronModMetadata {
    pub r#mod: ModSpec,
}

/// The core mod specification.
#[derive(Debug, Clone, Deserialize)]
pub struct ModSpec {
    /// Unique identifier for the mod.
    /// Must match `[a-z0-9_-]{3,64}`.
    pub name: String,
    /// Semver-compliant version.
    #[serde(deserialize_with = "read::deserialize_version")]
    pub version: semver::Version,

    /// Human-readable display name.
    pub display_name: Option<String>,
    /// List of mod authors.
    /// Can optionally include an email in angle brackets e.g. `Pyrrha Wills` or `Pyrrha Wills <pyrrhawills@gmail.com>`.
    pub authors: Option<Vec<String>>,
    /// Mod description, can be multiline.
    pub description: Option<String>,
    /// SPDX license identifier.
    pub license: Option<String>,

    /// Homepage url.
    pub homepage: Option<String>,
    /// Source repository url.
    pub repository: Option<String>,
    /// Issue tracker url.
    pub issue_tracker: Option<String>,

    /// Mod dependencies.
    pub dependencies: Option<Vec<DependencySpec>>,
}

/// A mod's dependency specification.
#[derive(Debug, Clone, Deserialize)]
pub struct DependencySpec {
    /// Dependency name.
    /// Matched against [ModSpec::name].
    pub name: String,
    /// A Semver version constraint.
    #[serde(deserialize_with = "read::deserialize_version_req")]
    pub version: semver::VersionReq,
    /// Dependency order specification.
    pub order: DependencyOrderSpec,
    /// Whether the dependency is optional.
    pub optional: bool,
}

/// Dependency load order specification.
#[derive(Debug, Clone, Deserialize, Default)]
pub enum DependencyOrderSpec {
    /// Place the dependency before the mod in the load order. (Default)
    #[default]
    Before,
    /// Place the dependency after the mod in the load order.
    After,
}
