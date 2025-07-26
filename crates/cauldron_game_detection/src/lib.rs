// This crate is loosely based off of [SatisfactoryModding/SatisfactoryModManager](https://github.com/satisfactorymodding/SatisfactoryModManager/tree/master/backend/installfinders)'s install finders.

use crate::steam::SteamInstallPlatform;
use semver::{BuildMetadata, Version};
use std::env::current_exe;
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

    /// DEATH STRANDING(DS)
    DeathStranding,

    /// DEATH STRANDING DIRECTOR'S CUT (DSDC)
    DeathStrandingDirectorsCut,
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
            Game::DeathStranding => "DEATH STRANDING",
            Game::DeathStrandingDirectorsCut => "DEATH STRANDING DIRECTOR'S CUT",
        }
        .into()
    }

    pub fn code(&self) -> String {
        match self {
            Game::HorizonZeroDawn => "hzd",
            Game::HorizonZeroDawnRemastered => "hzdr",
            Game::HorizonForbiddenWest => "hfw",
            Game::DeathStranding => "ds",
            Game::DeathStrandingDirectorsCut => "dsdc",
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

/// Find the current [Game] and [Version].
pub fn detect_active() -> Option<(Game, Version)> {
    let game: Option<Game> = match current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
    {
        "HorizonZeroDawn.exe" => Some(Game::HorizonZeroDawn),
        "HorizonZeroDawnRemastered.exe" => Some(Game::HorizonZeroDawnRemastered),
        "HorizonForbiddenWest.exe" => Some(Game::HorizonForbiddenWest),
        "ds.exe" => Some(Game::DeathStrandingDirectorsCut),
        // todo(py): differentiate between ds and dsdc, they both use the same "ds.exe" filename. (¬_¬")
        _ => None,
    };

    let Some(game) = game else {
        return None;
    };

    Some((game, get_current_version()))
}

#[cfg(windows)]
fn get_current_version() -> Version {
    use windows::Win32::Storage::FileSystem::{
        GetFileVersionInfoSizeW, GetFileVersionInfoW, VS_FIXEDFILEINFO, VerQueryValueW,
    };
    use windows::core::{HSTRING, PCWSTR, w};

    let mut ver = Version::new(0, 0, 0);

    let path = current_exe().unwrap();
    let mut version_info_size = unsafe {
        GetFileVersionInfoSizeW(
            PCWSTR::from_raw(HSTRING::from(path.as_path()).as_ptr()),
            None,
        )
    };
    let mut version_info_buf = vec![0u8; version_info_size as usize];
    unsafe {
        GetFileVersionInfoW(
            PCWSTR::from_raw(HSTRING::from(path.as_path()).as_ptr()),
            None,
            version_info_size,
            version_info_buf.as_mut_ptr() as _,
        )
        .unwrap()
    };

    let mut version_info: *mut VS_FIXEDFILEINFO = std::ptr::null_mut();
    unsafe {
        let _ = VerQueryValueW(
            version_info_buf.as_ptr() as _,
            w!("\\\\\0"),
            &mut version_info as *mut *mut _ as _,
            &mut version_info_size,
        );
    };

    let version_info = unsafe { version_info.as_ref().unwrap() };

    ver.major = ((version_info.dwFileVersionMS >> 16) & 0xfff) as u64;
    ver.minor = ((version_info.dwFileVersionMS) & 0xfff) as u64;
    ver.patch = ((version_info.dwFileVersionLS >> 16) & 0xfff) as u64;
    ver.build =
        BuildMetadata::new(format!("{}", (version_info.dwFileVersionLS) & 0xfff).as_str()).unwrap();

    ver
}

#[cfg(not(windows))]
fn get_current_version() -> Version {
    let mut ver = Version::new(0, 0, 0);
    ver.build = BuildMetadata::new("UNKNOWN").unwrap();
    ver
}
