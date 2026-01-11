use crate::sys::*;
use crate::{RTTIWithAliases, RTTIWithName, RTTIWithValues};
use std::ffi::CStr;

impl DecimaRTTI {
    pub fn as_atom_unchecked(&self) -> &DecimaRTTIAtom {
        unsafe {
            &*std::mem::transmute::<*mut DecimaRTTI, *mut DecimaRTTIAtom>(
                self as *const DecimaRTTI as *mut DecimaRTTI,
            )
        }
    }

    pub fn as_atom(&self) -> Option<&DecimaRTTIAtom> {
        if self.kind == DecimaRTTIKind::Atom {
            Some(self.as_atom_unchecked())
        } else {
            None
        }
    }

    pub fn as_pointer_unchecked(&self) -> &DecimaRTTIPointer {
        unsafe {
            &*std::mem::transmute::<*mut DecimaRTTI, *mut DecimaRTTIPointer>(
                self as *const DecimaRTTI as *mut DecimaRTTI,
            )
        }
    }

    pub fn as_pointer(&self) -> Option<&DecimaRTTIPointer> {
        if self.kind == DecimaRTTIKind::Pointer {
            Some(self.as_pointer_unchecked())
        } else {
            None
        }
    }

    pub fn as_container_unchecked(&self) -> &DecimaRTTIContainer {
        unsafe {
            &*std::mem::transmute::<*mut DecimaRTTI, *mut DecimaRTTIContainer>(
                self as *const DecimaRTTI as *mut DecimaRTTI,
            )
        }
    }

    pub fn as_container(&self) -> Option<&DecimaRTTIContainer> {
        if self.kind == DecimaRTTIKind::Container {
            Some(self.as_container_unchecked())
        } else {
            None
        }
    }

    pub fn as_enum_unchecked(&self) -> &DecimaRTTIEnum {
        unsafe {
            &*std::mem::transmute::<*mut DecimaRTTI, *mut DecimaRTTIEnum>(
                self as *const DecimaRTTI as *mut DecimaRTTI,
            )
        }
    }

    pub fn as_enum(&self) -> Option<&DecimaRTTIEnum> {
        if self.kind == DecimaRTTIKind::Enum {
            Some(self.as_enum_unchecked())
        } else {
            None
        }
    }

    pub fn as_compound_unchecked(&self) -> &DecimaRTTICompound {
        unsafe {
            &*std::mem::transmute::<*mut DecimaRTTI, *mut DecimaRTTICompound>(
                self as *const DecimaRTTI as *mut DecimaRTTI,
            )
        }
    }

    pub fn as_compound(&self) -> Option<&DecimaRTTICompound> {
        if self.kind == DecimaRTTIKind::Compound {
            Some(self.as_compound_unchecked())
        } else {
            None
        }
    }

    pub fn as_pod_unchecked(&self) -> &DecimaRTTIPod {
        unsafe {
            &*std::mem::transmute::<*mut DecimaRTTI, *mut DecimaRTTIPod>(
                self as *const DecimaRTTI as *mut DecimaRTTI,
            )
        }
    }

    pub fn as_pod(&self) -> Option<&DecimaRTTIPod> {
        if self.kind == DecimaRTTIKind::Pod {
            Some(self.as_pod_unchecked())
        } else {
            None
        }
    }

    pub fn as_bitset_enum_unchecked(&self) -> &DecimaRTTIBitSetEnum {
        unsafe {
            &*std::mem::transmute::<*mut DecimaRTTI, *mut DecimaRTTIBitSetEnum>(
                self as *const DecimaRTTI as *mut DecimaRTTI,
            )
        }
    }

    pub fn as_bitset_enum(&self) -> Option<&DecimaRTTIBitSetEnum> {
        if self.kind == DecimaRTTIKind::BitSetEnum {
            Some(self.as_bitset_enum_unchecked())
        } else {
            None
        }
    }
}

impl RTTIWithName for DecimaRTTI {
    fn symbol_name(&self) -> String {
        match self.kind {
            DecimaRTTIKind::Atom => self.as_atom_unchecked().symbol_name(),
            DecimaRTTIKind::Pointer => self.as_pointer_unchecked().symbol_name(),
            DecimaRTTIKind::Container => self.as_container_unchecked().symbol_name(),
            DecimaRTTIKind::Enum => self.as_enum_unchecked().symbol_name(),
            DecimaRTTIKind::Compound => self.as_compound_unchecked().symbol_name(),
            DecimaRTTIKind::FlagsEnum => self.as_enum_unchecked().symbol_name(),
            DecimaRTTIKind::Pod => self.as_pod_unchecked().symbol_name(),
            DecimaRTTIKind::BitSetEnum => self.as_bitset_enum_unchecked().symbol_name(),
        }
    }
}

impl RTTIWithName for DecimaRTTIAtom {
    fn symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}

impl RTTIWithName for DecimaRTTIPointer {
    fn symbol_name(&self) -> String {
        let outer_name = {
            let c_str = unsafe { (&*self.pointer_type).type_name };
            if c_str.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
            }
        };

        format!("{outer_name}<{}>", unsafe {
            (&*self.item_type).symbol_name()
        })
    }
}

impl RTTIWithName for DecimaRTTIPointerData {
    fn symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}

impl RTTIWithName for DecimaRTTIContainer {
    fn symbol_name(&self) -> String {
        let outer_name = {
            let c_str = unsafe { (&*self.container_type).type_name };
            if c_str.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
            }
        };

        format!("{outer_name}<{}>", unsafe {
            (&*self.item_type).symbol_name()
        })
    }
}

impl RTTIWithName for DecimaRTTIContainerData {
    fn symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}

impl RTTIWithName for DecimaRTTIEnum {
    fn symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}

impl RTTIWithName for DecimaRTTIEnumValue {
    fn symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap().to_string() }
    }
}

impl RTTIWithValues for DecimaRTTIEnum {
    type Value = DecimaRTTIEnumValue;

    fn values(&self) -> Vec<Self::Value> {
        if self.values_length > 0 && !self.values.is_null() {
            unsafe {
                Vec::from_raw_parts(
                    self.values as *mut DecimaRTTIEnumValue,
                    self.values_length as usize,
                    self.values_length as usize,
                )
            }
        } else {
            Vec::new()
        }
    }
}

impl RTTIWithAliases for DecimaRTTIEnumValue {
    fn aliases(&self) -> Vec<String> {
        fn maybe_alias(ptr: &*const c_char) -> Option<String> {
            let ptr = *ptr;
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap().to_string() })
            }
        }

        self.aliases.iter().filter_map(maybe_alias).collect()
    }
}

impl RTTIWithName for DecimaRTTICompound {
    fn symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}

impl DecimaRTTICompound {
    pub fn bases(&self) -> Vec<DecimaRTTICompoundBase> {
        if self.bases_length == 0 || self.bases.is_null() {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    self.bases as *mut _,
                    self.bases_length as usize,
                ).to_vec()
            }
        }
    }

    pub fn attributes(&self) -> Vec<DecimaRTTICompoundAttribute> {
        if self.attributes_length == 0 || self.attributes.is_null() {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    self.attributes as *mut _,
                    self.attributes_length as usize,
                ).to_vec()
            }
        }
    }

    pub fn message_handlers(&self) -> Vec<DecimaRTTICompoundMessageHandler> {
        if self.message_handlers_length == 0 || self.message_handlers.is_null() {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    self.message_handlers as *mut _,
                    self.message_handlers_length as usize,
                ).to_vec()
            }
        }
    }

    pub fn message_order_entries(&self) -> Vec<DecimaRTTICompoundMessageOrderEntry> {
        if self.message_order_entries_length == 0 || self.message_order_entries.is_null() {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    self.message_order_entries as *mut _,
                    self.message_order_entries_length as usize,
                ).to_vec()
            }
        }
    }

    pub fn ordered_attributes(&self) -> Vec<DecimaRTTICompoundOrderedAttribute> {
        if self.ordered_attributes_length == 0 || self.ordered_attributes.is_null() {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    self.ordered_attributes as *mut _,
                    self.ordered_attributes_length as usize,
                ).to_vec()
            }
        }
    }
}

impl RTTIWithName for DecimaRTTICompoundAttribute {
    fn symbol_name(&self) -> String {
        unsafe {
            CStr::from_ptr(self.attribute_name)
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}

impl RTTIWithName for DecimaRTTICompoundOrderedAttribute {
    fn symbol_name(&self) -> String {
        unsafe {
            CStr::from_ptr(self.attribute_name)
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}

impl RTTIWithName for DecimaRTTIPod {
    fn symbol_name(&self) -> String {
        format!("DecimaRTTIPod({})", self.size)
    }
}

impl RTTIWithName for DecimaRTTIBitSetEnum {
    fn symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}
