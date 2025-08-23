use cauldron::CauldronApi;
use cauldron::log::init_mod_logger;
use cauldron::prelude::{CauldronModDependency, CauldronModInfo};
use libdecima_core::types::core::exported_symbols::{ExportedSymbolKind, ExportedSymbols};
use std::time::Duration;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn CauldronMod_Load(loader_api: *const CauldronApi) -> bool {
    let loader = unsafe { &*loader_api };
    init_mod_logger(loader).expect("libdecima: failed to initialize mod logger.");

    let mut atom_count: u32 = 0;
    let mut enum_count: u32 = 0;
    let mut class_count: u32 = 0;
    let mut struct_count: u32 = 0;
    let mut typedef_count: u32 = 0;
    let mut function_count: u32 = 0;
    let mut variable_count: u32 = 0;
    let mut container_count: u32 = 0;
    let mut reference_count: u32 = 0;
    let mut pointer_count: u32 = 0;
    let mut source_file_count: u32 = 0;

    let symbols = ExportedSymbols::get().expect("libdecima: failed to get exported symbols");
    for group in symbols.groups.as_slice() {
        let group = unsafe { &**group };
        for symbol in group.symbols.as_slice() {
            let namespace = symbol.namespace().unwrap_or_default();
            let name = symbol
                .exported_definition
                .name()
                .unwrap_or(symbol.name().unwrap());

            match symbol.kind {
                ExportedSymbolKind::Atom => {
                    atom_count += 1;
                }
                ExportedSymbolKind::Enum => {
                    enum_count += 1;
                }
                ExportedSymbolKind::Class => {
                    class_count += 1;
                }
                ExportedSymbolKind::Struct => {
                    struct_count += 1;
                }
                ExportedSymbolKind::Typedef => {
                    typedef_count += 1;
                }
                ExportedSymbolKind::Function => {
                    loader.register(
                        "libdecima/game/functions",
                        format!("{namespace}/{name}").as_str(),
                        symbol.exported_definition.address,
                    );
                    function_count += 1;
                }
                ExportedSymbolKind::Variable => {
                    loader.register(
                        "libdecima/game/variable",
                        format!("{namespace}/{name}").as_str(),
                        symbol.exported_definition.address,
                    );
                    variable_count += 1;
                }
                ExportedSymbolKind::Container => {
                    container_count += 1;
                }
                ExportedSymbolKind::Reference => {
                    reference_count += 1;
                }
                ExportedSymbolKind::Pointer => {
                    pointer_count += 1;
                }
                ExportedSymbolKind::SourceFile => {
                    source_file_count += 1;
                }
            }
        }
    }

    log::info!("Registered ExportedSymbols:");
    log::info!("  Atoms: {atom_count}");
    log::info!("  Enums: {enum_count}");
    log::info!("  Classes: {class_count}");
    log::info!("  Structs: {struct_count}");
    log::info!("  Typedefs: {typedef_count}");
    log::info!("  Functions: {function_count}");
    log::info!("  Variables: {variable_count}");
    log::info!("  Containers: {container_count}");
    log::info!("  References: {reference_count}");
    log::info!("  Pointers: {pointer_count}");
    log::info!("  Source Files: {source_file_count}");

    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn CauldronMod_Info() -> *const CauldronModInfo {
    let info = Box::new(
        CauldronModInfo::builder("libdecima", env!("CARGO_PKG_VERSION"))
            .dependency(CauldronModDependency::new("hfw", Some(">=1.5.80"), false))
            .build(),
    );

    Box::into_raw(info)
}
