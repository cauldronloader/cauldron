use std::ffi::{c_char, CStr};
use cauldron_config::{LogLevel, VersionedConfig};
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use libloading::{Library, Symbol};
use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AllocConsole, AttachConsole};
use cauldron::CauldronApi;

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

pub extern "C" fn noop() {}

pub extern "C" fn loader_log_record(level: cauldron::log::LogLevel, target: *const c_char, message: *const c_char) {
    let target_str = unsafe { CStr::from_ptr(target).to_string_lossy() };
    let message_str = unsafe { CStr::from_ptr(message).to_string_lossy() };
    let log_level: log::Level = level.into();

    log::log!(target: &target_str, log_level, "{}", message_str);
}

static LOADER_API: CauldronApi = CauldronApi {
    __reserved_query: noop,
    __reserved_register: noop,

    __internal_log_record: loader_log_record,
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
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            write_level,
            Config::default(),
            File::create("cauldron/cauldron.log").unwrap(),
        ),
    ])
    .expect("Failed to initialize logger");

    log::info!("Starting Cauldron v{}...", env!("CARGO_PKG_VERSION"));


    let mods_dir = std::fs::read_dir("cauldron/mods").expect("Failed to read mods dir");
    for entry in mods_dir {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |ext| ext == "dll") {
            log::info!("Loading mod at {}", path.display());

            let lib = unsafe { Library::new(&path).unwrap() };

            let init_func: Symbol<unsafe extern "C" fn(*const CauldronApi)> = unsafe { lib.get(b"cauldron_mod__load\0").unwrap() };

            unsafe { init_func(&LOADER_API as *const CauldronApi) };

            std::mem::forget(lib);
        }
    }
}
