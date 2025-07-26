use crate::log::LogLevel;
use std::ffi::{CString, c_char, c_void};

pub mod log;
pub mod mod_info;

#[repr(C)]
pub struct CauldronApi {
    pub query_ptr: extern "C" fn(namespace: *const c_char, name: *const c_char) -> *const c_void,
    pub register_ptr:
        extern "C" fn(namespace: *const c_char, name: *const c_char, ptr: *const c_void) -> bool,

    /// Your bog-standard logging function.
    pub log: extern "C" fn(level: LogLevel, target: *const c_char, message: *const c_char),
}

impl CauldronApi {
    pub fn query(&self, namespace: String, name: String) -> Option<*const c_void> {
        let c_namespace = CString::new(namespace).unwrap();
        let c_name = CString::new(name).unwrap();

        let result = (self.query_ptr)(c_namespace.as_ptr(), c_name.as_ptr());
        if result.is_null() { None } else { Some(result) }
    }

    pub fn register(&self, namespace: String, name: String, ptr: *const c_void) -> bool {
        let c_namespace = CString::new(namespace).unwrap();
        let c_name = CString::new(name).unwrap();

        (self.register_ptr)(c_namespace.into_raw(), c_name.into_raw(), ptr)
    }
}

pub mod prelude {
    pub use crate::CauldronApi;
    pub use crate::log::LogLevel;
    pub use crate::log::init_mod_logger;
    pub use crate::mod_info::CauldronModDependency;
    pub use crate::mod_info::CauldronModInfo;
}
