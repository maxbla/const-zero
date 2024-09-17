# This crate is no longer necessary, as [`MaybeUninit::zeroed()`](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.zeroed) is const as of rust 1.75

# Const-zero

Provides a const version of [`core::mem::zeroed()`](https://doc.rust-lang.org/core/mem/fn.zeroed.html).

## Example Usage

Example usage:
```rust
use const_zero::const_zero;
struct OpaqueStruct {
    nothing: core::ffi::c_void,
};
static mut zeroed_opaque: OpaqueStruct = unsafe {const_zero!(OpaqueStruct)};
```
Ideally const_zero would be a generic function, but const generics need
more development first (`const_fn_transmute`, `const_generics`, 
`const_evaluatable_checked`)

## Differences with `std::mem::zeroed`
`const_zero` zeroes padding bits, while `std::mem::zeroed` doesn't

# How does it work?

The simplified version is
```rust
union TypeAsBytes<T> {
    bytes: [u8; core::mem::size_of::<T>()],
    inner: T,
};
```
which can be initalized with
```rust
TypeAsBytes {bytes: [0; core::mem::size_of::<T>()]};
```
Feel free to use this trick in your code if you want to skip out on a dependency