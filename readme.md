[![Build Status](https://api.travis-ci.org/Luro02/borrow_trait.svg?branch=master)](https://travis-ci.org/Luro02/borrow_trait)
[![Documentation](https://docs.rs/borrow_trait/badge.svg)](https://docs.rs/borrow_trait)
[![Crates.io](https://img.shields.io/crates/v/borrow_trait.svg)](https://crates.io/crates/borrow_trait)

This library provides traits for `borrow` and `borrow_mut` functions, most commonly found in `RefCell`s. Therefore it is possible to accept other kinds of `RefCell`s like an `AtomicRefCell` or smart pointers around `RefCell`s like `Arc`, `Rc` or `Box`.

## Example
``` rust
use std::io::{ Read, Cursor };
use std::cell::RefCell;
use borrow_trait::{ BorrowRefMut };

fn takes_bound<C, T>(value: &T) -> Vec<u8>
where
    T: for<'a> BorrowRefMut<'a, Target = C>,
    C: Read,
{
    let mut result = vec![];
    value.borrow_mut().read_to_end(&mut result).expect("Failed to read from `value: T`");
    result
}

let value = RefCell::new(Cursor::new(vec![0, 1, 2, 3]));
assert_eq!(takes_bound(&value), vec![0, 1, 2, 3]);
```

For more details please refer to the documentation, that you can find here:
[https://docs.rs/borrow_trait](https://docs.rs/borrow_trait)

## Usage
Simply add the following line to your `Cargo.toml` under `[dependencies]`:

```toml
borrow_trait = { version = "0.1" }
```

## Notes
- This crate does conform to semantic versioning.
- It contains not a single line of unsafe code.
- This crate re-exports it's dependencies for ease of use.

## Planned
+ Remove the lifetime requirement of `BorrowRef<'a, C, T>` and `BorrowRefMut<'a, C, T>`.
This feature requires Generic Associated Lifetimes
[rust-lang/rust#44265](https://github.com/rust-lang/rust/issues/44265)

## Credits
+ Parts of the documentation were copied from the std library
+ The feature flags were inspired by the [serde](https://crates.io/crates/serde) and
[rand](https://crates.io/crates/rand) crate.
+ The name for the traits were inspired by
[borrow_with_ref_obj](https://crates.io/crates/borrow_with_ref_obj) crate.

## License

This project is licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](http://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution
If you have any issue please don't hesitate to create one :)

Before you make a PR please ensure, that your code has been formatted with `rustfmt`:

```
cargo fmt
```
