use std::ops::{Add, Sub};
use std::ptr::read_unaligned;
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Offset(usize);

impl Offset {
    pub fn new(address: usize) -> Self {
        Offset(address)
    }

    pub fn from_signature(pattern: &str) -> Result<Self, PatternSearchError> {
        let (module_start, module_end) = get_module()?;
        let search = find_pattern(module_start as *mut _, module_end - module_start, pattern)?;
        Ok(Self::new(search as _))
    }

    pub fn as_adjusted(&self, offset: usize) -> Offset {
        let result = Offset(self.0.add(offset));
        result
    }

    pub fn as_nadjusted(&self, offset: usize) -> Offset {
        Offset(self.0.sub(offset))
    }

    pub fn as_relative(&self, instruction_length: usize) -> Offset {
        let rel_adjust = unsafe {
            std::mem::transmute::<usize, *mut u32>(
                self.0.add(instruction_length.sub(size_of::<u32>())),
            )
        };
        let rel_adjust = unsafe { read_unaligned(rel_adjust) } as usize;
        let result = Offset(self.0.add(rel_adjust.add(instruction_length)));
        result
    }

    pub fn as_ptr<T>(&self) -> *mut T {
        self.0 as *mut T
    }

    pub fn as_offset(&self) -> *mut u8 {
        self.0.sub(get_module().unwrap().0) as *mut _
    }
}

#[derive(Debug, Clone)]
pub enum PatternSearchError {
    ParseInt(std::num::ParseIntError),
    OutOfRange,
    NotFound,
}

/// parses an ida-style byte sequence pattern
pub fn parse_pattern(mask: &str) -> Result<Vec<(u8, bool)>, PatternSearchError> {
    let mask = mask.replace("?", "??");
    let mask = mask.replace(" ", "");

    (0..mask.len())
        .step_by(2)
        .map(|i| {
            let radix = &mask[i..i + 2];
            if radix == "??" {
                Ok((0x00, true))
            } else {
                Ok((
                    u8::from_str_radix(radix, 16).unwrap(), /*? todo */
                    false,
                ))
            }
        })
        .collect()
}

pub fn find_pattern(
    start_address: *mut u8,
    max_size: usize,
    mask: &str,
) -> Result<*mut u8, PatternSearchError> {
    let pattern = parse_pattern(mask)?;
    let data_end = start_address as usize + max_size + 1;

    let result = unsafe { std::slice::from_raw_parts(start_address, max_size + 1) }
        .windows(pattern.len())
        .position(|pos| {
            pos.iter()
                .enumerate()
                .all(|(i, b)| pattern[i].1 || pattern[i].0.eq(b))
        });

    let Some(result) = result else {
        return Err(PatternSearchError::NotFound);
    };

    if result > data_end {
        return Err(PatternSearchError::OutOfRange);
    }

    Ok((start_address as usize + result) as *mut u8)
}

pub fn get_module() -> Result<(usize, usize), PatternSearchError> {
    let base = unsafe { GetModuleHandleW(None).unwrap() };
    if base.0.is_null() {
        return Err(PatternSearchError::OutOfRange);
    }

    let base = base.0 as usize;
    let dos_header = unsafe { &*(base as *const IMAGE_DOS_HEADER) };
    let nt_headers_ptr =
        (base as isize).wrapping_add(dos_header.e_lfanew as isize) as *const IMAGE_NT_HEADERS64;
    let nt_headers = unsafe {
        if nt_headers_ptr.is_null() {
            return Err(PatternSearchError::OutOfRange);
        } else {
            &*nt_headers_ptr
        }
    };
    let end = base + nt_headers.OptionalHeader.SizeOfImage as usize;
    Ok((base, end))
}
