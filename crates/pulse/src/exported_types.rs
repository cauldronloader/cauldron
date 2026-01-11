use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum ExportedType {
    Atom {
        id: u32,
        factory_flags: u8,
        ptr: u32,

        size: u16,
        alignment: u8,
        simple: bool,
        type_name: String,
        base_type: ExportedTypeRef,

        func_ptr_from_string: u32,
        func_ptr_to_string: u32,
        func_ptr_unk30: u32,
        func_ptr_copy: u32,
        func_ptr_equals: u32,
        func_ptr_constructor: u32,
        func_ptr_destructor: u32,
        func_ptr_get_serialized_size: u32,
        func_ptr_check_range: u32,

        ptr_optimized_type: u32,
    },
    Pointer {
        id: u32,
        factory_flags: u8,
        ptr: u32,

        has_pointers: bool, // todo(py): probably not needed
        type_name: String,

        item_type: ExportedTypeRef,
        pointer: ExportedPointerData,
    },
    Container {
        id: u32,
        factory_flags: u8,
        ptr: u32,

        has_pointers: bool,
        type_name: String,

        item_type: ExportedTypeRef,
        container: ExportedContainerData,
    },
    Enum {
        id: u32,
        factory_flags: u8,
        ptr: u32,

        size: u8,
        alignment: u8,
        is_flags: bool,
        type_name: String,
        values: ExportedDataRef<Vec<ExportedEnumValue>>,
    },
    Compound {
        id: u32,
        factory_flags: u8,
        ptr: u32,

        version: u16,
        size: u32,
        flags: u16,
        type_name: String,
        next_type: Option<ExportedTypeRef>,
        previous_type: Option<ExportedTypeRef>,

        bases: ExportedDataRef<Vec<ExportedCompoundBase>>,
        attributes: ExportedDataRef<Vec<ExportedCompoundAttribute>>,
        ordered_attributes: ExportedDataRef<Vec<ExportedCompoundOrderedAttribute>>,
        message_handlers: ExportedDataRef<Vec<ExportedCompoundMessageHandler>>,
        message_order_entries: ExportedDataRef<Vec<ExportedCompoundMessageOrderEntry>>,

        message_read_binary: Option<ExportedCompoundMessageReadBinary>,

        func_ptr_constructor: u32,
        func_ptr_destructor: u32,
        func_ptr_from_string: u32,
        func_ptr_unk0: u32,
        func_ptr_to_string: u32,
        func_ptr_get_symbol_group: u32,
        ptr_optimized_type: u32,

        unk1: u32,
    },
    Pod {
        id: u32,
        factory_flags: u8,
        ptr: u32,
        size: u32,
    },
    BitSetEnum {
        ptr: u32,
        type_name: String,
    },
}

/// Not an actual type in Decima, used to reference types in the exported json.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedTypeRef {
    pub type_name: String,
    pub ptr: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedDataRef<T> {
    pub ptr: u32,
    pub data: T,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedEnumValue {
    pub value: i32,
    pub name: String,
    pub aliases: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedPointerData {
    pub type_name: String,
    pub ptr: u32,
    pub size: u16,
    pub alignment: u8,

    pub func_ptr_constructor: u32,
    pub func_ptr_destructor: u32,
    pub func_ptr_get: u32,
    pub func_ptr_set: u32,
    pub func_ptr_copy: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedContainerData {
    pub type_name: String,
    pub ptr: u32,
    pub size: u16,
    pub alignment: u8,
    pub simple: bool,
    pub associative: bool,

    pub func_ptr_constructor: u32,
    pub func_ptr_destructor: u32,
    pub func_ptr_resize: u32,
    pub func_ptr_remove: u32,
    pub func_ptr_length: u32,
    pub func_ptr_iter_start: u32,
    pub func_ptr_iter_end: u32,
    pub func_ptr_iter_next: u32,
    pub func_ptr_iter_deref: u32,
    pub func_ptr_iter_valid: u32,
    pub func_ptr_add_item: u32,
    pub func_ptr_add_empty: u32,
    pub func_ptr_clear: u32,
    pub func_ptr_to_string: u32,
    pub func_ptr_from_string: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedCompoundBase {
    pub base: ExportedTypeRef,
    pub offset: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedCompoundAttribute {
    pub attribute_type: Option<ExportedTypeRef>,
    pub attribute_name: Option<String>,
    pub offset: u16,
    pub flags: u16,
    pub range_min: Option<String>,
    pub range_max: Option<String>,
    pub func_ptr_get: u32,
    pub func_ptr_set: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedCompoundOrderedAttribute {
    pub attribute_type: Option<ExportedTypeRef>,
    pub attribute_name: Option<String>,
    pub offset: u16,
    pub flags: u16,
    pub range_min: Option<String>,
    pub range_max: Option<String>,

    pub parent: Option<ExportedTypeRef>,
    pub group: Option<String>,

    pub func_ptr_get: u32,
    pub func_ptr_set: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedCompoundMessageHandler {
    pub message: ExportedTypeRef,
    pub func_ptr_handler: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedCompoundMessageOrderEntry {
    pub before: bool,
    pub message: ExportedTypeRef,
    pub compound: ExportedTypeRef,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedCompoundMessageReadBinary {
    pub offset: u32,
    pub handler: ExportedCompoundMessageHandler,
}
