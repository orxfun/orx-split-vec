use crate::{Fragment, Growth, Linear};

#[test]
fn new_cap() {
    fn new_fra() -> Fragment<usize> {
        Vec::<usize>::with_capacity(10).into()
    }

    let growth = Linear::new(4);
    assert_eq!(16, growth.new_fragment_capacity(&[new_fra()]));
    assert_eq!(16, growth.new_fragment_capacity(&[new_fra(), new_fra()]));
    assert_eq!(
        16,
        growth.new_fragment_capacity(&[new_fra(), new_fra(), new_fra()])
    );
}

#[test]
fn indices_dont_panic_when_fragments_is_empty() {
    let linear = Linear::new(4);
    assert_eq!(
        None,
        <Linear as Growth>::get_fragment_and_inner_indices::<usize>(&linear, 0, &[], 0)
    );
}

#[test]
fn indices() {
    fn new_full() -> Fragment<usize> {
        (0..8).collect::<Vec<_>>().into()
    }
    fn new_half() -> Fragment<usize> {
        let mut vec = Vec::with_capacity(8);
        for i in 0..4 {
            vec.push(8 + i);
        }
        vec.into()
    }

    let growth = Linear::new(3);

    for i in 0..8 {
        assert_eq!(
            Some((0, i)),
            growth.get_fragment_and_inner_indices(8, &[new_full()], i)
        );
    }
    assert_eq!(
        None,
        growth.get_fragment_and_inner_indices(8, &[new_full()], 8)
    );

    for i in 0..8 {
        assert_eq!(
            Some((0, i)),
            growth.get_fragment_and_inner_indices(12, &[new_full(), new_half()], i)
        );
    }
    for i in 8..12 {
        assert_eq!(
            Some((1, i - 8)),
            growth.get_fragment_and_inner_indices(12, &[new_full(), new_half()], i)
        );
    }
    assert_eq!(
        None,
        growth.get_fragment_and_inner_indices(12, &[new_full(), new_half()], 12)
    );
}
