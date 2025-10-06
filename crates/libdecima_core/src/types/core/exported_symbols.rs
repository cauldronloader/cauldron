use crate::types::core::rtti::RTTI;
use crate::types::p_core::array::Array;
use crate::types::p_core::ggstring::GGString;
use crate::types::p_core::hashmap::HashMap;
use crate::{assert_size, gen_with_vtbl};
use bitflags::bitflags;
use cauldron::mem::offset::Offset;
use std::ffi::{CStr, c_char, c_void};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum ExportedSymbolKind {
    Atom,
    Enum,
    Class,
    Struct,
    Typedef,
    Function,
    Variable,
    Container,
    Reference,
    Pointer,
    SourceFile,
}
assert_size!(ExportedSymbolKind, 0x1);

impl Display for ExportedSymbolKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ExportedSymbolKind::Atom => "Atom",
            ExportedSymbolKind::Enum => "Enum",
            ExportedSymbolKind::Class => "Class",
            ExportedSymbolKind::Struct => "Struct",
            ExportedSymbolKind::Typedef => "Typedef",
            ExportedSymbolKind::Function => "Function",
            ExportedSymbolKind::Variable => "Variable",
            ExportedSymbolKind::Container => "Container",
            ExportedSymbolKind::Reference => "Reference",
            ExportedSymbolKind::Pointer => "Pointer",
            ExportedSymbolKind::SourceFile => "SourceFile",
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(C)]
    pub struct ExportedSymbolSignatureFlags : u8 {
        const Unk1 = 1;
        const Unk2 = 2;
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ExportedSymbolToken {
    pub type_name: *const c_char,
    pub modifiers: *const c_char,
    pub r#type: *mut RTTI,
    pub name: *const c_char,
    pub flags: ExportedSymbolSignatureFlags,
}
assert_size!(ExportedSymbolToken, 0x28);

impl ExportedSymbolToken {
    pub fn name(&self) -> Option<String> {
        unsafe {
            if self.name.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
            }
        }
    }
    pub fn modifiers(&self) -> Option<String> {
        unsafe {
            if self.name.is_null() {
                None
            } else {
                let modifiers = CStr::from_ptr(self.modifiers)
                    .to_str()
                    .unwrap_or_default()
                    .to_owned();
                if modifiers.is_empty() {
                    None
                } else {
                    Some(modifiers)
                }
            }
        }
    }

    pub fn type_name(&self) -> Option<String> {
        unsafe {
            if self.name.is_null() {
                if !self.r#type.is_null() {
                    Some((&*self.r#type).get_symbol_name())
                } else {
                    None
                }
            } else {
                Some(
                    CStr::from_ptr(self.type_name)
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
            }
        }
    }
    pub fn as_c_type(&self) -> String {
        format!(
            "{}{}",
            self.type_name().unwrap_or(String::from("void")),
            self.modifiers().unwrap_or(String::from(" "))
        )
    }

    pub fn as_named_c_type(&self) -> String {
        format!(
            "{} /* {} */",
            self.as_c_type(),
            self.type_name().unwrap_or(String::from("void"))
        )
    }

    pub fn as_c_argument(&self, default_name: String) -> String {
        format!(
            "{}{}",
            self.as_c_type(),
            self.name().unwrap_or(default_name)
        )
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ExportedSymbolDefinition {
    pub address: *mut c_void,
    pub name: *const c_char,
    pub header_file: *const c_char,
    pub source_file: *const c_char,
    pub tokens: Array<ExportedSymbolToken>,
    pub fn_unk30: *mut c_void,
    pub fn_unk38: *mut c_void,
}
assert_size!(ExportedSymbolDefinition, 0x40);

impl ExportedSymbolDefinition {
    pub fn name(&self) -> Option<String> {
        unsafe {
            if self.name.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
            }
        }
    }
    pub fn header_file(&self) -> Option<String> {
        unsafe {
            if self.name.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(self.header_file)
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
            }
        }
    }
    pub fn source_file(&self) -> Option<String> {
        unsafe {
            if self.name.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(self.source_file)
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
            }
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(C)]
    pub struct ExportedSymbolFlags : u8 {
        const Unk1 = 1;
        const Unk2 = 2;
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ExportedSymbol {
    pub kind: ExportedSymbolKind,
    pub r#type: *const RTTI,
    pub namespace: *const c_char,
    pub name: *const c_char,
    pub size_type: *const RTTI,
    pub flags: ExportedSymbolFlags,
    pub exported_definition: ExportedSymbolDefinition,
    pub internal_definition: ExportedSymbolDefinition,
}
assert_size!(ExportedSymbol, 0xB0);

impl ExportedSymbol {
    pub fn namespace(&self) -> Option<String> {
        unsafe {
            if self.namespace.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(self.namespace)
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
            }
        }
    }

    pub fn name(&self) -> Option<String> {
        unsafe {
            if self.name.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
            }
        }
    }
}

gen_with_vtbl!(
    ExportedSymbolsGroup,
    ExportedSymbolsGroupVtbl,

    fn constructor();
    fn register_symbols();

    pub export_mask: u32,
    pub namespace: *const c_char,
    pub symbols: Array<ExportedSymbol>,
    pub dependencies: Array<*const RTTI>,
);
assert_size!(ExportedSymbolsGroup, 0x38);

#[derive(Debug)]
#[repr(C)]
pub struct ExportedSymbols {
    pub groups: Array<*mut ExportedSymbolsGroup>,
    pub dependencies1: Array<*const RTTI>,
    pub dependencies2: Array<*const RTTI>,
    pub all_symbols: HashMap<*mut ExportedSymbol, u32>,
    pub type_symbols: HashMap<GGString, *mut ExportedSymbol>,
}

impl ExportedSymbols {
    pub fn get() -> Option<&'static ExportedSymbols> {
        let ptr = Offset::from_signature("48 63 05 ? ? ? ? 4D 8B 3E")
            .unwrap()
            .as_relative(7)
            .as_ptr::<ExportedSymbols>();
        if !ptr.is_null() {
            let instance = unsafe { &*ptr };
            return Some(instance);
        }
        None
    }

    pub fn import_symbol(symbol: u32) -> Option<*mut c_void> {
        match Self::get() {
            None => None,
            Some(symbols) => {
                let Ok(importer) =
                    Offset::from_signature("48 89 5C 24 ? 57 48 83 EC 20 48 8D 7A ? 89 4C 24")
                else {
                    return None;
                };

                let importer: extern "C" fn(u32, *mut ExportedSymbols) -> *mut c_void = unsafe {
                    std::mem::transmute(
                        importer
                            .as_ptr::<extern "C" fn(u32, *mut ExportedSymbols) -> *mut c_void>(),
                    )
                };

                let ptr = importer(
                    symbol,
                    symbols as *const ExportedSymbols as *mut ExportedSymbols,
                );
                if ptr.is_null() { None } else { Some(ptr) }
            }
        }
    }

    /// 0x40506d26, fn(ptr: *mut c_void);
    pub const G_MEM_FREE: u32 = 0x40506d26;
    /// 0xda6ab4e, fn(size: u64) -> *mut c_void;
    pub const G_MEM_ALLOC: u32 = 0xda6ab4e;
}
