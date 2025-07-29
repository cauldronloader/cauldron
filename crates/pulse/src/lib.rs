use cauldron::prelude::*;
use once_cell::sync::Lazy;
use retour::static_detour;
use std::ffi::CString;
use std::ops::Deref;
use std::sync::Mutex;

static_detour! {
    static RegisterAllTypes: unsafe extern "C" fn();
}

unsafe fn register_all_types_impl() {
    unsafe { RegisterAllTypes.call() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cauldron_mod__load(loader_api: *const CauldronApi) -> bool {
    let loader = unsafe { &*loader_api };
    init_mod_logger(loader).expect("pulse: failed to initialize mod logger.");

    let c_namespace = CString::new("libdecima/game").unwrap();
    let c_name = CString::new("uh oh not yet").unwrap();

    let rtti_register_func_ptr = (loader.query_ptr)(c_namespace.as_ptr(), c_name.as_ptr());

    if !rtti_register_func_ptr.is_null() {
        unsafe {
            RegisterAllTypes
                .initialize(std::mem::transmute(rtti_register_func_ptr), || {
                    register_all_types_impl()
                })
                .unwrap()
        };

        unsafe { RegisterAllTypes.enable().unwrap() };
    }

    log::info!("Pulse loaded.");

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cauldron_mod__info() -> *const CauldronModInfo {
    let info = Box::new(
        CauldronModInfo::builder("pulse", env!("CARGO_PKG_VERSION"))
            .display_name("Pulse")
            .description("RTTI and symbol dumper.")
            .homepage_url("https://github.com/cauldron-decima/cauldron")
            .source_url("https://github.com/cauldron-decima/cauldron/tree/main/crates/pulse")
            .issue_tracker_url("https://github.com/cauldron-decima/cauldron/issues")
            .author("Pyrrha Wills <pyrrhawills@gmail.com>")
            .dependency(CauldronModDependency::new("hfw", Some(">=1.5.80"), false))
            // .dependency(CauldronModDependency::new("libdecima", None, false))
            .build(),
    );

    // note(py): yes im aware this explicitly leaks
    // todo(py): maybe look into sending the ptr back after load so the rust allocator can drop it?
    Box::into_raw(info)
}
