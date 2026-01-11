use crate::exported_types::{
    ExportedCompoundAttribute, ExportedCompoundBase, ExportedCompoundMessageHandler,
    ExportedCompoundMessageOrderEntry, ExportedCompoundMessageReadBinary,
    ExportedCompoundOrderedAttribute, ExportedContainerData, ExportedDataRef, ExportedEnumValue,
    ExportedPointerData, ExportedType, ExportedTypeRef,
};
use libdecima_core::types::core::factory_manager::FactoryManager;
use libdecima_core::types::core::rtti::{DecimaRTTI, DecimaRTTIKind, RTTIWithValues};
use libdecima_core::types::core::rtti::{RTTIWithAliases, RTTIWithName};
use std::ffi::CStr;
use std::fs::File;

pub(crate) fn export_types() -> anyhow::Result<()> {
    let mut exporting_types: Vec<ExportedType> = Vec::new();
    log::info!("exporting types");

    let types = FactoryManager::get_instance()
        .unwrap()
        .types
        .slice()
        .iter()
        .map(|x| {
            if x.value.is_null() {
                None
            } else {
                Some(unsafe { &*x.value })
            }
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<_>>();

    const SKIP_KINDS: [DecimaRTTIKind; 1] = [DecimaRTTIKind::BitSetEnum]; // note 100% on the impl of bitsets yet

    for ty in types {
        if SKIP_KINDS.contains(&ty.kind) {
            continue;
        }

        log::info!("exporting ({:?}) {}", ty.kind, ty.symbol_name());

        let exported = match ty.kind {
            DecimaRTTIKind::Atom => {
                let a = ty.as_atom_unchecked();

                ExportedType::Atom {
                    id: a.rtti_base.id,
                    factory_flags: a.rtti_base.flags.bits(),
                    ptr: ty as *const DecimaRTTI as u32,
                    size: a.size,
                    alignment: a.alignment,
                    simple: a.simple,
                    type_name: a.symbol_name(),
                    base_type: ExportedTypeRef {
                        type_name: unsafe { &*a.parent_type }.symbol_name(),
                        ptr: a.parent_type as u32,
                    },
                    func_ptr_from_string: a.func_from_string as u32,
                    func_ptr_to_string: a.func_to_string as u32,
                    func_ptr_unk30: a.func_unk30 as u32,
                    func_ptr_copy: a.func_copy as u32,
                    func_ptr_equals: a.func_equals as u32,
                    func_ptr_constructor: a.func_constructor as u32,
                    func_ptr_destructor: a.func_destructor as u32,
                    func_ptr_get_serialized_size: a.func_get_serialized_size as u32,
                    func_ptr_check_range: a.func_check_range as u32,

                    ptr_optimized_type: a.optimized_type as u32,
                }
            }
            DecimaRTTIKind::Pointer => {
                let p = ty.as_pointer_unchecked();
                let pd = unsafe { &*p.pointer_type };

                ExportedType::Pointer {
                    id: p.rtti_base.id,
                    factory_flags: p.rtti_base.flags.bits(),
                    ptr: ty as *const DecimaRTTI as u32,
                    has_pointers: p.has_pointers,
                    type_name: p.symbol_name(),
                    item_type: ExportedTypeRef {
                        type_name: unsafe { &*p.item_type }.symbol_name(),
                        ptr: p.item_type as u32,
                    },
                    pointer: ExportedPointerData {
                        type_name: pd.symbol_name(),
                        ptr: p.pointer_type as u32,
                        size: pd.size,
                        alignment: pd.alignment,

                        func_ptr_constructor: pd.func_constructor as u32,
                        func_ptr_destructor: pd.func_destructor as u32,
                        func_ptr_get: pd.func_get as u32,
                        func_ptr_set: pd.func_set as u32,
                        func_ptr_copy: pd.func_copy as u32,
                    },
                }
            }
            DecimaRTTIKind::Container => {
                let c = ty.as_container_unchecked();
                let cd = unsafe { &*c.container_type };

                ExportedType::Container {
                    id: c.rtti_base.id,
                    factory_flags: c.rtti_base.flags.bits(),
                    ptr: ty as *const DecimaRTTI as u32,

                    has_pointers: c.has_pointers,
                    type_name: c.symbol_name(),

                    item_type: ExportedTypeRef {
                        type_name: unsafe { &*c.item_type }.symbol_name(),
                        ptr: c.item_type as u32,
                    },
                    container: ExportedContainerData {
                        type_name: cd.symbol_name(),
                        ptr: c.container_type as u32,
                        size: cd.size,
                        alignment: cd.alignment,
                        simple: cd.simple,
                        associative: cd.associative,

                        func_ptr_constructor: cd.func_constructor as u32,
                        func_ptr_destructor: cd.func_deconstructor as u32,
                        func_ptr_resize: cd.func_resize as u32,
                        func_ptr_remove: cd.func_remove as u32,
                        func_ptr_length: cd.func_length as u32,
                        func_ptr_iter_start: cd.func_iterator_start as u32,
                        func_ptr_iter_end: cd.func_iterator_end as u32,
                        func_ptr_iter_next: cd.func_iterator_next as u32,
                        func_ptr_iter_deref: cd.func_iterator_deref as u32,
                        func_ptr_iter_valid: cd.func_iterator_validate as u32,
                        func_ptr_add_item: cd.func_add_item as u32,
                        func_ptr_add_empty: cd.func_add_empty as u32,
                        func_ptr_clear: cd.func_clear as u32,
                        func_ptr_to_string: cd.func_to_string as u32,
                        func_ptr_from_string: cd.func_from_string as u32,
                    },
                }
            }
            DecimaRTTIKind::Enum | DecimaRTTIKind::FlagsEnum => {
                let e = ty.as_enum_unchecked();

                ExportedType::Enum {
                    id: e.rtti_base.id,
                    factory_flags: e.rtti_base.flags.bits(),
                    ptr: ty as *const DecimaRTTI as u32,
                    size: e.size,
                    alignment: e.alignment,
                    is_flags: ty.kind == DecimaRTTIKind::FlagsEnum,
                    type_name: e.symbol_name(),
                    values: ExportedDataRef {
                        ptr: e.values as u32,
                        data: e
                            .values()
                            .iter()
                            .map(|val| ExportedEnumValue {
                                value: val.value,
                                name: val.symbol_name(),
                                aliases: val.aliases(),
                            })
                            .collect(),
                    },
                }
            }
            DecimaRTTIKind::Compound => {
                let c = ty.as_compound_unchecked();

                ExportedType::Compound {
                    id: c.rtti_base.id,
                    factory_flags: c.rtti_base.flags.bits(),
                    ptr: ty as *const DecimaRTTI as u32,

                    version: c.version,
                    size: c.size,
                    flags: c.flags,
                    type_name: c.symbol_name(),
                    next_type: if c.next_type.is_null() {
                        None
                    } else {
                        Some(ExportedTypeRef {
                            type_name: unsafe { &*c.next_type }.symbol_name(),
                            ptr: c.next_type as u32,
                        })
                    },
                    previous_type: if c.previous_type.is_null() {
                        None
                    } else {
                        Some(ExportedTypeRef {
                            type_name: unsafe { &*c.previous_type }.symbol_name(),
                            ptr: c.previous_type as u32,
                        })
                    },

                    bases: ExportedDataRef {
                        ptr: c.bases as u32,
                        data: c
                            .bases()
                            .iter()
                            .map(|base| ExportedCompoundBase {
                                base: ExportedTypeRef {
                                    type_name: unsafe { &*base.base_type }.symbol_name(),
                                    ptr: base.base_type as u32,
                                },
                                offset: base.offset,
                            })
                            .collect(),
                    },
                    attributes: ExportedDataRef {
                        ptr: c.attributes as u32,
                        data: c
                            .attributes()
                            .iter()
                            .map(|attr| ExportedCompoundAttribute {
                                attribute_type: if attr.attribute_type.is_null() {
                                    None
                                } else {
                                    Some(ExportedTypeRef {
                                        type_name: unsafe { &*attr.attribute_type }.symbol_name(),
                                        ptr: attr.attribute_type as u32,
                                    })
                                },
                                attribute_name: if attr.attribute_name.is_null() {
                                    None
                                } else {
                                    Some(unsafe {
                                        CStr::from_ptr(attr.attribute_name)
                                            .to_str()
                                            .unwrap()
                                            .to_string()
                                    })
                                },
                                offset: attr.offset,
                                flags: attr.flags.bits(),
                                range_min: if attr.range_min.is_null() {
                                    None
                                } else {
                                    Some(unsafe {
                                        CStr::from_ptr(attr.range_min).to_str().unwrap().to_string()
                                    })
                                },
                                range_max: if attr.range_max.is_null() {
                                    None
                                } else {
                                    Some(unsafe {
                                        CStr::from_ptr(attr.range_max).to_str().unwrap().to_string()
                                    })
                                },
                                func_ptr_get: attr.func_get as u32,
                                func_ptr_set: attr.func_get as u32,
                            })
                            .collect(),
                    },
                    ordered_attributes: ExportedDataRef {
                        ptr: c.ordered_attributes as u32,
                        data: c
                            .ordered_attributes()
                            .iter()
                            .map(|attr| ExportedCompoundOrderedAttribute {
                                attribute_type: if attr.attribute_type.is_null() {
                                    None
                                } else {
                                    Some(ExportedTypeRef {
                                        type_name: unsafe { &*attr.attribute_type }.symbol_name(),
                                        ptr: attr.attribute_type as u32,
                                    })
                                },
                                attribute_name: if attr.attribute_name.is_null() {
                                    None
                                } else {
                                    Some(unsafe {
                                        CStr::from_ptr(attr.attribute_name)
                                            .to_str()
                                            .unwrap()
                                            .to_string()
                                    })
                                },
                                offset: attr.offset,
                                flags: attr.flags.bits(),
                                range_min: if attr.range_min.is_null() {
                                    None
                                } else {
                                    Some(unsafe {
                                        CStr::from_ptr(attr.range_min).to_str().unwrap().to_string()
                                    })
                                },
                                range_max: if attr.range_max.is_null() {
                                    None
                                } else {
                                    Some(unsafe {
                                        CStr::from_ptr(attr.range_max).to_str().unwrap().to_string()
                                    })
                                },

                                parent: if attr.parent.is_null() {
                                    None
                                } else {
                                    Some(ExportedTypeRef {
                                        type_name: unsafe { &*attr.parent }.symbol_name(),
                                        ptr: attr.parent as u32,
                                    })
                                },
                                group: if attr.group.is_null() {
                                    None
                                } else {
                                    Some(unsafe {
                                        CStr::from_ptr(attr.group).to_str().unwrap().to_string()
                                    })
                                },

                                func_ptr_get: attr.func_get as u32,
                                func_ptr_set: attr.func_get as u32,
                            })
                            .collect(),
                    },
                    message_handlers: ExportedDataRef {
                        ptr: c.message_handlers as u32,
                        data: c
                            .message_handlers()
                            .iter()
                            .map(|handler| ExportedCompoundMessageHandler {
                                message: ExportedTypeRef {
                                    type_name: unsafe { &*handler.message }.symbol_name(),
                                    ptr: handler.message as u32,
                                },
                                func_ptr_handler: handler.handler as u32,
                            })
                            .collect(),
                    },
                    message_order_entries: ExportedDataRef {
                        ptr: c.message_order_entries as u32,
                        data: c
                            .message_order_entries()
                            .iter()
                            .map(|entry| ExportedCompoundMessageOrderEntry {
                                before: entry.before,
                                message: ExportedTypeRef {
                                    type_name: unsafe { &*entry.message }.symbol_name(),
                                    ptr: entry.message as u32,
                                },
                                compound: ExportedTypeRef {
                                    type_name: unsafe { &*entry.compound }.symbol_name(),
                                    ptr: entry.compound as u32,
                                },
                            })
                            .collect(),
                    },
                    message_read_binary: if c.message_read_binary.message.is_null() {
                        None
                    } else {
                        Some(ExportedCompoundMessageReadBinary {
                            offset: c.message_read_binary_offset,
                            handler: ExportedCompoundMessageHandler {
                                message: ExportedTypeRef {
                                    type_name: unsafe { &*c.message_read_binary.message }
                                        .symbol_name(),
                                    ptr: c.message_read_binary.message as u32,
                                },
                                func_ptr_handler: c.message_read_binary.handler as u32,
                            },
                        })
                    },

                    func_ptr_constructor: c.func_constructor as u32,
                    func_ptr_destructor: c.func_destructor as u32,
                    func_ptr_from_string: c.func_from_string as u32,
                    func_ptr_unk0: c.func_unk0 as u32,
                    func_ptr_to_string: c.func_to_string as u32,
                    func_ptr_get_symbol_group: c.func_get_symbol_group as u32,
                    ptr_optimized_type: c.optimized_type as u32,
                    unk1: c.unk1,
                }
            }
            DecimaRTTIKind::Pod => {
                let p = ty.as_pod_unchecked();
                ExportedType::Pod {
                    id: p.rtti_base.id,
                    factory_flags: p.rtti_base.flags.bits(),
                    ptr: ty as *const DecimaRTTI as u32,
                    size: p.size,
                }
            }
            DecimaRTTIKind::BitSetEnum => {
                let e = ty.as_bitset_enum_unchecked();
                ExportedType::BitSetEnum {
                    ptr: ty as *const DecimaRTTI as u32,
                    type_name: e.symbol_name(),
                }
            }
        };

        exporting_types.push(exported.clone());
    }

    let mut file = File::create("cauldron/types.json")?;
    if cfg!(debug_assertions) {
        serde_json::to_writer_pretty(&mut file, &exporting_types)?;
    } else {
        serde_json::to_writer(&mut file, &exporting_types)?;
    }

    Ok(())
}
