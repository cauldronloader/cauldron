#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libdecima_rtti::sys::DecimaRTTI;

#[derive(Debug)]
#[repr(C)]
pub struct RTTIObject_VTable {
    pub GetRTTI: *const extern "C" fn(this: *mut RTTIObject) -> *const DecimaRTTI,
    pub Destructor: *mut extern "C" fn(this: *mut RTTIObject),
}

#[derive(Debug)]
#[repr(C)]
pub struct RTTIObject {
    pub __vftable: *const RTTIObject_VTable,
}

impl RTTIObject {
    pub fn GetRTTI(&mut self) -> &DecimaRTTI {
        unsafe { &*(&*(&*self.__vftable).GetRTTI)(self as *mut Self) }
    }
}

impl Drop for RTTIObject {
    fn drop(&mut self) {
        unsafe { (&*(&*self.__vftable).Destructor)(self as *mut Self) }
    }
}
