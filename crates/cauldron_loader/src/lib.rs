pub mod util;

use crate::util::message_box;
use cauldron::mod_info::{SafeCauldronModDependency, SafeCauldronModInfo};
use cauldron::prelude::{CauldronApi, CauldronModInfo};
use cauldron_config::{LogLevel, VersionedConfig};
use libloading::{Library, Symbol};
use once_cell::sync::Lazy;
use semver::{Version, VersionReq};
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::{CStr, c_char, c_void};
use std::fs::File;
use std::sync::Mutex;
use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AllocConsole, AttachConsole};

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_: usize, reason: u32, _: isize) -> bool {
    match reason {
        // DLL_PROCESS_ATTACH
        1 => unsafe {
            std::thread::spawn(|| initialize_loader());
        },

        // DLL_PROCESS_DETACH
        0 => {
            // todo(py): graceful shutdown
        }
        _ => {}
    }
    true
}

static LOADER_STATE: Lazy<Mutex<LoaderState>> = Lazy::new(|| Mutex::new(LoaderState::default()));

struct LoaderState {
    registered_funcs: HashMap<String, HashMap<String, *const c_void>>,
}

impl Default for LoaderState {
    fn default() -> Self {
        LoaderState {
            registered_funcs: HashMap::new(),
        }
    }
}

unsafe impl Send for LoaderState {}
unsafe impl Sync for LoaderState {}

pub extern "C" fn loader_query_ptr_impl(
    namespace: *const c_char,
    name: *const c_char,
) -> *const c_void {
    let c_namespace = unsafe { CStr::from_ptr(namespace) };
    let c_name = unsafe { CStr::from_ptr(name) };
    let r_namespace = c_namespace.to_str().unwrap().to_owned();
    let r_name = c_name.to_str().unwrap().to_owned();

    if let Some(funcs) = LOADER_STATE
        .lock()
        .unwrap()
        .registered_funcs
        .get_mut(&r_namespace)
    {
        if let Some(func) = funcs.get(&r_name) {
            return *func;
        }
    }

    std::ptr::null()
}

pub extern "C" fn loader_register_ptr_impl(
    namespace: *const c_char,
    name: *const c_char,
    function: *const c_void,
) -> bool {
    let c_namespace = unsafe { CStr::from_ptr(namespace) };
    let c_name = unsafe { CStr::from_ptr(name) };
    let r_namespace = c_namespace.to_str().unwrap().to_owned();
    let r_name = c_name.to_str().unwrap().to_owned();

    let mut added = false;

    LOADER_STATE
        .lock()
        .unwrap()
        .registered_funcs
        .entry(r_namespace)
        .or_default()
        .entry(r_name)
        .or_insert_with(|| {
            added = true;
            function
        });

    added
}

pub extern "C" fn loader_log_impl(
    level: cauldron::log::LogLevel,
    target: *const c_char,
    message: *const c_char,
) {
    let target_str = unsafe { CStr::from_ptr(target).to_string_lossy() };
    let message_str = unsafe { CStr::from_ptr(message).to_string_lossy() };
    let log_level: log::Level = level.into();

    log::log!(target: &target_str, log_level, "{}", message_str);
}

static LOADER_API: CauldronApi = CauldronApi {
    query_ptr: loader_query_ptr_impl,
    register_ptr: loader_register_ptr_impl,
    log: loader_log_impl,
};

unsafe fn initialize_loader() {
    let config = cauldron_config::load_config_or_write_default();
    let config = match config {
        VersionedConfig::V1(cauldron_config) => cauldron_config,
    };

    if config.logging.show_console {
        unsafe {
            AllocConsole();
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    let term_level = match config.logging.console_level {
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
    };
    let write_level = match config.logging.file_level {
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
    };
    CombinedLogger::init(vec![
        TermLogger::new(
            term_level,
            Config::default(),
            TerminalMode::Mixed,
            if config.logging.disable_colours {
                ColorChoice::Never
            } else {
                ColorChoice::Auto
            },
        ),
        WriteLogger::new(
            write_level,
            Config::default(),
            File::create("cauldron/cauldron.log").unwrap(),
        ),
    ])
    .expect("Failed to initialize logger");

    log::info!("Starting Cauldron v{}...", env!("CARGO_PKG_VERSION"));
    let game_ver_tuple = cauldron_game_detection::detect_active();

    let (game, game_version) = match game_ver_tuple {
        None => {
            log::error!("Unable to detect the running game or version.");
            return;
        }
        Some((game, ver)) => {
            log::info!("Running on {} v{}", game.pretty_name(), ver);

            (game, ver)
        }
    };

    let mods_dir = std::fs::read_dir("cauldron/mods").expect("Failed to read mods dir");
    let mut loading_mods: Vec<(Library, SafeCauldronModInfo)> = Vec::new();
    let mut mod_versions: HashMap<String, Version> = HashMap::new();

    // add active game to mod_versions
    mod_versions.insert(game.code(), game_version.clone());

    for entry in mods_dir {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |ext| ext == "dll") {
            log::debug!("Loading mod at {}", path.display());

            let lib = unsafe { Library::new(&path).unwrap() };

            let info_func: Symbol<unsafe extern "C" fn() -> *const CauldronModInfo> =
                unsafe { lib.get(b"cauldron_mod__info\0").unwrap() };

            let mod_info = unsafe { &*info_func() };
            let mod_info = SafeCauldronModInfo::from(mod_info.clone());
            log::debug!("{mod_info:?}");
            loading_mods.push((lib, mod_info));
        }
    }

    // parse versions
    for (_, mod_info) in &loading_mods {
        let version = match Version::parse(&mod_info.version) {
            Ok(v) => v,
            Err(e) => {
                message_box(
                    "Mod Loading Error",
                    format!(
                        "{} has an invalid version string, \"{}\", must be semver compliant.",
                        &mod_info.name, &mod_info.version
                    )
                    .as_str(),
                    0u32 | 16u32, /* MB_OK | MB_ICONERROR */
                );

                log::error!(
                    "Failed to parse {}'s version string as semver version: {e}, exiting.",
                    &mod_info.name
                );
                std::process::exit(0);
            }
        };

        mod_versions.insert(mod_info.name.clone(), version);
    }

    // ensure all required dependencies are loaded
    for (_, mod_info) in &loading_mods {
        for dep in &mod_info.dependencies {
            let version = match VersionReq::parse(&match &dep.version {
                None => String::from("*"),
                Some(v) => v.clone(),
            }) {
                Ok(v) => v,
                Err(e) => {
                    message_box("Mod Loading Error", format!("{} has an invalid dependency version constraint for {}: \"{}\", see log for mod details.", &mod_info.name, &dep.name, &match &dep.version {
                        None => String::from("*"),
                        Some(v) => v.clone(),
                    }).as_str(), 0u32 | 16u32 /* MB_OK | MB_ICONERROR */);
                    log::error!(
                        "Mod {} has an invalid dependency version constraint for {}: {e}, exiting.",
                        &mod_info.name,
                        &dep.name
                    );
                    std::process::exit(0);
                }
            };

            if !mod_versions.contains_key(&dep.name) && !dep.optional {
                // dep not found but is required
                log::error!(
                    "{} requires {} {version} to be installed.",
                    &mod_info.name,
                    &dep.name
                );
                message_box(
                    "Mod Loading Error",
                    format!(
                        "{} lists {} as a required dependency but it is not present.",
                        &mod_info.name, &dep.name,
                    )
                    .as_str(),
                    0u32 | 16u32, /* MB_OK | MB_ICONERROR */
                );
                std::process::exit(0);
            } else if !version.matches(&mod_versions[&dep.name]) && dep.optional {
                // is optional but versions dont match
                log::error!(
                    "{} optionally depends on {} {version} but {} is installed instead, exiting.",
                    &mod_info.name,
                    &dep.name,
                    &mod_versions[&dep.name]
                );
                message_box("Mod Loading Error", format!("{} lists {} as an optional dependency but a version that doesn't fit the required constraints is present. (required: {version}, present: {})", &mod_info.name, &dep.name, &mod_versions[&dep.name]).as_str(), 0u32 | 16u32 /* MB_OK | MB_ICONERROR */);
                std::process::exit(0);
            }
        }
    }

    loading_mods.sort_by(|(_, a), (_, b)| {
        let a_deps = a.dependencies.iter().map(|d| &d.name).collect::<Vec<_>>();
        let b_deps = b.dependencies.iter().map(|d| &d.name).collect::<Vec<_>>();

        if a_deps.contains(&&b.name) && b_deps.contains(&&a.name) {
            // circular dependencies
            log::error!(
                "circular dependencies: {} and {} both depend on each other.",
                &a.name,
                &b.name
            );
            message_box(
                "Mod Loading Error",
                format!("{} and {} both depend on each other.", &a.name, &b.name).as_str(),
                0u32 | 16u32, /* MB_OK | MB_ICONERROR */
            );
            std::process::exit(0);
        } else if a_deps.contains(&&b.name) {
            // a depends on b
            Ordering::Greater
        } else if b_deps.contains(&&a.name) {
            // b depends on a
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    // todo(py): table these (see https://github.com/QuiltMC/quilt-loader/blob/0a17274320a646551abb04435d810158988f0fcc/src/main/java/org/quiltmc/loader/impl/QuiltLoaderImpl.java#L819)
    let mut mods_string = format!("\t 0. {} v{}", game.code(), &game_version);
    for (i, (_, mod_info)) in loading_mods.iter().enumerate() {
        mods_string.push_str(&format!(
            "\n\t {}. {} v{}",
            i + 1,
            &mod_info.name,
            &mod_info.version
        ));
    }

    log::info!("Found {} mods:\n{mods_string}", mod_versions.len());
    log::info!("Loading mods...");

    for (lib, _) in &loading_mods {
        let init_func: Symbol<unsafe extern "C" fn(*const CauldronApi) -> bool> =
            unsafe { lib.get(b"cauldron_mod__load\0").unwrap() };

        let _load_status = unsafe { init_func(&LOADER_API as *const CauldronApi) };
        // todo(py): handle mod load failure
    }

    std::mem::forget(loading_mods);
}
