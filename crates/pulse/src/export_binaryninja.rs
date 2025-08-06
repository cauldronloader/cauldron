use libdecima_core::types::core::exported_symbols::{
    ExportedSymbolKind, ExportedSymbols, ExportedSymbolsGroup,
};
use libdecima_core::types::core::rtti::{RTTI, RTTIContainerData, RTTIKind};
use std::collections::HashMap;
use std::ffi::{CStr, c_void};
use std::fs::File;
use std::io::Write as _;

pub fn export_binary_ninja(types: Vec<&RTTI>) -> anyhow::Result<()> {
    let mut output_file = File::create("cauldron/binary_ninja.py")?;

    writeln!(output_file, "log = bv.create_logger(\"decima\")")?;

    let mut containers = Vec::new();
    for r#type in &types {
        export_type(&mut output_file, r#type, &mut containers)?;
    }

    writeln!(output_file, "bv.remove_tag_type(\"decima\")")?;
    writeln!(output_file, "bv.create_tag_type(\"decima\", \"⚙️\")")?;
    // todo(py):
    let mut tagged_symbols: HashMap<*const c_void, Vec<String>> = HashMap::new();
    let symbols = ExportedSymbols::get().unwrap();
    for group in symbols.groups.as_slice() {
        let group = unsafe { &*(*group) };
        export_symbols_group(&mut tagged_symbols, group)?;
    }

    // normally in bv you'd add the tag directly to the Function, but since not all functions are discovered we need to add them to addresses instead.
    for (ptr, symbols) in tagged_symbols {
        writeln!(
            output_file,
            "bv.add_tag({ptr:p}, \"decima\", \"Decima Exported: \\n{}\")",
            symbols.join("\\n")
        )?;
    }

    writeln!(output_file, "log.log_info(\"done\")")?;

    Ok(())
}

fn bn_kind_name(kind: &RTTIKind) -> String {
    match kind {
        RTTIKind::Atom => "RTTIAtom",
        RTTIKind::Pointer => "RTTIPointer",
        RTTIKind::Container => "RTTIContainer",
        RTTIKind::Enum | RTTIKind::EnumFlags => "RTTIEnum",
        RTTIKind::Compound => "RTTICompound",
        RTTIKind::POD => "RTTIPod",
        RTTIKind::EnumBitSet => "RTTIBitSet",
    }
    .to_string()
}

fn bn_symbol_name(rtti: &RTTI) -> String {
    rtti.get_symbol_name().replace("<", "__").replace(">", "")
}

fn export_type(
    file: &mut File,
    rtti: &RTTI,
    containers: &mut Vec<*const RTTIContainerData>,
) -> anyhow::Result<()> {
    let kind_str = bn_kind_name(&rtti.kind);
    let type_str = bn_symbol_name(&rtti);

    writeln!(file, "# {type_str} ({kind_str})")?;
    writeln!(
        file,
        "bv.define_data_var({rtti:p}, \"{kind_str}\", \"RTTI_{type_str}\")"
    )?;

    if let Some(compound) = rtti.as_compound() {
        let bases_len = compound.bases_len as usize;
        let bases = compound.bases;
        if !bases.is_null() {
            writeln!(
                file,
                "bv.define_data_var({bases:p}, ArrayType.create(bv.types[\"RTTICompound__Base\"], {bases_len}),\"RTTI_{type_str}__bases\")"
            )?;
        }

        let attrs_len = compound.attributes_len as usize;
        let attrs = compound.attributes;
        if !attrs.is_null() {
            writeln!(
                file,
                "bv.define_data_var({attrs:p}, ArrayType.create(bv.types[\"RTTICompound__Attribute\"], {attrs_len}), \"RTTI_{type_str}__attributes\")"
            )?;
        }

        let msg_handlers_len = compound.message_handlers_len as usize;
        let msg_handlers = compound.message_handlers;
        if !msg_handlers.is_null() {
            writeln!(
                file,
                "bv.define_data_var({msg_handlers:p}, ArrayType.create(bv.types[\"RTTICompound__MessageHandler\"], {msg_handlers_len}), \"RTTI_{type_str}__message_handlers\")"
            )?;
        }

        let msg_order_entries_len = compound.message_order_entries_len as usize;
        let msg_order_entries = compound.message_order_entries;
        if !msg_order_entries.is_null() {
            writeln!(
                file,
                "bv.define_data_var({msg_order_entries:p}, ArrayType.create(bv.types[\"RTTICompound__MessageOrderEntry\"], {msg_order_entries_len}), \"RTTI_{type_str}__message_order_entries\")"
            )?;
        }
    }

    if let Some(r#enum) = rtti.as_enum() {
        let values = r#enum.values;
        let values_len = r#enum.values_len as usize;
        if !values.is_null() {
            writeln!(
                file,
                "bv.define_data_var({values:p}, ArrayType.create(bv.types[\"RTTIEnum__Value\"], {values_len}), \"RTTI_{type_str}__values\")"
            )?;
        }
    }

    if let Some(container) = rtti.as_container() {
        if !unsafe { containers.contains(&std::mem::transmute(container.container_type)) } {
            unsafe { containers.push(std::mem::transmute(container.container_type)) };
            let data = container.container_type;

            if container.base.kind == RTTIKind::Pointer {
                writeln!(
                    file,
                    "bv.define_data_var({data:p}, \"RTTIPointer__Data\", \"RTTI_{type_str}__pointer\")"
                )?;
            } else {
                writeln!(
                    file,
                    "bv.define_data_var({data:p}, \"RTTIContainer__Data\", \"RTTI_{type_str}__container\")"
                )?;
            }
        }
    }

    Ok(())
}

fn export_symbols_group(
    tags: &mut HashMap<*const c_void, Vec<String>>,
    group: &ExportedSymbolsGroup,
) -> anyhow::Result<()> {
    for symbol in group.symbols.as_slice() {
        let export_name = {
            let name = symbol.language[0].name().unwrap_or(symbol.name().unwrap());
            if let Some(namespace) = symbol.namespace() {
                format!("{namespace}::{name}")
            } else {
                name
            }
        };
        match symbol.kind {
            ExportedSymbolKind::Variable => {
                let signatures = symbol.language[0].signature.as_slice();
                tags.entry(symbol.language[0].address)
                    .or_default()
                    .push(format!("{}{export_name}", signatures[0].as_c_type()))
            }
            ExportedSymbolKind::Function => {
                let signatures = symbol.language[0].signature.as_slice();
                let mut signature = format!("{}{export_name}(", signatures[0].as_c_type());
                for (i, argument) in signatures.iter().enumerate() {
                    if i == 0 {
                        // index 0 is the function return type
                        continue;
                    }

                    if i != 1 {
                        signature.push_str(", ");
                    }

                    signature.push_str(argument.as_c_argument(format!("arg{i}")).as_str());
                }
                signature.push_str(");");
                tags.entry(symbol.language[0].address)
                    .or_default()
                    .push(signature);
            }
            _ => {}
        }
    }

    Ok(())
}
