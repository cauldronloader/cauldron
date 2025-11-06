use crate::NamedRTTI;
use crate::sys::*;
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

impl NamedRTTI for DecimaRTTI {
    fn get_symbol_name(&self) -> String {
        match self.kind {
            DecimaRTTIKind::Atom => self.as_atom_unchecked().get_symbol_name(),
            DecimaRTTIKind::Pointer => self.as_pointer_unchecked().get_symbol_name(),
            DecimaRTTIKind::Container => self.as_container_unchecked().get_symbol_name(),
            DecimaRTTIKind::Enum => self.as_enum_unchecked().get_symbol_name(),
            DecimaRTTIKind::Compound => self.as_compound_unchecked().get_symbol_name(),
            DecimaRTTIKind::FlagsEnum => self.as_enum_unchecked().get_symbol_name(),
            DecimaRTTIKind::Pod => self.as_pod_unchecked().get_symbol_name(),
            DecimaRTTIKind::BitSetEnum => self.as_bitset_enum_unchecked().get_symbol_name(),
        }
    }
}

impl NamedRTTI for DecimaRTTIAtom {
    fn get_symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}

impl NamedRTTI for DecimaRTTIPointer {
    fn get_symbol_name(&self) -> String {
        let outer_name = {
            let c_str = unsafe { (&*self.pointer_type).type_name };
            if c_str.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
            }
        };

        format!("{outer_name}<{}>", unsafe {
            (&*self.item_type).get_symbol_name()
        })
    }
}

impl NamedRTTI for DecimaRTTIContainer {
    fn get_symbol_name(&self) -> String {
        let outer_name = {
            let c_str = unsafe { (&*self.container_type).type_name };
            if c_str.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
            }
        };

        format!("{outer_name}<{}>", unsafe {
            (&*self.item_type).get_symbol_name()
        })
    }
}

impl NamedRTTI for DecimaRTTIEnum {
    fn get_symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}

impl NamedRTTI for DecimaRTTIEnumValue {
    fn get_symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap().to_string() }
    }
}

impl NamedRTTI for DecimaRTTICompound {
    fn get_symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}

impl NamedRTTI for DecimaRTTIPod {
    fn get_symbol_name(&self) -> String {
        format!("DecimaRTTIPod({:0x})", self.size)
    }
}

impl NamedRTTI for DecimaRTTIBitSetEnum {
    fn get_symbol_name(&self) -> String {
        unsafe { CStr::from_ptr(self.type_name).to_str().unwrap().to_string() }
    }
}
