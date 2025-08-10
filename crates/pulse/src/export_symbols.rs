use libdecima_core::types::core::exported_symbols::{ExportedSymbolKind, ExportedSymbolLanguage, ExportedSymbols};
use std::ffi::CStr;
use std::fs::File;
use std::io::Write as _;

pub fn export_symbols() -> anyhow::Result<()> {
    let mut file = File::create("cauldron/symbols.csv")?;

    writeln!(file, "Hash,Namespace,Name,Kind,Signature,Header,Source")?;

    let symbols = ExportedSymbols::get().unwrap().all_symbols.slice();
    let mut exported: Vec<u32> = Vec::new();
    for entry in symbols {
        let hash = entry.value.value;
        if entry.value.key.is_null() {
            continue;
        }
        if exported.contains(&hash) {
            continue;
        } else {
            exported.push(hash);
        }
        let symbol = unsafe { &*entry.value.key };


        let namespace = symbol.namespace().unwrap_or_default();
        let kind = &symbol.kind;
        if symbol.kind != ExportedSymbolKind::Function {
            continue;
        }
        let language: &ExportedSymbolLanguage = &symbol.language[0];
        let source_file = if language.source_file.is_null() {
            String::new()
        } else {
            unsafe {
                CStr::from_ptr(language.source_file)
                    .to_str()
                    .unwrap_or_default()
                    .to_owned()
            }
        };
        let header_file = if language.header_file.is_null() {
            String::new()
        } else {
            unsafe {
                CStr::from_ptr(language.header_file)
                    .to_str()
                    .unwrap_or_default()
                    .to_owned()
            }
        };
        let name = language.name().unwrap_or(symbol.name().unwrap_or_default());
        let signature = {
            match kind {
                ExportedSymbolKind::Function => {
                    let mut sig = String::new();
                    let signature = language.signature.as_slice();
                    if signature.len() == 0 {
                        sig.push_str("void fn();");
                    } else {
                        for (i, arg) in signature.iter().enumerate() {
                            if i == 0 {
                                // function return type
                                sig.push_str(arg.as_c_type().as_str());
                                sig.push_str("fn(");
                            } else {
                                // function argument
                                sig.push_str(
                                    format!(
                                        "{}{}",
                                        if i == 1 { "" } else { ", " },
                                        arg.as_c_argument(format!("unk{i}"))
                                    )
                                    .as_str(),
                                )
                            }
                        }
                        sig.push_str(");");
                    }

                    sig
                }
                _ => String::new(),
            }
        };

        writeln!(
            file,
            "0x{hash:x},{namespace},{name},{kind},{signature},{header_file},{source_file}"
        )?;
    }

    Ok(())
}
