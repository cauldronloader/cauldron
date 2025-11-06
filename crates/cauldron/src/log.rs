use crate::CauldronApi;
use std::ffi::{CString, c_char};

#[repr(C)]
pub enum LogLevel {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<log::Level> for LogLevel {
    fn from(value: log::Level) -> Self {
        match value {
            log::Level::Error => LogLevel::Error,
            log::Level::Warn => LogLevel::Warn,
            log::Level::Info => LogLevel::Info,
            log::Level::Debug => LogLevel::Debug,
            log::Level::Trace => LogLevel::Trace,
        }
    }
}

impl From<LogLevel> for log::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => log::Level::Error,
            LogLevel::Warn => log::Level::Warn,
            LogLevel::Info => log::Level::Info,
            LogLevel::Debug => log::Level::Debug,
            LogLevel::Trace => log::Level::Trace,
        }
    }
}

pub fn init_mod_logger(loader: &CauldronApi) -> Result<(), log::SetLoggerError> {
    let logger = ModLogger {
        log_func: loader.log,
    };

    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(log::LevelFilter::Trace))
}

struct ModLogger {
    log_func: extern "C" fn(level: LogLevel, target: *const c_char, message: *const c_char),
}

impl log::Log for ModLogger {
    fn enabled(&self, _metadata: &::log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let target = CString::new(record.target()).unwrap_or_default();
        let message = CString::new(format!("{}", record.args())).unwrap_or_default();

        (self.log_func)(record.level().into(), target.as_ptr(), message.as_ptr());
    }

    fn flush(&self) {}
}
