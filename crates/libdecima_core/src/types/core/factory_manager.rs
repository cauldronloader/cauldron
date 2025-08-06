use crate::types::core::rtti::{RTTI, RTTIPod, RTTIPointerData};
use crate::types::p_core::array::Array;
use crate::types::p_core::hashmap::{HashMap, HashSet};
use crate::types::p_core::lock::SharedLockProtected;
use crate::{gen_with_vtbl, impl_instance};
use std::ffi::c_void;

gen_with_vtbl!(
    FactoryManager,
    FactoryManagerVtbl,

    fn constructor();
    fn destructor();
    fn register(rtti: *const RTTI);
    fn unregister(rtti: *const RTTI);

    pub types: HashSet<*const RTTI>,
    pub pod_types: HashMap<*mut RTTIPod, u32>,
    pub locked_types: SharedLockProtected<HashSet<*const RTTI>>,
    pub pointer_types: HashMap<RTTIPointerData, HashMap<*const RTTI, *const RTTI>>,
    pub unk_50: HashMap<*const RTTI, *mut c_void>,
    pub unk_60: SharedLockProtected<Array<*mut c_void>>,
);

impl_instance!(
    FactoryManager,
    "48 8B 0D ? ? ? ? 48 89 54 24 ? 8B 42 F8 89 44 24 28 8B 42 F4 48 8D 54 24 ? 89 44 24 2C E8 ? ? ? ? 48 85 C0 74 0D 48 8B C8 E8"
);
