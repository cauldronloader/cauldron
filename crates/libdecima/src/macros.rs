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
