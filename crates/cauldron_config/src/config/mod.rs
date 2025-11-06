use crate::Result;
use documented::DocumentedFields;
use serde::{Deserialize, Serialize};
use toml_edit::{Decor, DocumentMut, RawString, Table};

pub mod v1;

#[derive(Serialize, Deserialize)]
pub struct CauldronConfigVersionOnly {
    pub config_version: u32,
}

pub trait Config {
    fn as_annotated_toml<T>(&self) -> Result<DocumentMut>
    where
        Self: Serialize + DocumentedFields + Sized,
    {
        let mut toml = toml_edit::ser::to_string_pretty(self)?.parse::<DocumentMut>()?;

        annotate_toml_table::<Self>(toml.as_table_mut())?;

        Ok(toml)
    }
}

fn annotate_toml_table<T>(table: &mut Table) -> Result<()>
where
    T: DocumentedFields,
{
    use toml_edit::Item as I;

    fn append_docs_as_toml_comments(decor: &mut Decor, docs: &str) {
        let old_prefix = decor.prefix().and_then(RawString::as_str);
        let last_line = old_prefix.and_then(|p| p.lines().last());

        let comments = docs
            .lines()
            .map(|l| {
                if l.is_empty() {
                    "#\n".into()
                } else {
                    format!("# {l}\n")
                }
            })
            .collect();

        let new_prefix = match (old_prefix, last_line) {
            // no prior comments
            (None | Some(""), None) => comments,
            // no prior comments, but somehow there are lines
            (None, Some(_)) => unreachable!(),
            // prior comments is contentful, but there are no lines
            (Some(_), None) => unreachable!(),
            // last line of prior comments is empty
            (Some(prefix), Some("")) => format!("{prefix}{comments}"),
            // last line of prior comments is contentful
            (Some(prefix), Some(_)) => format!("{prefix}#\n{comments}"),
        };
        decor.set_prefix(new_prefix);
    }

    for (mut key, value) in table.iter_mut() {
        let field_name = key.get();
        let Ok(docs) = T::get_field_docs(&field_name) else {
            // let ty = type_name::<T>();
            // warn!("no field docs for {ty}");
            continue;
        };

        match value {
            I::None => {}
            I::Value(_) => append_docs_as_toml_comments(key.leaf_decor_mut(), docs),
            I::Table(sub_table) => append_docs_as_toml_comments(sub_table.decor_mut(), docs),
            I::ArrayOfTables(arr) => {
                let first_table = arr
                    .iter_mut()
                    .next()
                    .expect("Array of tables should not be empty");
                append_docs_as_toml_comments(first_table.decor_mut(), docs);
            }
        }
    }

    Ok(())
}
