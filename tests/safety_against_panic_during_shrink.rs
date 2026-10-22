use orx_split_vec::*;
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use test_case::test_matrix;

thread_local!(static PANIC_ON_DROP: Cell<bool> = Cell::new(false));

#[allow(dead_code)]
struct Elem(Box<u8>);

impl Drop for Elem {
    fn drop(&mut self) {
        if PANIC_ON_DROP.with(|f| f.replace(false)) {
            assert_eq!(3, 4);
            panic!("Elem::drop panics"); // once
        }
    }
}

enum ShrinkOp {
    Clear,
    Truncate,
    Remove,
    Pop,
}

#[test_matrix([
    (ShrinkOp::Clear, 0),
    (ShrinkOp::Truncate, 1),
    (ShrinkOp::Remove, 9),
    (ShrinkOp::Pop, 9),
])]
/// This test is created against potential safety issues when shrinking methods panics
/// while dropping the removed elements. See the detailed explanation on the corresponding
/// [issue](https://github.com/orxfun/orx-split-vec/issues/112).
fn safety_against_panic_during_shrink((shrink_op, expected_len): (ShrinkOp, usize)) {
    let mut v: SplitVec<Elem, Linear> = SplitVec::with_linear_growth(2); // fragments of 4
    for i in 0..10 {
        v.push(Elem(Box::new(i)));
    }

    PANIC_ON_DROP.with(|f| f.set(true));
    let _ = catch_unwind(AssertUnwindSafe(|| match shrink_op {
        ShrinkOp::Clear => v.clear(),
        ShrinkOp::Truncate => v.truncate(1),
        ShrinkOp::Remove => _ = v.remove(4),
        ShrinkOp::Pop => _ = v.pop(),
    }));
    println!("len() = {}, fragments = {}", v.len(), v.fragments().len()); // len() is still 10

    for i in expected_len..10 {
        assert!(
            v.get_mut(i).is_none(),
            "The last element is dropped, we should not have access to this element"
        );
    }

    assert_eq!(v.len(), expected_len);
}
