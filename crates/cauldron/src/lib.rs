use crate::log::LogLevel;
use std::ffi::{CString, c_char};

pub mod log;

#[repr(C)]
pub struct CauldronModInfo {
    pub name: *const c_char,
    pub version: u32,
    pub author: *const c_char,
}

impl CauldronModInfo {
    pub fn new(name: String, version: u32, author: Option<String>) -> Self {
        let c_name = CString::new(name).unwrap_or_default();
        let c_author = CString::new(author.unwrap_or_default()).unwrap_or_default();

        CauldronModInfo {
            name: c_name.as_ptr(),
            version,
            author: c_author.as_ptr(),
        }
    }
}

#[repr(C)]
pub struct CauldronApi {
    // reserved for future inter-mod api.
    pub __reserved_query: extern "C" fn(),
    pub __reserved_register: extern "C" fn(),

    /// Used to enable use of the [::log] crate across the FFI boundary.
    /// See [log::init_mod_logger]
    pub __internal_log_record:
        extern "C" fn(level: LogLevel, target: *const c_char, message: *const c_char),
}

pub mod prelude {
    pub use crate::CauldronApi;
    pub use crate::log::init_mod_logger;
}
