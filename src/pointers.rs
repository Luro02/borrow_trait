use crate::{BorrowRef, BorrowRefMut};
use alloc::rc::Rc;
use alloc::sync::Arc;

// TODO: better name?
macro_rules! pointer_trait {
    ( $( $name:ty ),* ) => {
        $(
            impl<'a, K: 'a, T: 'a> BorrowRefMut<'a> for $name
            where
                T: BorrowRefMut<'a, Target = K>,
            {
                type Target = K;
                type Pointer = <T as BorrowRefMut<'a>>::Pointer;

                fn borrow_mut(&'a self) -> Self::Pointer { self.as_ref().borrow_mut() }
            }

            impl<'a, K: 'a, T: 'a> BorrowRef<'a> for $name
            where
                T: BorrowRef<'a, Target = K>,
            {
                type Target = K;
                type Pointer = <T as BorrowRef<'a>>::Pointer;

                fn borrow(&'a self) -> Self::Pointer { self.as_ref().borrow() }
            }
        )*
    };
}

pointer_trait![
    Rc<T>,
    &Rc<T>,
    &mut Rc<T>,
    Arc<T>,
    &Arc<T>,
    &mut Arc<T>,
    Box<T>,
    &Box<T>,
    &mut Box<T>
];
