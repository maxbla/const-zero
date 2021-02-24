# Const-zero

Provides a const version of [`std::mem::zeroed()`](https://doc.rust-lang.org/std/mem/fn.zeroed.html) or more accurately `core::mem::zeroed()`, as it does not require `std`.

## Example Usage

```rust
use const_zeroed::const_zero;
struct OpaqueStruct {};
static mut zeroed_opaque: OpaqueStruct = unsafe {const_zero!(OpaqueStruct)};
```