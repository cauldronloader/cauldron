use cauldron::CauldronModInfo;
use cauldron::prelude::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cauldron_mod__load(loader_api: *const CauldronApi) -> bool {
    let loader = unsafe { &*loader_api };
    init_mod_logger(loader).expect("pulse: failed to initialize mod logger.");

    log::info!("Pulse loaded.");

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cauldron_mod__info() -> *const CauldronModInfo {
    let info = CauldronModInfo::new("Pulse".into(), 0, Some(env!("CARGO_PKG_AUTHORS").into()));

    Box::into_raw(Box::new(info)) as *const CauldronModInfo
}
