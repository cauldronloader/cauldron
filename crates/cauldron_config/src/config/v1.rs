use crate::LogLevel;
use crate::config::Config;
use documented::DocumentedFields;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DocumentedFields)]
pub struct CauldronConfig {
    /// ### DO NOT EDIT ###
    ///
    /// This is used so Cauldron can migrate older versions of the file.
    pub config_version: u32,

    /// Logging configuration.
    pub logging: CauldronLoggingConfig,

    /// Patch configuration.
    pub patches: CauldronPatchConfig,

    /// Configure how our `winhttp.dll` proxy loads Cauldron itself.
    pub proxy_loader: CauldronProxyLoaderConfig,
}

#[derive(Serialize, Deserialize, DocumentedFields)]
pub struct CauldronLoggingConfig {
    /// Should a console window be opened at launch.
    ///
    /// Type: Boolean (true, false)
    /// Default: true
    pub show_console: bool,
    /// Disable console colours, may be useful on Proton.
    ///
    /// Type: Boolean (true, false)
    /// Default: false
    pub disable_colours: bool,
    /// Log level for the console logger.
    ///
    /// Type: Enum (Error, Warn, Info, Debug, Trace)
    /// Default: "Info"
    pub console_level: LogLevel,
    /// Log file path relative to the game's directory.
    ///
    /// Type: String
    /// Default: "cauldron/cauldron.log"
    pub file_path: String,
    /// Log level for the file logger.
    /// Warning: Having this above Info may create very large log files.
    ///
    /// Type: Enum (Error, Warn, Info, Debug, Trace)
    /// Default: "Info"
    pub file_level: LogLevel,
}

#[derive(Serialize, Deserialize, DocumentedFields)]
pub struct CauldronPatchConfig {
    /// Disables game telemetry from being sent to Guerrilla Games and Sony.
    ///
    /// Type: Boolean (true, false)
    /// Default: true
    pub disable_telemetry: bool,

    /// Disables the Sony crash reporter.
    ///
    /// Type: Boolean (true, false)
    /// Default: true
    pub disable_crash_reporter: bool,
}

#[derive(Serialize, Deserialize, DocumentedFields)]
pub struct CauldronProxyLoaderConfig {
    /// Sets `RUST_BACKTRACE` to `FULL` if true, useful for debugging.
    ///
    /// Type: Boolean
    /// Default: false
    pub enable_rust_backtracing: bool,

    /// Locks the main thread before loading the loader dll and waits for a debugger to attach.
    ///
    /// Type: Boolean
    /// Default: false
    pub wait_for_debugger: bool,

    /// Path to the loader dll file.
    ///
    /// Type: String
    /// Default: "cauldron/cauldron.dll"
    pub loader_file: String,
}

impl Config for CauldronConfig {}

impl Default for CauldronConfig {
    fn default() -> Self {
        CauldronConfig {
            config_version: 1,
            logging: CauldronLoggingConfig {
                show_console: true,
                console_level: LogLevel::Debug,
                disable_colours: false,
                file_path: "cauldron/cauldron.log".into(),
                file_level: LogLevel::Info,
            },
            patches: CauldronPatchConfig {
                disable_telemetry: true,
                disable_crash_reporter: true,
            },
            proxy_loader: CauldronProxyLoaderConfig {
                enable_rust_backtracing: false,
                wait_for_debugger: false,
                loader_file: "cauldron/cauldron.dll".into(),
            },
        }
    }
}
