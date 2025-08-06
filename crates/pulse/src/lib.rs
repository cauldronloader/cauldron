mod export_binaryninja;

use crate::export_binaryninja::export_binary_ninja;
use cauldron::prelude::*;
use libdecima_core::types::core::factory_manager::FactoryManager;
use retour::static_detour;

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

    let Some(factory) = FactoryManager::get_instance() else {
        log::error!("failed to get factory manager");
        return false;
    };

    log::info!("found {} types", factory.types.count);

    let types = factory.types.slice();
    let mut new_types = vec![];
    for ty in types {
        if !ty.value.is_null() {
            let ty = unsafe { &*ty.value };
            new_types.push(ty);
        }
    }
    export_binary_ninja(new_types).unwrap();

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
            .dependency(CauldronModDependency::new("libdecima", None, false))
            .build(),
    );

    // note(py): yes im aware this explicitly leaks
    // todo(py): maybe look into sending the ptr back after load so the rust allocator can drop it?
    Box::into_raw(info)
}
