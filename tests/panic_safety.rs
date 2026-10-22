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
            panic!("Elem::drop panics"); // once
        }
    }
}

enum ShrinkOp {
    Clear,
    Truncate,
    Remove,
}

#[test_matrix([ShrinkOp::Clear, ShrinkOp::Truncate])]
fn panic_in_clear_safety_issue_112(shrink_op: ShrinkOp) {
    let mut v: SplitVec<Elem, Linear> = SplitVec::with_linear_growth(2); // fragments of 4
    for i in 0..10 {
        v.push(Elem(Box::new(i)));
    }

    PANIC_ON_DROP.with(|f| f.set(true));
    let _ = catch_unwind(AssertUnwindSafe(|| match shrink_op {
        ShrinkOp::Clear => v.clear(),
        ShrinkOp::Truncate => v.truncate(1),
        ShrinkOp::Remove => _ = v.remove(9),
    }));
    println!("len() = {}, fragments = {}", v.len(), v.fragments().len()); // len() is still 10

    if let Some(e) = v.get_mut(9) {
        *e = Elem(Box::new(7)) // index 8 maps to a fragment that was dropped
    }
}
