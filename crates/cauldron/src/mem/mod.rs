// #[deprecated]
pub mod offset;

use std::ffi::c_void;
use windows::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows::Win32::System::Memory::{
    PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualProtect,
};
use windows::Win32::System::Threading::GetCurrentProcess;

pub fn patch(ptr: *mut c_void, data: &[u8]) {
    if !ptr.is_null() {
        unsafe {
            let mut flags = PAGE_PROTECTION_FLAGS::default();
            VirtualProtect(ptr, data.len(), PAGE_EXECUTE_READWRITE, &mut flags).unwrap();
            for (index, byte) in data.iter().enumerate() {
                std::ptr::write_bytes(ptr.add(index * size_of::<u8>()), *byte, 1);
            }
            VirtualProtect(ptr, data.len(), flags, &mut flags).unwrap();
            FlushInstructionCache(GetCurrentProcess(), Some(ptr as *const _), data.len()).unwrap();
        }
    }
}
