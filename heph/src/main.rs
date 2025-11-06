use cauldron_game_detection::steam::SteamId;
use cauldron_game_detection::{Game, Installation};
use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ENABLED_MODS: &[&str] = &["libdecima", "pulse"];

#[derive(Parser, Debug)]
#[command()]
/// The Cauldron Build Tool (Hephaestus)
struct Args {
    /// Choose game target.
    #[arg(short, long)]
    target: GameTarget,

    /// Skip game detection and output directly to this directory.
    #[arg(short = 'T', long)]
    target_dir: Option<PathBuf>,

    /// Set the cargo build profile.
    #[arg(short, long, value_enum, default_value_t)]
    profile: BuildProfile,

    /// Launch the target game after build and installation.
    #[arg(short, long)]
    launch: bool,
}

#[derive(ValueEnum, Clone, Debug, Hash, PartialEq, Eq)]
enum GameTarget {
    /// Horizon: Forbidden West
    #[value()]
    HFW,
    /// Horizon: Zero Dawn
    #[value()]
    HZD,

    /// Horizon: Zero Dawn Remastered
    #[value()]
    HZDR,

    /// DEATH STRANDING
    #[value()]
    DS,

    /// DEATH STRANDING DIRECTOR'S CUT
    #[value()]
    DSDC,
}

#[derive(ValueEnum, Clone, Debug, Hash, PartialEq, Eq, Default)]
enum BuildProfile {
    #[default]
    #[value()]
    Dev,
    #[value()]
    Release,
}

impl GameTarget {
    fn pretty_name(&self) -> String {
        let game: Game = self.into();
        game.pretty_name()
    }

    fn steam_id(&self) -> SteamId {
        let game: Game = self.into();
        (&game).into()
    }

    fn _feature_name(&self) -> &'static str {
        match self {
            GameTarget::HFW => "hfw",
            GameTarget::HZD => "hzd",
            GameTarget::HZDR => "hzdr",
            GameTarget::DS => "ds",
            GameTarget::DSDC => "dsdc",
        }
    }
}

impl Into<Game> for &GameTarget {
    fn into(self) -> Game {
        match self {
            GameTarget::HFW => Game::HorizonForbiddenWest,
            GameTarget::HZD => Game::HorizonZeroDawn,
            GameTarget::HZDR => Game::HorizonZeroDawn,
            GameTarget::DS => Game::DeathStranding,
            GameTarget::DSDC => Game::DeathStrandingDirectorsCut,
        }
    }
}

impl From<&Game> for GameTarget {
    fn from(value: &Game) -> Self {
        match value {
            Game::HorizonZeroDawn => GameTarget::HZD,
            Game::HorizonZeroDawnRemastered => GameTarget::HZDR,
            Game::HorizonForbiddenWest => GameTarget::HFW,
            Game::DeathStranding => GameTarget::DS,
            Game::DeathStrandingDirectorsCut => GameTarget::DSDC,
        }
    }
}

impl BuildProfile {
    fn cargo_profile(&self) -> &'static str {
        match self {
            BuildProfile::Dev => "dev",
            BuildProfile::Release => "release",
        }
    }

    fn cargo_target_dir(&self) -> &'static str {
        match self {
            BuildProfile::Dev => "debug",
            BuildProfile::Release => "release",
        }
    }
}

type DynError = Box<dyn std::error::Error>;

fn main() {
    let _ = dotenvy::dotenv(); // if .env exists load it but we dont really care if it doesnt.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let args = Args::parse();

    let target_dir = match args.target_dir {
        Some(dir) => {
            log::info!("Target directory provided, skipping discovery.");
            dir
        }
        None => {
            log::info!("Looking for installations...");
            let mut installations = cauldron_game_detection::find_installations();
            installations.sort_by(|a, b| match a {
                Installation::Steam { library_path, .. } => {
                    let a_library_path = library_path.clone();
                    match b {
                        Installation::Steam { library_path, .. } => {
                            a_library_path.cmp(library_path)
                        }
                    }
                }
            });

            log::info!("Found {} installations!", installations.len());
            let mut logged_paths = Vec::new();
            let mut matched_installs = Vec::new();
            for install in &installations {
                match install {
                    Installation::Steam {
                        game, library_path, ..
                    } => {
                        if !logged_paths.contains(&library_path.clone()) {
                            logged_paths.push(library_path.clone());
                            log::info!("  󰓓 {}", &library_path.to_str().unwrap().underline());
                        }
                        log::info!(
                            "    {} {} ({}/{}): {}",
                            if &args.target.steam_id() == &game.steam_id() {
                                "".bold().green()
                            } else {
                                "".normal()
                            },
                            &game.pretty_name(),
                            &game.code(),
                            &game.steam_id().0,
                            &game.steam_path(&library_path).to_str().unwrap().underline()
                        );

                        if &args.target.steam_id() == &game.steam_id() {
                            let install = install.clone();
                            matched_installs.push(install);
                        }
                    }
                }
            }

            if matched_installs.is_empty() {
                log::error!(
                    "Unable to find an installation for {}, please use `--target-dir` to set your target game's directory.",
                    &args.target.pretty_name()
                );
                std::process::exit(1);
            }

            match matched_installs.first().unwrap() {
                Installation::Steam {
                    game, library_path, ..
                } => game.steam_path(&library_path),
            }
        }
    };

    log::info!("Building Cauldron...");
    if build(&args.target, &args.profile).is_err() {
        log::error!("Cargo build failed, check logs and try again.");
        std::process::exit(1);
    }

    log::info!("Installing Cauldron...");
    match install(&target_dir, &args.profile) {
        Err(e) => {
            log::error!("Failed to install Cauldron: {e:?}.");
            std::process::exit(1);
        }
        _ => {}
    }
    if args.launch {
        log::info!("Launching...");
        open::that(format!("steam://rungameid/{}", &args.target.steam_id().0)).unwrap();
    }
    log::info!("Done!");
}

fn build(_target: &GameTarget, profile: &BuildProfile) -> Result<(), DynError> {
    let cargo = dotenvy::var("CARGO").unwrap_or("cargo".to_string());

    let mut args = Vec::new();
    if cfg!(not(target_os = "windows")) {
        args.push("xwin");
    }

    args.append(&mut vec![
        "build",
        "--target",
        "x86_64-pc-windows-msvc",
        "--profile",
        profile.cargo_profile(),
        // "--features",
        // target.feature_name(),
        "--package",
        "winhttp",
        "--package",
        "cauldron_loader",
    ]);
    for pkg in ENABLED_MODS {
        args.append(&mut vec!["--package", pkg]);
    }

    log::debug!("Running command: {} {}", cargo, args.join(" "));

    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(args.as_slice())
        .status()?;

    if !status.success() {
        log::error!("Cargo build failed.");
        Err("cargo build failed")?;
    }

    Ok(())
}

fn install(target_dir: &PathBuf, profile: &BuildProfile) -> Result<(), std::io::Error> {
    let loader_dir = target_dir.join("cauldron");
    let mods_dir = loader_dir.join("mods");
    let build_dir = project_root()
        .join("target")
        .join("x86_64-pc-windows-msvc")
        .join(profile.cargo_target_dir());

    let include_pdbs = profile == &BuildProfile::Dev;

    let mut target_outputs: Vec<&str> = vec!["winhttp.dll"];
    if include_pdbs {
        target_outputs.push("winhttp.pdb");
    }

    let mut loader_outputs: Vec<&str> = vec!["cauldron.dll"];
    if include_pdbs {
        loader_outputs.push("cauldron.pdb");
    }

    let mut mod_outputs: Vec<String> = vec![];
    for pkg in ENABLED_MODS {
        mod_outputs.push(format!("{pkg}.dll"));
    }
    if include_pdbs {
        for pkg in ENABLED_MODS {
            mod_outputs.push(format!("{pkg}.pdb"));
        }
    }

    if !mods_dir.exists() {
        fs::create_dir_all(&mods_dir)?;
    }

    log::debug!(
        "{}, exists: {}",
        loader_dir.to_str().unwrap(),
        fs::exists(&loader_dir)?
    );
    log::debug!(
        "{}, exists: {}",
        mods_dir.to_str().unwrap(),
        fs::exists(&mods_dir)?
    );
    log::debug!(
        "{}, exists: {}",
        build_dir.to_str().unwrap(),
        fs::exists(&build_dir)?
    );

    for output in target_outputs {
        log::debug!(
            "copying {} to {}",
            build_dir.join(output).to_string_lossy(),
            target_dir.join(output).to_string_lossy()
        );
        fs::copy(build_dir.join(output), target_dir.join(output))?;
    }

    for output in loader_outputs {
        log::debug!(
            "copying {} to {}",
            build_dir.join(output).to_string_lossy(),
            loader_dir.join(output).to_string_lossy()
        );
        fs::copy(build_dir.join(output), loader_dir.join(output))?;
    }

    for output in mod_outputs {
        log::debug!(
            "copying {} to {}",
            build_dir.join(&output.as_str()).to_string_lossy(),
            mods_dir.join(&output.as_str()).to_string_lossy()
        );
        fs::copy(
            build_dir.join(&output.as_str()),
            mods_dir.join(&output.as_str()),
        )?;
    }

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
