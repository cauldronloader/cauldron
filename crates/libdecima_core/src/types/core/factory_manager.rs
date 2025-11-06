use crate::types::p_core::array::Array;
use crate::types::p_core::hashmap::{HashMap, HashSet};
use crate::types::p_core::lock::SharedLockProtected;
use crate::{gen_with_vtbl, impl_instance};
use libdecima_rtti::sys::{DecimaRTTI, DecimaRTTIPod, DecimaRTTIPointerData};
use std::ffi::c_void;

gen_with_vtbl!(
    FactoryManager,
    FactoryManagerVtbl,

    fn constructor();
    fn destructor();
    fn register(rtti: *const DecimaRTTI);
    fn unregister(rtti: *const DecimaRTTI);

    pub types: HashSet<*const DecimaRTTI>,
    pub pod_types: HashMap<*mut DecimaRTTIPod, u32>,
    pub locked_types: SharedLockProtected<HashSet<*const DecimaRTTI>>,
    pub pointer_types: HashMap<DecimaRTTIPointerData, HashMap<*const DecimaRTTI, *const DecimaRTTI>>,
    pub unk_50: HashMap<*const DecimaRTTI, *mut c_void>,
    pub unk_60: SharedLockProtected<Array<*mut c_void>>,
);

impl_instance!(
    FactoryManager,
    "48 8B 0D ? ? ? ? 48 89 54 24 ? 8B 42 F8 89 44 24 28 8B 42 F4 48 8D 54 24 ? 89 44 24 2C E8 ? ? ? ? 48 85 C0 74 0D 48 8B C8 E8"
);
