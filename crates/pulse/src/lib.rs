mod export_binaryninja;
mod export_symbols;
mod export_types;
mod exported_type_defs;

use crate::export_binaryninja::export_binary_ninja;
use crate::export_symbols::export_symbols;
use crate::export_types::export_types;
use cauldron::prelude::*;
use libdecima_core::types::core::factory_manager::FactoryManager;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn CauldronMod_Load(loader_api: *const CauldronApi) -> bool {
    let loader = unsafe { &*loader_api };
    init_mod_logger(loader).expect("pulse: failed to initialize mod logger.");

    let Some(factory) = FactoryManager::get_instance() else {
        log::error!("failed to get factory manager");
        return false;
    };
    let types = factory.types.slice();
    let mut new_types = vec![];
    for ty in types {
        if !ty.value.is_null() {
            let ty = unsafe { &*ty.value };
            new_types.push(ty);
        }
    }

    if export_types().is_ok() {
        log::info!("exported types");
    } else {
        log::error!("failed to export types");
    }

    if export_binary_ninja(new_types).is_ok() {
        log::info!("Binary Ninja script exported to cauldron/binary_ninja.py");
    } else {
        log::error!("Failed to export for Binary Ninja.");
    }

    if export_symbols().is_ok() {
        log::info!("Exported symbols to cauldron/symbols.csv");
    } else {
        log::error!("Failed to export symbols.");
    }

    log::info!("Pulse loaded.");

    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn CauldronMod_Info() -> *const CauldronModInfo {
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
