#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(intra_doc_link_resolution_failure)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]
// enable no_std if feature "std" isn't present
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    clippy::excessive_precision,
    clippy::unreadable_literal,
    clippy::float_cmp
)]
//! This library provides traits for [borrow] and
//! [borrow_mut] functions, most commonly found in
//! [RefCell]s. Therefore it is possible to accept other kinds of
//! [RefCell]s like an [AtomicRefCell](atomic_refcell::AtomicRefCell) or
//! smart pointers around [RefCell]s like [Arc](std::sync::Arc),
//! [Rc](std::rc::Rc) or [Box](std::boxed::Box).
//!
//! # Examples
//! Let's say you have a library, that needs to read some data from a [Read]er, but doesn't want to
//! mutate it and wants to accept any kind of [RefCell], that gives a mut reference to the
//! [Read]er.
//! ```
//! use std::io::{ Read, Cursor };
//! use std::cell::RefCell;
//! use borrow_trait::{ BorrowRefMut };
//!
//! fn takes_bound<C, T>(value: &T) -> Vec<u8>
//! where
//!     T: for<'a> BorrowRefMut<'a, Target = C>,
//!     C: Read,
//! {
//!     let mut result = vec![];
//!     value.borrow_mut().read_to_end(&mut result).expect("Failed to read from `value: T`");
//!     result
//! }
//!
//! let value = RefCell::new(Cursor::new(vec![0, 1, 2, 3]));
//! assert_eq!(takes_bound(&value), vec![0, 1, 2, 3]);
//! ```
//! Only accepting [RefCell]s, that can be cloned (for example `Rc<RefCell<T>>`):
//! ```
//! use std::io::{ Read, Cursor };
//! use std::cell::{ RefCell };
//! use borrow_trait::{ BorrowRefMut };
//! use std::rc::{ Rc };
//!
//! fn takes_bound<C, T>(value: T) -> Vec<u8>
//! where
//!     T: for<'a> BorrowRefMut<'a, Target = C> + Clone,
//!     C: Read,
//! {
//!     let mut result = vec![];
//!     value
//!         .clone()
//!         .borrow_mut()
//!         .read_to_end(&mut result)
//!         .expect("Failed to read from `value: T`")
//!     ;
//!     result
//! }
//!
//! let value = Rc::new(RefCell::new(Cursor::new(vec![0, 1, 2, 3])));
//! assert_eq!(takes_bound(value.clone()), vec![0, 1, 2, 3]);
//! ```
//! # Features
//! + `atomic_refcell`, implements traits for [AtomicRefCell] (thread-safe [RefCell])
//! + `cell`, implements traits for [cell::RefCell] (this is not [std::cell::RefCell])
//!
//! `no_std` support can be enabled by adding the following to the `Cargo.toml`:
//! ```toml
//! [dependencies]
//! borrow_trait = { version = "0.1", default-features = false }
//! ```
//! By enabling the `alloc` feature, the library will implement the traits for `Rc`, `Arc` and
//! `Box`.
//! ```toml
//! [dependencies]
//! borrow_trait = { version = "0.1", default-features = false, features = [ "alloc" ] }
//! ```
//!
//! # Notes
//! - This crate re-exports it's dependencies for ease of use.
//! - This crate does conform to semantic versioning.
//! - It contains not a single line of unsafe code.
//!
//! # Planned
//! + Remove the lifetime requirement of `BorrowRef<'a, C, T>` and `BorrowRefMut<'a, C, T>`.
//! This feature requires Generic Associated Lifetimes
//! [rust-lang/rust#44265](https://github.com/rust-lang/rust/issues/44265)
//!
//! # Credits
//! + Parts of the documentation were copied from the std library
//! + The feature flags were inspired by the [serde](https://crates.io/crates/serde) and
//! [rand](https://crates.io/crates/rand) crate.
//! + The name for the traits were inspired by
//! [borrow_with_ref_obj](https://crates.io/crates/borrow_with_ref_obj) crate.
//!
//! [RefCell]: core::cell::RefCell
//! [borrow_mut]: core::cell::RefCell::borrow_mut
//! [borrow]: core::cell::RefCell::borrow
//! [AtomicRefCell]: atomic_refcell::AtomicRefCell
//! [cell::RefCell]: cell::RefCell
//! [Read]: std::io::Read

#[cfg(all(feature = "alloc"))]
extern crate alloc;
mod borrow_ref;
mod borrow_ref_mut;
#[cfg(feature = "alloc")]
mod pointers;

pub use borrow_ref::*;
pub use borrow_ref_mut::*;
#[cfg(feature = "alloc")]
pub use pointers::*;

#[cfg(all(feature = "atomic_refcell", feature = "alloc"))]
pub use atomic_refcell;

#[cfg(all(feature = "cell", feature = "alloc"))]
pub use cell;
