use crate::{Game, InstallPlatform, Installation};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone)]
pub struct LibraryFolders {
    #[serde(flatten)]
    pub folders: HashMap<String, LibraryFolder>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LibraryFolder {
    pub path: String,
    pub label: String,
    #[serde(rename = "contentid")]
    pub content_id: String,
    #[serde(rename = "totalsize")]
    pub total_size: String,
    pub update_clean_bytes_tally: String,
    pub time_last_update_verified: String,
    pub apps: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct SteamId(pub u32);



pub const STEAM_ID_HZD: SteamId = SteamId(1151640); // https://steamdb.info/app/1151640/
pub const STEAM_ID_HZDR: SteamId = SteamId(2561580); // https://steamdb.info/app/2561580/
pub const STEAM_ID_HFW: SteamId = SteamId(2420110); // https://steamdb.info/app/2420110/
pub const STEAM_ID_DS: SteamId = SteamId(1190460); // https://steamdb.info/app/1190460/
pub const STEAM_ID_DSDC: SteamId = SteamId(1850570); // https://steamdb.info/app/1850570/

impl TryFrom<String> for SteamId {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(SteamId(u32::from_str(value.as_str())?))
    }
}

impl LibraryFolders {
    pub fn try_parse(library_folders_vdf: String) -> Option<LibraryFolders> {
        if let Ok(out) = keyvalues_serde::from_str(&library_folders_vdf) {
            out
        } else {
            None
        }
    }
}

impl Into<SteamId> for &Game {
    fn into(self) -> SteamId {
        match self {
            Game::HorizonZeroDawn => STEAM_ID_HZD,
            Game::HorizonZeroDawnRemastered => STEAM_ID_HZDR,
            Game::HorizonForbiddenWest => STEAM_ID_HFW,
            Game::DeathStranding => STEAM_ID_DS,
            Game::DeathStrandingDirectorsCut => STEAM_ID_DSDC,
        }
    }
}

impl TryInto<Game> for &SteamId {
    type Error = ();

    fn try_into(self) -> Result<Game, Self::Error> {
        match self {
            &STEAM_ID_HZD => Ok(Game::HorizonZeroDawn),
            &STEAM_ID_HZDR => Ok(Game::HorizonZeroDawnRemastered),
            &STEAM_ID_HFW => Ok(Game::HorizonForbiddenWest),
            &STEAM_ID_DS => Ok(Game::DeathStranding),
            &STEAM_ID_DSDC => Ok(Game::DeathStrandingDirectorsCut),
            _ => Err(()),
        }
    }
}

impl TryFrom<SteamId> for Game {
    type Error = ();

    fn try_from(value: SteamId) -> Result<Self, Self::Error> {
        match value {
            STEAM_ID_HZD => Ok(Game::HorizonZeroDawn),
            STEAM_ID_HZDR => Ok(Game::HorizonZeroDawnRemastered),
            STEAM_ID_HFW => Ok(Game::HorizonForbiddenWest),
            STEAM_ID_DS => Ok(Game::DeathStranding),
            STEAM_ID_DSDC => Ok(Game::DeathStrandingDirectorsCut),

            _ => Err(()),
        }
    }
}

impl Game {
    pub fn steam_install_dir(&self) -> PathBuf {
        match self {
            Game::HorizonZeroDawn => PathBuf::from("Horizon Zero Dawn"),
            Game::HorizonZeroDawnRemastered => PathBuf::from("Horizon Zero Dawn Remastered"),
            Game::HorizonForbiddenWest => PathBuf::from("Horizon Forbidden West Complete Edition"),
            Game::DeathStranding => PathBuf::from("DEATH STRANDING"),
            Game::DeathStrandingDirectorsCut => PathBuf::from("DEATH STRANDING DIRECTORS CUT"),
        }
    }

    pub fn steam_id(&self) -> SteamId {
        self.into()
    }

    pub fn steam_path(&self, library_path: &PathBuf) -> PathBuf {
        library_path
            .join("steamapps")
            .join("common")
            .join(self.steam_install_dir())
    }
}

pub struct SteamInstallPlatform;
impl InstallPlatform for SteamInstallPlatform {
    fn find_installations() -> Vec<Installation> {
        // todo(py): replace return with Result<Vec<Installations>, GameDetectionError>

        let steam_path = if cfg!(target_os = "windows") {
            let native = PathBuf::from("C:\\Program Files (x86)\\Steam");
            if !native.exists() {
                // cannot find steam install
                return Vec::new()
            } else {
                native
            }
        } else if cfg!(target_os = "linux") {
            let native = PathBuf::from(dotenvy::var("HOME").unwrap())
                .join(".steam")
                .join("steam");
            let flatpak = PathBuf::from(dotenvy::var("HOME").unwrap())
                .join(".var")
                .join("app")
                .join("com.valvesoftware.Steam")
                .join(".steam")
                .join("steam");
            let snap = PathBuf::from(dotenvy::var("HOME").unwrap())
                .join("snap")
                .join("steam")
                .join("common")
                .join(".local")
                .join("share")
                .join("Share");

            // priority: native, flatpak, snap
            if !native.exists() {
                if !flatpak.exists() {
                    if !snap.exists() {
                        // cannot find steam install
                        return Vec::new()
                    } else {
                        snap
                    }
                } else {
                    flatpak
                }
            } else {
                native
            }

        } else {
            // not supported os
            return Vec::new();
        };

        let library_folders_path = steam_path.join("steamapps").join("libraryfolders.vdf");
        if !library_folders_path.exists() {
            // library folders vdf not found
            return Vec::new();
        }

        let Ok(library_folders_vdf) = fs::read_to_string(library_folders_path) else {
            // couldn't read library folders vdf
            return Vec::new();
        };

        let Ok(library_folders_vdf) =
            keyvalues_serde::from_str::<LibraryFolders>(library_folders_vdf.as_str())
        else {
            // failed to parse vdf
            return Vec::new();
        };

        let mut installs = Vec::new();
        for (_, folder) in library_folders_vdf.folders {
            for (app, _) in folder.apps {
                let Ok(app_id) = SteamId::try_from(app) else {
                    // couldn't parse app id
                    continue;
                };
                let Ok(game) = Game::try_from(app_id) else {
                    // app id isnt a known game.
                    continue;
                };

                installs.push(Installation::Steam {
                    run_command: format!("steam://rungameid/{}", game.steam_id().0.clone()),
                    game,
                    library_path: PathBuf::from(folder.path.clone()),
                })
            }
        }

        installs
    }
}
