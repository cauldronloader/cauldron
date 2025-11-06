use std::ffi::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecimaStringData {
    pub ref_count: u32,
    pub crc: u32,
    pub length: u32,
    pub capacity: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecimaString {
    pub data: *const c_char,
}

pub type DString = DecimaString;
