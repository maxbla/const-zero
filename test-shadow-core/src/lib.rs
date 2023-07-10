#![cfg(test)]

//! In this module the name `core` / `::core` is shadowed by a non-conformant implementation.
//!
//! In your usual `core`, `core::primitive::usize` is an unsigned integer.
//! In the shadowed `core`, the type is a signed integer.
//!
//! We want to ensure `const_core!()` works correctly in such a context, i.e.
//!  * that the macro keeps working if e.g. `usize` / `core::primitive::usize` is another type, and
//!  * that the user can use such a shadowed name as argument to `const_zero!()` without the
//!    "canonical" meaning replacing the shadowed name.

use const_zero::const_zero;
use core::primitive::usize;

#[test]
fn name_in_scope() {
    // If the macro was implemented incorrectly, this line would fail to compile, because the names
    // `usize` / `core::primitive::usize` / `::core::primitive::usize` refer to a diffent tyep in
    // this module than their canonical meaning.
    const ZERO: usize = unsafe { const_zero!(usize) };
    // This is just your baseline test if `ZERO` got initialized to `0`.
    std::assert_eq!(ZERO, 0);
    // If the original meaning of `usize` was leaking into the code, then this line would fail to
    // compile, because `-1` in not a valid value for an unsigned integer.
    std::assert_eq!(ZERO - 1, -1);
}

#[test]
fn simple_path() {
    const ZERO: usize = unsafe { const_zero!(core::primitive::usize) };
    std::assert_eq!(ZERO, 0);
    std::assert_eq!(ZERO - 1, -1);
}

#[test]
fn qualified_path() {
    const ZERO: usize = unsafe { const_zero!(::core::primitive::usize) };
    std::assert_eq!(ZERO, 0);
    std::assert_eq!(ZERO - 1, -1);
}
