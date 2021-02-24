#![no_std]

/// A marco that acts similarly to std::mem::zeroed(), only is const
/// Example usage:
/// ```rust
/// use const_zeroed::const_zero;
/// struct OpaqueStruct {};
/// static mut zeroed_opaque: OpaqueStruct = unsafe {const_zero!(OpaqueStruct)};
/// ```
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

    #[test]
    fn zeroed_int() {
        const ZERO: i32 = unsafe { const_zero!(i32) };
        assert_eq!(ZERO, 0);
    }

    #[test]
    fn zeroed_ptr() {
        const NULL: *const () = unsafe { const_zero!(*const ()) };
        assert_eq!(NULL, core::ptr::null());
    }

    #[test]
    fn zeroed_option() {
        const NONE: Option<NonZeroU8> = unsafe { const_zero!(Option<NonZeroU8>) };
        assert_eq!(NONE, None);
    }

    #[test]
    fn drop_type() {
        #[derive(Clone, Debug)]
        struct Droppable {
            inner: (),
        }
        impl Drop for Droppable {
            fn drop(&mut self) {
                //no op
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
