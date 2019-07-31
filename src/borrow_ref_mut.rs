use core::cell::{RefCell, RefMut};
use core::ops::DerefMut;

/// A trait for mutably borrowing data.
///
/// The `borrow` function returns an mutable reference to `Self::Target`.
/// ```
/// use std::ops::DerefMut;
/// use std::cell::RefCell;
/// use borrow_trait::{ BorrowRefMut };
///
/// fn takes_bound<T>(value: &T)
/// where
///     T: for<'a> BorrowRefMut<'a, Target = String>,
/// {
///     assert_eq!(value.borrow_mut().deref_mut(), &mut "Hello World".to_string());
/// }
///
/// let value = RefCell::new("Hello World".to_string());
/// takes_bound(&value)
/// ```
/// # Implementation Example
/// Implementing `BorrowRefMut` for RefCell:
/// ``` ignore
/// use std::cell::{ RefMut, RefCell };
/// use borrow_trait::{ BorrowRefMut };
///
/// impl<'a, T: 'a> BorrowRefMut<'a> for RefCell<T> {
///     type Target = T;
///     type Pointer = RefMut<'a, Self::Target>;
///
///     fn borrow_mut(&'a self) -> Self::Pointer { RefCell::borrow_mut(self) }
/// }
/// ```
pub trait BorrowRefMut<'a> {
    /// The type, that is wrapped by the implementation.
    /// # Example
    /// A `RefCell<T>` wraps around `T` therefore `Target` has to be `T`
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
    type Pointer: 'a + DerefMut<Target = Self::Target>;

    /// Mutably borrows the wrapped value.
    /// The value cannot be borrowed while this borrow is active.
    /// # Panics
    /// The function should panic, if the value is currently borrowed.
    /// # Example
    /// ```
    /// use std::ops::DerefMut;
    /// use std::cell::RefCell;
    /// use borrow_trait::{ BorrowRefMut };
    ///
    /// fn takes_bound<T>(value: &T)
    /// where
    ///     T: for<'a> BorrowRefMut<'a, Target = String>,
    /// {
    ///     assert_eq!(value.borrow_mut().deref_mut(), &mut "Hello World".to_string());
    /// }
    ///
    /// let value = RefCell::new("Hello World".to_string());
    /// takes_bound(&value)
    /// ```
    /// An example of panic:
    /// ``` ignore
    /// use std::ops::DerefMut;
    /// use std::cell::RefCell;
    /// use borrow_trait::{ BorrowRefMut };
    /// use std::panic;
    ///
    /// fn takes_bound<T>(value: &T)
    /// where
    ///     T: for<'a> BorrowRefMut<'a, Target = String>,
    /// {
    ///     let result = panic::catch_unwind(move || {
    ///         let mut first_borrow = value.borrow_mut();
    ///         let mut second_borrow = value.borrow_mut(); // this causes a panic
    ///
    ///         assert_eq!(first_borrow.deref_mut(), &mut "Hello World".to_string());
    ///         assert_eq!(second_borrow.deref_mut(), &mut "Hello World".to_string());
    ///     });
    ///     assert!(result.is_err());
    /// }
    ///
    /// let value = RefCell::new("Hello World".to_string());
    /// takes_bound(&value)
    /// ```
    fn borrow_mut(&'a self) -> Self::Pointer;
}

macro_rules! borrow_ref_mut {
    (
        $( $pointer:ty => $body:path => $( $name:ty ),* );*
        $(;)* // <- allows to have a trailing semi-colon
    ) => {
        $(
            $(
                impl<'a, T: 'a> BorrowRefMut<'a> for $name {
                    type Target = T;
                    type Pointer = $pointer;

                    #[inline]
                    fn borrow_mut(&'a self) -> Self::Pointer { $body(self) }
                }
            )* // repeat for each value, seperated by ','
        )* // repeat for each line, seperated by ';'
    }
}

borrow_ref_mut![
    RefMut<'a, T> => RefCell::borrow_mut => RefCell<T>, &RefCell<T>, &mut RefCell<T>;
];

// AtomicRefCell
#[cfg(feature = "atomic_refcell")]
use atomic_refcell::{AtomicRefCell, AtomicRefMut};
#[cfg(feature = "atomic_refcell")]
borrow_ref_mut![
    AtomicRefMut<'a, T> =>
    AtomicRefCell::borrow_mut =>
    AtomicRefCell<T>, &AtomicRefCell<T>, &mut AtomicRefCell<T>;
];

// Cell
#[cfg(feature = "cell")]
use cell;
#[cfg(feature = "cell")]
borrow_ref_mut![
    cell::RefMut<'a, T> =>
    cell::RefCell::borrow_mut => cell::RefCell<T>, &cell::RefCell<T>, &mut cell::RefCell<T>;
];
