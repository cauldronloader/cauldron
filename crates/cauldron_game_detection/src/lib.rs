use crate::steam::SteamInstallPlatform;
/// This crate is loosely based off of [SatisfactoryModding/SatisfactoryModManager](https://github.com/satisfactorymodding/SatisfactoryModManager/tree/master/backend/installfinders)'s install finders.
use std::path::PathBuf;

pub mod steam;

/// A discovered game installation, keyed by launcher.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Installation {
    Steam {
        game: Game,
        library_path: PathBuf,
        run_command: String,
    },
    // todo(py): EpicGames, Lutris, Heroic
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Game {
    /// Horizon: Zero Dawn (HZD)
    HorizonZeroDawn,

    /// Horizon: Zero Dawn Remastered (HZDR)
    HorizonZeroDawnRemastered,

    /// Horizon: Forbidden West (HFW)
    HorizonForbiddenWest,
}

trait InstallPlatform {
    fn find_installations() -> Vec<Installation>;
}

impl Game {
    pub fn pretty_name(&self) -> String {
        match self {
            Game::HorizonZeroDawn => "Horizon: Zero Dawn",
            Game::HorizonZeroDawnRemastered => "Horizon: Zero Dawn Remastered",
            Game::HorizonForbiddenWest => "Horizon: Forbidden West",
        }
        .into()
    }

    pub fn code(&self) -> String {
        match self {
            Game::HorizonZeroDawn => "hzd",
            Game::HorizonZeroDawnRemastered => "hzdr",
            Game::HorizonForbiddenWest => "hfw",
        }
        .into()
    }
}

/// Find all installations of [Game] on the system.
pub fn find_installations() -> Vec<Installation> {
    let mut installations = Vec::new();

    installations.append(&mut SteamInstallPlatform::find_installations());

    installations
}
