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

    writeln!(
        output_file,
        r#"import time

log = bv.create_logger("Decima")
log.log_info("Starting...")
start_time = time.time()

bv.remove_tag_type("decima")
bv.create_tag_type("decima", "⚙️")

def decima_set_func_name(addr, name):
    func = bv.get_function_at(addr)
    if func != None:
        func.name = name

def decima_tag_func(addr, tag):
    func = bv.get_function_at(addr)
    if func != None:
        func.add_tag("decima", tag)
    else:
        bv.add_tag(addr, "decima", tag)

"#
    )?;

    let mut containers = Vec::new();
    for (i, r#type) in types.iter().enumerate() {
        writeln!(
            output_file,
            "log.log_info(\"Exporting Types: {}/{} ({:.0}%)\")",
            i + 1,
            &types.len(),
            ((i as f64 + 1.0) / types.len() as f64 * 100.0).floor()
        )?;
        export_type(&mut output_file, r#type, &mut containers)?;
    }

    let mut tagged_symbols: HashMap<*const c_void, Vec<(String, String)>> = HashMap::new();
    let symbols = ExportedSymbols::get().unwrap();
    for group in symbols.groups.as_slice() {
        let group = unsafe { &*(*group) };
        export_symbols_group(&mut tagged_symbols, group)?;
    }

    for (i, (ptr, symbols)) in tagged_symbols.iter().enumerate() {
        let ptr = *ptr;
        writeln!(
            output_file,
            "log.log_info(\"Exporting Symbols: {}/{} ({:.0}%)\")",
            i + 1,
            tagged_symbols.len(),
            ((i as f64 + 1.0) / tagged_symbols.len() as f64 * 100.0).floor()
        )?;

        let s = symbols.iter().map(|(s, _)| s.clone()).collect::<Vec<_>>();
        writeln!(
            output_file,
            "decima_tag_func({ptr:p},\"{}\")",
            s.join("\\n")
        )?;
        if symbols.len() == 1 {
            writeln!(
                output_file,
                "decima_set_func_name({ptr:p}, \"{}\")",
                symbols[0].1
            )?;
        }
    }

    writeln!(
        output_file,
        "\nlog.log_info(\"Done. (Took %s seconds)\" % (time.time() - start_time))"
    )?;

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

fn export_type(
    file: &mut File,
    rtti: &RTTI,
    containers: &mut Vec<*const RTTIContainerData>,
) -> anyhow::Result<()> {
    let kind_str = bn_kind_name(&rtti.kind);
    let type_str = rtti.get_symbol_name();

    writeln!(file, "# {type_str} ({kind_str})")?;

    if let Some(compound) = rtti.as_compound() {
        let bases_len = compound.bases_len as usize;
        let bases = compound.bases;
        if !bases.is_null() {
            writeln!(
                file,
                "bv.define_data_var({bases:p}, ArrayType.create(bv.types[\"RTTICompound::Base\"], {bases_len}),\"RTTI_{type_str}__bases\")"
            )?;
        }

        let attrs_len = compound.attributes_len as usize;
        let attrs = compound.attributes;
        if !attrs.is_null() {
            writeln!(
                file,
                "bv.define_data_var({attrs:p}, ArrayType.create(bv.types[\"RTTICompound::Attribute\"], {attrs_len}), \"RTTI_{type_str}__attributes\")"
            )?;
        }

        let msg_handlers_len = compound.message_handlers_len as usize;
        let msg_handlers = compound.message_handlers;
        if !msg_handlers.is_null() {
            writeln!(
                file,
                "bv.define_data_var({msg_handlers:p}, ArrayType.create(bv.types[\"RTTICompound::MessageHandler\"], {msg_handlers_len}), \"RTTI_{type_str}__message_handlers\")"
            )?;
        }

        let msg_order_entries_len = compound.message_order_entries_len as usize;
        let msg_order_entries = compound.message_order_entries;
        if !msg_order_entries.is_null() {
            writeln!(
                file,
                "bv.define_data_var({msg_order_entries:p}, ArrayType.create(bv.types[\"RTTICompound::MessageOrderEntry\"], {msg_order_entries_len}), \"RTTI_{type_str}__message_order_entries\")"
            )?;
        }

        let ordered_attrs_len = compound.ordered_attributes_len as usize;
        let ordered_attrs = compound.ordered_attributes;
        if !ordered_attrs.is_null() {
            writeln!(
                file,
                "bv.define_data_var({ordered_attrs:p}, ArrayType.create(bv.types[\"RTTICompound::OrderedAttribute\"], {ordered_attrs_len}), \"RTTI_{type_str}__ordered_attributes\")"
            )?;
        }
    }

    if let Some(r#enum) = rtti.as_enum() {
        let values = r#enum.values;
        let values_len = r#enum.values_len as usize;
        if !values.is_null() {
            writeln!(
                file,
                "bv.define_data_var({values:p}, ArrayType.create(bv.types[\"RTTIEnum::Value\"], {values_len}), \"RTTI_{type_str}__values\")"
            )?;
        }
    }

    if let Some(container) = rtti.as_container() {
        if !unsafe { containers.contains(&std::mem::transmute(container.container_type)) } {
            unsafe { containers.push(std::mem::transmute(container.container_type)) };
            let data = container.container_type;
            let container_name = unsafe {
                CStr::from_ptr((&*data).type_name)
                    .to_str()?
                    .to_owned()
            };

            if container.base.kind == RTTIKind::Pointer {
                writeln!(
                    file,
                    "bv.define_data_var({data:p}, \"`RTTIPointer::Data`\", \"RTTIPointer_{container_name}\")"
                )?;
            } else {
                writeln!(
                    file,
                    "bv.define_data_var({data:p}, \"`RTTIContainer::Data`\", \"RTTIContainer_{container_name}\")"
                )?;
            }
        }
    }

    writeln!(
        file,
        "bv.define_data_var({rtti:p}, \"{kind_str}\", \"RTTI_{type_str}\")"
    )?;

    Ok(())
}

fn export_symbols_group(
    tags: &mut HashMap<*const c_void, Vec<(String, String)>>,
    group: &ExportedSymbolsGroup,
) -> anyhow::Result<()> {
    for symbol in group.symbols.as_slice() {
        let export_name = {
            let name = symbol
                .exported_definition
                .name()
                .unwrap_or(symbol.name().unwrap());
            if let Some(namespace) = symbol.namespace() {
                format!("{namespace}::{name}")
            } else {
                name
            }
        };
        match symbol.kind {
            ExportedSymbolKind::Variable => {
                let tokens = symbol.exported_definition.tokens.as_slice();
                tags.entry(symbol.exported_definition.address)
                    .or_default()
                    .push((
                        format!("{}{export_name}", tokens[0].as_c_type()),
                        export_name,
                    ));
            }
            ExportedSymbolKind::Function => {
                let tokens = symbol.exported_definition.tokens.as_slice();
                let mut signature = format!("{}{export_name}(", tokens[0].as_c_type());
                for (i, argument) in tokens.iter().enumerate() {
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
                tags.entry(symbol.exported_definition.address)
                    .or_default()
                    .push((signature, export_name));
            }
            _ => {}
        }
    }

    Ok(())
}
