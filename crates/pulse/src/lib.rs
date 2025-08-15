mod export_binaryninja;
mod export_symbols;
mod export_types;
mod exported_type_defs;

use crate::export_binaryninja::export_binary_ninja;
use crate::export_symbols::export_symbols;
use crate::export_types::export_types;
use cauldron::mem::offset::Offset;
use cauldron::prelude::*;
use libdecima_core::types::core::exported_symbols::ExportedSymbols;
use libdecima_core::types::core::factory_manager::FactoryManager;
use libdecima_core::types::core::rtti::RTTI;
use libdecima_core::types::core::rtti_object::RTTIObject;
use libdecima_core::types::p_core::array::Array;
use libdecima_core::types::p_core::ggstring::GGString;
use libdecima_core::types::p_core::gguuid::GGUUID;
use retour::static_detour;
use std::ffi::{CStr, CString, c_char, c_void};
use std::ptr::null_mut;
use std::time::Duration;

static_detour! {
    static Engine_ImportType: unsafe extern "C" fn(u32, *mut ExportedSymbols) -> *mut c_void;
}

#[unsafe(no_mangle)]
unsafe extern "C" fn graph_value_impl(c_str: *const c_char, user_data: u64) {
    if !c_str.is_null() {
        let text = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_owned() };

        log::info!("graph_value: {text} {user_data}");
    }
}

#[allow(non_snake_case)]
unsafe extern "C" fn NodeGraph_Alert_impl(alert: *const c_char, unk: bool) {
    let text = unsafe { CStr::from_ptr(alert).to_str().unwrap().to_owned() };
    log::info!("NodeGraph::Alert({text:?}, {unk})");
}

#[allow(non_snake_case)]
unsafe extern "C" fn NodeGraph_AlertWithName_impl(
    text0: *const c_char,
    text1: *const c_char,
    text2: *const c_char,
    unk3: bool,
) {
    let text0 = unsafe { CStr::from_ptr(text0).to_str().unwrap().to_owned() };
    let text1 = unsafe { CStr::from_ptr(text1).to_str().unwrap().to_owned() };
    let text2 = unsafe { CStr::from_ptr(text2).to_str().unwrap().to_owned() };
    log::info!("NodeGraph::AlertWithName({text0:?}, {text1:?}, {text2:?}, {unk3})");
}

#[allow(non_snake_case)]
unsafe extern "C" fn NodeGraph_Trace_impl(uuid: GGUUID, text: *const c_char) {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_owned() };
    log::info!("NodeGraph::Trace({uuid:?}, {text:?})");
}

#[allow(non_snake_case)]
unsafe extern "C" fn NodeGraph_Assert_impl(
    uuid: GGUUID,
    text: *const c_char,
    text2: *const c_char,
) {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap().to_owned() };
    let text2 = unsafe { CStr::from_ptr(text2).to_str().unwrap().to_owned() };
    log::info!("NodeGraph::Assert({uuid:?}, {text:?}, {text2:?})");
}

#[allow(non_snake_case)]
unsafe extern "C" fn NodeGraph_MarkStartNode_impl(
    name: *const c_char,
    node_id: u64,
    unk0: i32,
    unk1: *mut c_void,
    unk2: *mut c_void,
) {
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap().to_owned() };
    log::info!("NodeGraph::MarkBeginNode({name:?}, {node_id}, {unk0}, {unk1:p}, {unk2:p})");
}

#[allow(non_snake_case)]
unsafe extern "C" fn NodeGraph_MarkEndNode_impl(
    node_id: u64,
    unk0: i32,
    unk1: *mut c_void,
    unk2: *mut c_void,
) {
    log::info!("NodeGraph::MarkEndNode({node_id}, {unk0}, {unk1:p}, {unk2:p})");
}

#[allow(non_snake_case)]
unsafe extern "C" fn NodeGraph_IsProfiling_impl() -> bool {
    true
}

#[allow(non_snake_case)]
unsafe extern "C" fn GCore_sDrawText(text: *mut GGString) {
    let text = unsafe { CStr::from_ptr((&*text).data).to_str().unwrap().to_owned() };
    log::info!("GCore::sDrawText({text:?}, ...)");
}

#[repr(C)]
#[derive(Debug)]
struct LocalizedTextResource {
    pub vftable: *mut LocalizedTextResourceVTable,
    pub uuid: GGUUID,
    pub ref_count: u32,

    pub text: *const c_char,
    pub len: u16,
    pub entry: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
struct LocalizedTextResourceVTable {
    _0: *mut c_void,
    _1: *mut c_void,
    _2: *mut c_void,
    _3: *mut c_void,
    set_text: extern "C" fn(*mut LocalizedTextResource, *const c_char) -> *const c_char,
}

impl LocalizedTextResource {
    pub fn new() -> *mut LocalizedTextResource {
        let rtti = RTTI::find_by_name("LocalizedTextResource")
            .as_compound()
            .unwrap();
        let ctor = rtti.fn_constructor;
        let mut ptr: *mut LocalizedTextResource = std::ptr::null_mut();
        unsafe {
            (*ctor)(rtti as *const _ as *mut _, ptr as *mut c_void) as *mut LocalizedTextResource
        }
    }

    pub fn set_text(&mut self, text: &str) {
        let fun = unsafe { &*self.vftable }.set_text;
        let text = unsafe { CString::new(text).unwrap() };
        fun(self as *mut LocalizedTextResource, text.as_ptr());
    }
}

#[allow(non_snake_case)]
unsafe extern "C" fn MenuSymbols_OpenPage_impl(
    menu: *mut c_void,
    uuid: *mut GGUUID,
    params: *mut Array<u8>,
    unk: bool,
) {
    let gguuid = unsafe { &*uuid };
    log::info!("MenuSymbols::OpenPage({menu:p}, {gguuid}, {params:p}, {unk})");

    let orig: extern "C" fn(*mut c_void, *mut GGUUID, *mut Array<u8>, bool) = unsafe {
        std::mem::transmute(Engine_ImportType.call(
            0x1cd20bcd,
            ExportedSymbols::get().unwrap() as *const ExportedSymbols as *mut ExportedSymbols,
        ))
    };
    orig(menu, uuid, params, unk);

    let title_screen = GGUUID {
        data0: 113,
        data1: 136,
        data2: 8,
        data3: 26,
        data4: 117,
        data5: 114,
        data6: 72,
        data7: 75,
        data8: 143,
        data9: 6,
        data10: 159,
        data11: 225,
        data12: 162,
        data13: 116,
        data14: 149,
        data15: 166,
    };
    if *gguuid == title_screen {
        log::info!("title screen hit, showing toast");
        let show_toast: extern "C" fn(*mut c_void, *mut LocalizedTextResource, i32, i8) = unsafe {
            std::mem::transmute(Engine_ImportType.call(
                0x7296053b,
                ExportedSymbols::get().unwrap() as *const ExportedSymbols as *mut ExportedSymbols,
            ))
        };
        // let text = format!("<icon name=triangle> <em>Cauldron</em> Loaded.<br>v{}", env!("CARGO_PKG_VERSION"));
        // let str = CString::new().unwrap();

        let localized_text = unsafe { &mut *LocalizedTextResource::new() };
        // localized_text.set_text("<icon name=triangle> <em>Cauldron</em> Loaded.");

        show_toast(menu, localized_text as *mut LocalizedTextResource, 5, 2);
        // std::mem::forget(localized_text);
    }
}

#[allow(non_snake_case)]
unsafe extern "C" fn MenuSymbols_ShowToast_impl(
    menu: *mut c_void,
    text: *mut LocalizedTextResource,
    duration: i32,
    priority: i8,
) {
    let show_toast: extern "C" fn(*mut c_void, *mut LocalizedTextResource, i32, i8) = unsafe {
        std::mem::transmute(Engine_ImportType.call(
            0x7296053b,
            ExportedSymbols::get().unwrap() as *const ExportedSymbols as *mut ExportedSymbols,
        ))
    };
    log::info!("MenuSymbols::ShowToast({menu:p}, {text:p}, {duration}, {priority})");
    let mut text = unsafe { &mut *text };
    log::info!("{text:?}");
    log::info!("{}", CStr::from_ptr(text.text).to_str().unwrap());
    text.set_text("<icon name=triangle> <em>Cauldron</em> Patch.");

    show_toast(menu, text as *const _ as *mut _, duration, priority);
}

fn engine_import_type_impl(symbol_hash: u32, symbols: *mut ExportedSymbols) -> *mut c_void {
    match symbol_hash {
        // 0x1bdc3e56 => graph_value_impl as *mut c_void,

        // 0x6b96f9ac => NodeGraph_Alert_impl as *mut c_void,
        // 0x46a57fae => NodeGraph_AlertWithName_impl as *mut c_void,
        // 0x588560e0 => NodeGraph_Trace_impl as *mut c_void,
        // 0x5bc3c297 => NodeGraph_Assert_impl as *mut c_void,
        // 0x689d3904 => NodeGraph_MarkStartNode_impl as *mut c_void,
        // 0x7d499b0c => NodeGraph_MarkEndNode_impl as *mut c_void,
        // 0x7682c6c7 /* gIsRuntimeDebug */ | 0x4b1f33dc => NodeGraph_IsProfiling_impl as *mut c_void,

        // 0x1cd20bcd => {
        //     log::info!("hit for 0x1cd20bcd (MenuSymbols_OpenPage)");
        //     MenuSymbols_OpenPage_impl as *mut c_void
        // },
        // 0x7296053b => {
        //     log::info!("hit for 0x7296053b (MenuSymbols_ShowToast");
        //     MenuSymbols_ShowToast_impl as *mut c_void
        // }

        // 0x79f0aa97 => GCore_sDrawText as *mut c_void,
        hash => unsafe { Engine_ImportType.call(hash, symbols) },
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cauldron_mod__load(loader_api: *const CauldronApi) -> bool {
    let loader = unsafe { &*loader_api };
    init_mod_logger(loader).expect("pulse: failed to initialize mod logger.");

    if let Ok(offset) = Offset::from_signature("48 89 5C 24 ? 57 48 83 EC ? 48 8D 7A ? 89 4C 24 ?")
    {
        unsafe {
            Engine_ImportType
                .initialize(
                    std::mem::transmute(offset.as_ptr::<*mut c_void>()),
                    engine_import_type_impl,
                )
                .unwrap()
                .enable()
                .unwrap()
        };
    }

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
