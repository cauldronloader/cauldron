use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum ExportedType {
    Atom {
        id: u32,
        factory_flags: u8,

        size: u16,
        alignment: u8,
        simple: bool,
        type_name: String,
        base_type: String,
    },
    Enum {
        id: u32,
        factory_flags: u8,

        size: u8,
        alignment: u8,
        type_name: String,
        values: Vec<ExportedEnumValue>,
    },
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportedEnumValue {
    pub value: i32,
    pub name: String,
    pub aliases: Vec<String>,
}
