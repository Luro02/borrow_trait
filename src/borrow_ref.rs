// Note: you can use `cargo expand path::to::this::file` to debug the macro :)
// Planned/Todo:
// + it might be useful to pass an entire function body to the macro
//   the problem is, that you can't pass 'self' to a macro like this:
//   borrow_mut![... => RefCell::borrow_mut(self) => ...];
// + might export the macros
// + merge the macros?
use core::cell::{Ref, RefCell};
use core::ops::Deref;

/// A trait for immutably borrowing data.
///
/// The `borrow` function returns an immutable reference to `Self::Target`.
/// I strongly discourage implementing the trait in it's current form, as it will change in the
/// near future!
/// ```
/// use std::ops::Deref;
/// use std::cell::RefCell;
/// use borrow_trait::{ BorrowRef };
///
/// fn takes_bound<T>(value: &T)
/// where
///     T: for<'a> BorrowRef<'a, Target = String>,
/// {
///     assert_eq!(value.borrow().deref(), &"Hello World".to_string());
/// }
///
/// let value = RefCell::new("Hello World".to_string());
/// takes_bound(&value)
/// ```
/// # Implementation Example
/// Implementing `BorrowRef` for RefCell:
/// ``` ignore
/// use std::cell::{ Ref, RefCell };
/// use borrow_trait::{ BorrowRef };
///
/// impl<'a, T: 'a> BorrowRef<'a> for RefCell<T> {
///     type Target = T;
///     type Pointer = Ref<'a, Self::Target>;
///
///     fn borrow(&'a self) -> Self::Pointer { RefCell::borrow(self) }
/// }
/// ```
pub trait BorrowRef<'a> {
    /// The type, that is wrapped by the implementation.
    /// # Example
    /// A `RefCell<T>` wraps around `T`, therefore `Target` has to be `T`
    /// ``` ignore
    /// type Target = T;
    /// ```
    type Target;
    /// The type returned by the implementor.
    /// # Example
    /// A `RefCell` returns `RefMut` so `Pointer` has to be `RefMut`.
    /// ``` ignore
    /// type Pointer = RefMut<'a, Self::Target>;
    /// ```
    type Pointer: 'a + Deref<Target = Self::Target>;

    /// Immutably borrows the wrapped value.
    /// Multiple immutable borrows can be taken out at the same time.
    /// # Panics
    /// The function should panic, if the value is currently mutably borrowed.
    /// # Example
    /// ```
    /// use std::ops::Deref;
    /// use std::cell::RefCell;
    /// use borrow_trait::{ BorrowRef };
    ///
    /// fn takes_bound<T>(value: &T)
    /// where
    ///     T: for<'a> BorrowRef<'a, Target = String>,
    /// {
    ///     let first_borrow = value.borrow();
    ///     let second_borrow = value.borrow();
    ///
    ///     assert_eq!(first_borrow.deref(), &"Hello World".to_string());
    ///     assert_eq!(second_borrow.deref(), &"Hello World".to_string());
    /// }
    ///
    /// let value = RefCell::new("Hello World".to_string());
    /// takes_bound(&value)
    /// ```
    fn borrow(&'a self) -> Self::Pointer;
}

macro_rules! borrow_ref {
    (
        $( $pointer:ty => $body:path => $( $name:ty ),* );*
        $(;)* // <- allows to have a trailing semi-colon
    ) => {
        $(
            $(
                impl<'a, T: 'a> BorrowRef<'a> for $name {
                    type Target = T;
                    type Pointer = $pointer;

                    #[inline]
                    fn borrow(&'a self) -> Self::Pointer { $body(self) }
                }
            )* // repeat for each value, seperated by ','
        )* // repeat for each line, seperated by ';'
    };
}

borrow_ref![
    Ref<'a, T> => RefCell::borrow => RefCell<T>, &RefCell<T>, &mut RefCell<T>;
];

// AtomicRefCell
#[cfg(all(feature = "atomic_refcell", feature = "alloc"))]
use atomic_refcell::{AtomicRef, AtomicRefCell};
#[cfg(all(feature = "atomic_refcell", feature = "alloc"))]
borrow_ref![
    AtomicRef<'a, T> =>
    AtomicRefCell::borrow =>
    AtomicRefCell<T>, &AtomicRefCell<T>, &mut AtomicRefCell<T>;
];

// Cell
#[cfg(all(feature = "cell", feature = "alloc"))]
use cell;
#[cfg(all(feature = "cell", feature = "alloc"))]
borrow_ref![
    cell::Ref<'a, T> =>
    cell::RefCell::borrow =>
    cell::RefCell<T>, &cell::RefCell<T>, &mut cell::RefCell<T>;
];
