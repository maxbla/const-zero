#![no_std]

/// A marco that acts similarly to `std::mem::zeroed()`, only is const
/// Example usage:
/// ```rust
/// use const_zero::const_zero;
/// struct OpaqueStruct {
///     nothing: core::ffi::c_void,
/// };
/// static mut zeroed_opaque: OpaqueStruct = unsafe {const_zero!(OpaqueStruct)};
/// ```
/// Ideally const_zero would be a generic function, but const generics need
/// more development first (`const_fn_transmute`, `const_generics`,
/// `const_evaluatable_checked`)
///
/// ## Differences with `std::mem::zeroed`
/// `const_zero` zeroes padding bits, while `std::mem::zeroed` doesn't
#[macro_export]
macro_rules! const_zero {
    ($type_:ty) => {{
        const TYPE_SIZE: usize = core::mem::size_of::<$type_>();
        union TypeAsBytes {
            bytes: [u8; TYPE_SIZE],
            inner: core::mem::ManuallyDrop<$type_>,
        };
        const ZERO: TypeAsBytes = TypeAsBytes {
            bytes: [0; TYPE_SIZE],
        };
        core::mem::ManuallyDrop::<$type_>::into_inner(ZERO.inner)
    }};
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroU8;
    use core::num::Wrapping;

    // Ensure macros are hygienic, don't create name conflicts
    #[test]
    fn multiple() {
        const ZERO_1: i32 = unsafe { const_zero!(i32) };
        const ZERO_2: i32 = unsafe { const_zero!(i32) };
        const ZERO_3: i64 = unsafe { const_zero!(i64) };
        assert_eq!(ZERO_1, 0);
        assert_eq!(ZERO_2, 0);
        assert_eq!(ZERO_3, 0);
    }

    // All integers can be constructed
    #[test]
    fn zeroed_int() {
        macro_rules! test_int {
            ($type_:ty, $zero:expr) => {{
                const ZERO: $type_ = unsafe { const_zero!($type_) };
                assert_eq!(ZERO, $zero);
            }};
        }
        test_int!(i8, 0);
        test_int!(i16, 0);
        test_int!(i32, 0);
        test_int!(i64, 0);
        test_int!(i128, 0);
        test_int!(isize, 0);
        test_int!(Wrapping<i8>, Wrapping(0));
        test_int!(Wrapping<i16>, Wrapping(0));
        test_int!(Wrapping<i32>, Wrapping(0));
        test_int!(Wrapping<i64>, Wrapping(0));
        test_int!(Wrapping<i128>, Wrapping(0));
        test_int!(Wrapping<isize>, Wrapping(0));

        test_int!(u8, 0);
        test_int!(u16, 0);
        test_int!(u32, 0);
        test_int!(u64, 0);
        test_int!(u128, 0);
        test_int!(usize, 0);
        test_int!(Wrapping<u8>, Wrapping(0));
        test_int!(Wrapping<u16>, Wrapping(0));
        test_int!(Wrapping<u32>, Wrapping(0));
        test_int!(Wrapping<u64>, Wrapping(0));
        test_int!(Wrapping<u128>, Wrapping(0));
        test_int!(Wrapping<usize>, Wrapping(0));

        test_int!(f32, 0.);
        test_int!(f64, 0.);
    }

    #[test]
    fn zeroed_ptr() {
        const NULL: *const () = unsafe { const_zero!(*const ()) };
        assert_eq!(NULL, core::ptr::null());
        const NULL_MUT: *mut () = unsafe { const_zero!(*mut ()) };
        assert_eq!(NULL_MUT, core::ptr::null_mut());
    }

    // sentinel value Option optimization works
    #[test]
    fn zeroed_option() {
        const NONE: Option<NonZeroU8> = unsafe { const_zero!(Option<NonZeroU8>) };
        assert_eq!(NONE, None);
    }

    // a type with a drop implementation works
    #[test]
    fn drop_type() {
        #[derive(Clone, Debug)]
        struct Droppable {
            inner: (),
        }
        impl Drop for Droppable {
            fn drop(&mut self) {
                // no-op
            }
        }
        #[allow(unused)]
        const DROPPABLE: Droppable = unsafe { const_zero!(Droppable) };
    }

    #[test]
    fn zeroed_unit() {
        const UNIT: () = unsafe { const_zero!(()) };
        assert_eq!((), UNIT);
    }
}
