use orx_split_vec::*;
use test_case::test_matrix;

fn num_moves() -> usize {
    #[cfg(not(miri))]
    return 2500;

    #[cfg(miri)]
    return 21;
}

/// Takes ownership of the `vec`.
///
/// Pushes `growth_len` elements to it.
///
/// Right after pushing an element, element's memory location is
/// stored in the `pointers` vector.
///
/// Finally, returns back the vector.
fn grow<G: Growth>(
    vec: SplitVec<String, G>,
    growth_len: usize,
    immediate_ptrs: &mut Vec<*const String>,
) -> SplitVec<String, G> {
    let mut vec = std::hint::black_box(vec);

    let n = vec.len();
    for i in n..(n + growth_len) {
        vec.push(i.to_string());
        immediate_ptrs.push(vec.get_ptr(i).unwrap());
    }

    std::hint::black_box(vec)
}

/// Tests whether or not the vector contains expected elements,
/// which are "0", "1", ..., "n-1".
///
/// This is just a basic validation.
/// It must pass regardless of whether or not the elements
/// are moved in memory during the growth.
fn check_current_values(vec: &SplitVec<String, impl Growth>) {
    for i in 0..vec.len() {
        assert_eq!(&vec[i], &i.to_string());
    }
}

/// Tests validity of immediate pointers.
///
/// By immediate pointer, we mean that the memory address of each
/// element is taken and cached as soon as the element is pushed
/// to the vector.
///
/// We will read the value from each pointer and compare it to the
/// expected value.
///
/// It is possible that this test passes even if the elements are
/// moved in memory in a normal execution. However, it must fail
/// when executed with miri.
fn validate_immediate_pointers(immediate_ptrs: &Vec<*const String>) {
    for (i, ptr) in immediate_ptrs.iter().copied().enumerate() {
        let elem = unsafe { &*ptr };
        assert_eq!(elem, &i.to_string());
    }
}

/// Compares current and immediate pointers:
/// * immediate pointer is the memory location of an element which
///   is taken and cached immediately after it is pushed to the
///   vector.
/// * current pointer is the memory location of an element in the
///   vector that will be taken now after the moves have been
///   simulated.
///
/// Immediate and current pointer of each element will be the same
/// iff the memory locations of elements remained intact.
fn compare_current_and_immediate_pointers(
    vec: &SplitVec<String, impl Growth>,
    immediate_ptrs: &Vec<*const String>,
) {
    for i in 0..vec.len() {
        let initial_ptr = immediate_ptrs[i];
        let current_ptr = vec.get_ptr(i).unwrap();
        assert_eq!(initial_ptr, current_ptr);
    }
}

#[test_matrix([
    SplitVec::with_doubling_growth(),
    SplitVec::with_linear_growth(10),
    SplitVec::with_recursive_growth(),
])]
fn elements_pinned_after_move<G: Growth>(vec: SplitVec<String, G>) {
    assert!(vec.is_empty());
    let mut immediate_ptrs = vec![];

    let num_moves = num_moves();
    let len = num_moves * (num_moves - 1) / 2;

    let mut vec = vec;
    for len in 0..num_moves {
        vec = grow(vec, len, &mut immediate_ptrs);
    }

    assert_eq!(vec.len(), len);
    assert_eq!(immediate_ptrs.len(), len);

    check_current_values(&vec);

    validate_immediate_pointers(&immediate_ptrs);

    compare_current_and_immediate_pointers(&vec, &immediate_ptrs);
}
