#[macro_export]
macro_rules! assert_size {
    ($t:ty, $size:expr) => {
        const _: [(); $size] = [(); core::mem::size_of::<$t>()];
    };
}

#[macro_export]
macro_rules! assert_field_offset {
    ($t:ty, $f:expr, $n:literal) => {
        const _: () = [(); 1][(::std::mem::offset_of!($t, $f) == $n) as usize ^ 1];
    };
}

#[macro_export]
macro_rules! gen_with_vtbl {
    (
        $name:ident,
        $name_vtbl:ident,
        $(
            fn $func:ident($($arg:ident: $arg_t:ty),*) $(-> $func_ret:ty)?
        );*;
        $(
            pub $field:ident: $field_t:ty
        ),*,
    ) => {
        #[repr(C)]
        #[derive(Debug)]
        #[allow(non_camel_case_types, non_snake_case)]
        pub struct /* VFT */ /*$ {concat($name, _vtbl)}*/ $name_vtbl {
            $(
                pub $func: extern "C" fn(this: *mut $name $(, $arg: $arg_t)*) $(-> $func_ret)?
            ),*
        }

        #[repr(C)]
        #[derive(Debug)]
        pub struct $name {
            pub __vftable: *mut /*$ {concat($name, _vtbl)}*/ $name_vtbl,
            $(
                pub $field: $field_t
            ),*
        }

        impl $name {
            pub fn __vftable<'a>(this: *mut $name) -> &'a /*$ {concat($name, _vtbl)}*/ $name_vtbl {
                let instance = unsafe { &*this };
                let vftable = unsafe { &*instance.__vftable };
                vftable
            }

            $(
                #[allow(non_snake_case)]
                pub fn $func(this: *mut $name $(, $arg: $arg_t)*) $(-> $func_ret)? {
                    let vftable = Self::__vftable(this as *const _ as *mut _);
                    (vftable.$func)(this $(, $arg)*)
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! impl_instance {
    ($name:ident, $signature:literal, $instruction_length:literal) => {
        impl $name {
            pub fn get_instance() -> Option<&'static $name> {
                let ptr = ::cauldron::mem::offset::Offset::from_signature($signature)
                    .unwrap()
                    .as_relative($instruction_length)
                    .as_ptr::<*mut $name>();
                if !ptr.is_null() {
                    let ptr = unsafe { *ptr };
                    if !ptr.is_null() {
                        let instance = unsafe { &*ptr };
                        return Some(instance);
                    }
                }
                None
            }
        }
    };
    ($name:ident, $signature:literal) => {
        impl_instance!($name, $signature, 7);
    };
}

#[cfg(test)]
mod tests {

    #[test]
    const fn size_zero() {
        assert_size!((), 0x0);
    }

    #[test]
    const fn size_bool() {
        assert_size!(bool, 0x1);
    }

    #[test]
    const fn size_array() {
        assert_size!([u8; 5], 0x5);
    }

    #[test]
    const fn size_struct() {
        #[allow(dead_code)]
        #[repr(C)]
        struct S {
            a: u8, // 1
            // 1 (pad)
            b: u16, // 2
        }
        assert_size!(S, 0x4);
    }

    #[test]
    const fn size_enum() {
        #[allow(dead_code)]
        enum E {
            A,      // 1
            B(u8),  // 2
            C(u16), // 4
        }
        assert_size!(E, 0x4);
    }

    #[test]
    const fn offset() {
        #[allow(dead_code)]
        #[repr(C)]
        struct S {
            a: u8,
            b: u16,
        }
        assert_field_offset!(S, a, 0x0);
        assert_field_offset!(S, b, 0x2);
    }
}
