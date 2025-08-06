use cauldron::CauldronApi;
use cauldron::log::init_mod_logger;
use cauldron::prelude::{CauldronModDependency, CauldronModInfo};
use libdecima_core::types::core::exported_symbols::{ExportedSymbolKind, ExportedSymbols};
use std::time::Duration;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cauldron_mod__load(loader_api: *const CauldronApi) -> bool {
    let loader = unsafe { &*loader_api };
    init_mod_logger(loader).expect("libdecima: failed to initialize mod logger.");

    // just wait a couple seconds to make sure we're loading after symbol registration has complete
    // todo(py): hook something and wait for it to finish rather than just thread sleeping
    std::thread::sleep(Duration::from_secs(2));

    let mut function_count: u32 = 0;
    let mut variable_count: u32 = 0;

    let symbols = ExportedSymbols::get().expect("libdecima: failed to get exported symbols");
    for group in symbols.groups.as_slice() {
        let group = unsafe { &**group };
        for symbol in group.symbols.as_slice() {
            let namespace = symbol.namespace().unwrap_or_default();
            let name = symbol.language[0].name().unwrap_or(symbol.name().unwrap());

            match symbol.kind {
                ExportedSymbolKind::Function => {
                    loader.register(
                        "libdecima/game/functions",
                        format!("{namespace}/{name}").as_str(),
                        symbol.language[0].address,
                    );
                    function_count += 1;
                }
                ExportedSymbolKind::Variable => {
                    loader.register(
                        "libdecima/game/variable",
                        format!("{namespace}/{name}").as_str(),
                        symbol.language[0].address,
                    );
                    variable_count += 1;
                }
                _ => {}
            }
        }
    }

    log::info!("Registered ExportedSymbols:");
    log::info!("\tFunctions: {function_count}");
    log::info!("\tVariables: {variable_count}");

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cauldron_mod__info() -> *const CauldronModInfo {
    let info = Box::new(
        CauldronModInfo::builder("libdecima", env!("CARGO_PKG_VERSION"))
            .dependency(CauldronModDependency::new("hfw", Some(">=1.5.80"), false))
            .build(),
    );

    Box::into_raw(info)
}
