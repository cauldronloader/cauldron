use crate::exported_type_defs::{ExportedEnumValue, ExportedType};
use libdecima_core::types::core::factory_manager::FactoryManager;
use libdecima_core::types::core::rtti::{NamedRTTI, RTTIKind};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::{CStr, c_char};
use std::fs::File;
use std::io::Write as _;

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
                log::warn!("null type pointer in factory manager.");
                None
            } else {
                Some(unsafe { &*x.value })
            }
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<_>>();

    for ty in types {
        if let Some(atom) = ty.as_atom() {
            exporting_types.push(ExportedType::Atom {
                id: atom.base.id,
                factory_flags: atom.base.factory_flags.bits(),
                size: atom.size,
                alignment: atom.alignment,
                simple: atom.simple,
                type_name: atom.name(),
                base_type: unsafe { &*atom.base_type }.name(),
            });
        } else if let Some(r#enum) = ty.as_enum() {
            exporting_types.push(ExportedType::Enum {
                id: r#enum.base.id,
                factory_flags: r#enum.base.factory_flags.bits(),
                size: r#enum.size,
                alignment: r#enum.alignment,
                type_name: r#enum.name(),
                values: r#enum
                    .values()
                    .iter()
                    .map(|v| ExportedEnumValue {
                        value: v.value,
                        name: unsafe { CStr::from_ptr(v.name).to_str().unwrap().to_owned() },
                        aliases: v
                            .aliases
                            .iter()
                            .map(|p| if p.is_null() { None } else { Some(p) })
                            .filter(|p| p.is_some())
                            .map(|p| unsafe {
                                CStr::from_ptr(*p.unwrap()).to_str().unwrap().to_owned()
                            })
                            .collect::<Vec<_>>(),
                    })
                    .collect::<Vec<_>>(),
            });
        }
    }

    exporting_types.sort_by(|one, two| match one {
        ExportedType::Atom {
            type_name,
            base_type,
            ..
        } => {
            let one_type_name = type_name;
            let one_base_type = base_type;

            match two {
                ExportedType::Atom {
                    type_name,
                    base_type,
                    ..
                } => {
                    if one_type_name == type_name {
                        Ordering::Equal
                    } else if one_type_name == base_type {
                        Ordering::Greater
                    } else if one_base_type == type_name {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
                _ => Ordering::Less,
            }
        }
        ExportedType::Enum { .. } => match two {
            ExportedType::Atom { .. } => Ordering::Greater,
            ExportedType::Enum { .. } => Ordering::Equal,
        },
    });

    exporting_types.dedup();

    let mut file = File::create("types.json")?;
    if cfg!(debug_assertions) {
        serde_json::to_writer_pretty(&mut file, &exporting_types)?;
    } else {
        serde_json::to_writer(&mut file, &exporting_types)?;
    }

    Ok(())
}
