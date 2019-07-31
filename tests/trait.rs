use std::cell::RefCell;
use std::io::{Cursor, Read};
use std::rc::Rc;

use borrow_trait::BorrowRefMut;

fn takes_bound<'a, C, T>(value: &'a T) -> Vec<u8>
where
    T: BorrowRefMut<'a, Target = C>,
    C: Read,
{
    let mut result = vec![];
    value
        .borrow_mut()
        .read_to_end(&mut result)
        .expect("Failed to read from `value: T`");
    result
}

#[test]
fn refcell() {
    let value = RefCell::new(Cursor::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    assert_eq!(takes_bound(&value), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn rc_refcell() {
    let value = Rc::new(RefCell::new(Cursor::new(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    ])));
    assert_eq!(takes_bound(&value), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
