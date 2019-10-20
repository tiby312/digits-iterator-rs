# Digits Iterator

This crate adds an extension method to the integers that permits
to iterate over their digits.

Note that the signed integers will be casted to the corresponding unsigned
integers. Do not use this iterator with negative signed integers unless you
*really* want to iterate over the digits of the complement.

To use this extension, add the crate and import its content:

```rust
extern crate digits_iterator;
use digits_iterator::*;
```

## Examples

```rust
use digits_iterator::*;

let digits: Vec<_> = 2018_u32.digits().collect();
assert_eq!(digits[..], [2, 0, 1, 8]);

let digits: Vec<_> = 0b101010.digits_with_base(2).collect();
assert_eq!(digits[..], [1_u8, 0, 1, 0, 1, 0]);
```
# digits-iterator-rs
