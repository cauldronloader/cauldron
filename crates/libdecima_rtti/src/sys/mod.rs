mod impls;

use crate::string::DString;
use std::ffi::{c_char, c_void};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecimaRTTIKind {
    Atom,
    Pointer,
    Container,
    Enum,
    Compound,
    FlagsEnum,
    Pod,
    BitSetEnum,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DecimaRTTIFlags: u8 {
        const SourceRTTIFactory = 0x2;
        const SourceFactoryManager = 0x4;
    }
}

/// While Rust doesn't have a concept of type inheritance, C++ does and Decima is a C++ engine.
/// This is used as a base for all other Decima RTTI type structs.
/// In any Decima RTTI struct that "extends" this, it is accessible as the first field which is named `rtti_base`.
#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTI {
    pub id: u32,
    pub kind: DecimaRTTIKind,
    pub flags: DecimaRTTIFlags,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTIStringSpan {
    pub data: *const c_char,
    pub length: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTIIterator {
    pub value_type: *mut DecimaRTTI,
    pub iterator: *mut c_void,
    pub user_data: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Hash)]
pub struct DecimaRTTIAtom {
    pub rtti_base: DecimaRTTI,

    pub size: u16,
    pub alignment: u8,
    pub simple: bool,
    pub type_name: *const c_char,
    pub parent_type: *const DecimaRTTIAtom,

    pub func_from_string:
        extern "C" fn(value: &DecimaRTTIStringSpan, out_object: *mut c_void) -> bool,
    pub func_to_string: extern "C" fn(value: *const c_void, out_string: &mut DString) -> bool,
    pub func_unk30: extern "C" fn(),
    pub func_copy: extern "C" fn(from: *mut c_void, to: *mut c_void),
    pub func_equals: extern "C" fn(first: *const c_void, second: *const c_void) -> bool,
    pub func_constructor:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> *mut c_void,
    pub func_destructor: extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> *mut c_void,
    pub func_serialize:
        extern "C" fn(in_object: *mut c_void, out_object: *mut c_void, swap_endian: bool) -> bool,
    pub func_deserialize: extern "C" fn(in_object: *mut c_void, out_object: *mut c_void) -> bool,
    pub func_get_serialized_size: extern "C" fn(in_object: *mut c_void) -> i32,
    pub func_check_range:
        extern "C" fn(object: *mut c_void, min: *const c_char, max: *const c_char) -> bool,

    pub optimized_type: *const DecimaRTTI,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Hash)]
pub struct DecimaRTTIPointerData {
    pub type_name: *const c_char,
    pub size: u16,
    pub alignment: u8,

    pub func_constructor:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> *mut c_void,
    pub func_destructor: extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> *mut c_void,
    pub func_get: extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> *mut c_void,
    pub func_set: extern "C" fn(
        rtti: *const DecimaRTTI,
        object: *mut *mut c_void,
        value: *mut c_void,
    ) -> bool,
    pub func_copy: extern "C" fn(from: *mut *mut c_void, to: *mut *mut c_void) -> bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Hash)]
pub struct DecimaRTTIContainerData {
    pub type_name: *const c_char,

    pub size: u16,
    pub alignment: u8,
    pub simple: bool,
    pub associative: bool,

    pub func_constructor:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> *mut c_void,
    pub func_deconstructor:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> *mut c_void,
    pub func_resize:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void, size: i32, unk: bool) -> bool,
    pub _pad0: [u8; 0x8],
    pub func_remove:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void, index: i32) -> bool,
    pub func_length: extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> i32,
    pub func_iterator_start:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> DecimaRTTIIterator,
    pub func_iterator_end:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> DecimaRTTIIterator,
    pub func_iterator_next: extern "C" fn(iterator: &DecimaRTTIIterator),
    pub func_iterator_deref: extern "C" fn(iterator: &DecimaRTTIIterator) -> *mut c_void,
    pub func_iterator_validate: extern "C" fn(iterator: &DecimaRTTIIterator) -> bool,
    pub _pad1: [u8; 0x10],
    pub func_add_item: extern "C" fn(
        rtti: *const DecimaRTTI,
        object: *mut c_void,
        item: *mut c_void,
    ) -> DecimaRTTIIterator,
    pub func_add_empty:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> DecimaRTTIIterator,
    pub func_clear: extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> bool,
    pub func_to_string:
        extern "C" fn(object: *mut c_void, rtti: *const DecimaRTTI, string: &mut DString) -> bool,
    pub func_from_string: extern "C" fn(
        string: &DecimaRTTIStringSpan,
        rtti: *const DecimaRTTI,
        object: *mut c_void,
    ) -> bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTIContainer {
    pub rtti_base: DecimaRTTI,
    pub has_pointers: bool,
    pub item_type: *mut DecimaRTTI,
    pub container_type: *mut DecimaRTTIContainerData,
    pub type_name: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTIPointer {
    pub rtti_base: DecimaRTTI,
    pub has_pointers: bool,
    pub item_type: *mut DecimaRTTI,
    pub pointer_type: *mut DecimaRTTIPointerData,
    pub type_name: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTIEnumValue {
    pub value: i32,
    pub name: *const c_char,
    pub aliases: [*const c_char; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTIEnum {
    pub rtti_base: DecimaRTTI,

    pub size: u8,
    pub alignment: u8,
    pub values_length: u16,
    pub type_name: *const c_char,
    pub values: *const DecimaRTTIEnumValue,

    pub optimized_type: *const DecimaRTTI,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTICompoundBase {
    pub base_type: *mut DecimaRTTI,
    pub offset: u32,
}

bitflags::bitflags! {
    // might not be entirely accurate for modern Decima, based on ERTTIAttributeFlags from the KZ2 leak.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DecimaRTTICompoundAttributeFlags : u16 {
        const Normal = 0x0;
        const DontSerializeText = 0x1;
        const DontSerializeBinary = 0x2;
        const Hidden = 0x4;
        const Private = 0x7;
        const ReadOnly = 0x10;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTICompoundAttribute {
    pub attribute_type: *mut DecimaRTTI,
    pub offset: u16,
    pub flags: DecimaRTTICompoundAttributeFlags,
    pub attribute_name: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTICompoundOrderedAttribute {
    pub attribute_type: *mut DecimaRTTI,
    pub offset: u16,
    pub flags: DecimaRTTICompoundAttributeFlags,
    pub attribute_name: *const c_char,

    pub parent: *const DecimaRTTI,
    pub group: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Hash)]
pub struct DecimaRTTICompoundMessageHandler {
    pub message: *mut DecimaRTTI,
    pub handler: extern "C" fn(object: *mut c_void, message: *mut c_void),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Hash)]
pub struct DecimaRTTICompoundMessageOrderEntry {
    pub before: bool,
    pub message: *mut DecimaRTTI,
    pub compound: *mut DecimaRTTI,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Hash)]
pub struct DecimaRTTICompound {
    pub rtti_base: DecimaRTTI,

    pub bases_length: u8,
    pub attributes_length: u8,
    pub message_handlers_length: u8,
    pub message_order_entries_length: u8,
    pub _pad0: u8,

    pub version: u16,
    pub size: u32,
    pub flags: u16,

    pub func_constructor:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> *mut c_void,
    pub func_deconstructor:
        extern "C" fn(rtti: *const DecimaRTTI, object: *mut c_void) -> *mut c_void,
    pub func_from_string: extern "C" fn(object: *mut c_void, &mut DString) -> bool,
    pub func_unk0: extern "C" fn(),
    pub func_to_string: extern "C" fn(object: *mut c_void, &mut DString) -> bool,

    pub type_name: *const c_char,
    pub next_type: *const DecimaRTTI,
    pub previous_type: *const DecimaRTTI,

    pub bases: *const DecimaRTTICompoundBase,
    pub attributes: *const DecimaRTTICompoundAttribute,
    pub message_handlers: *const DecimaRTTICompoundMessageHandler,
    pub message_order_entries: *const DecimaRTTICompoundMessageOrderEntry,

    pub func_get_symbol_group: extern "C" fn() -> *mut DecimaRTTI,
    pub optimized_type: *const DecimaRTTI,

    pub ordered_attributes: *const DecimaRTTICompoundOrderedAttribute,
    pub ordered_attributes_length: u32,
    pub message_read_binary: DecimaRTTICompoundMessageHandler,
    pub message_read_binary_offset: u32,

    pub unk1: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTIPod {
    pub rtti_base: DecimaRTTI,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimaRTTIBitSetEnum {
    pub rtti_type: *mut DecimaRTTI,
    pub type_name: *const c_char,
}
