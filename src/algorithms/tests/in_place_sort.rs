use crate::{
    Doubling, Fragment, Growth, Linear, Recursive,
    algorithms::in_place_sort::{find_position_to_insert, in_place_sort_by},
};
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use test_case::test_case;

#[test]
fn insertion_position() {
    let fragment: Fragment<u32> = alloc::vec![4, 7, 9, 13, 16, 17, 23].into();

    let mut c = |a: &u32, b: &u32| a.cmp(b);
    let mut pos = |val: &u32| find_position_to_insert(&fragment, &mut c, val);

    assert_eq!(pos(&0), None);
    assert_eq!(pos(&3), None);
    assert_eq!(pos(&4), None);

    assert_eq!(pos(&5), Some(1));
    assert_eq!(pos(&6), Some(1));
    assert_eq!(pos(&7), Some(1));

    assert_eq!(pos(&8), Some(2));
    assert_eq!(pos(&9), Some(2));

    assert_eq!(pos(&10), Some(3));
    assert_eq!(pos(&11), Some(3));
    assert_eq!(pos(&12), Some(3));
    assert_eq!(pos(&13), Some(3));

    assert_eq!(pos(&14), Some(4));
    assert_eq!(pos(&15), Some(4));
    assert_eq!(pos(&16), Some(4));

    assert_eq!(pos(&17), Some(5));

    assert_eq!(pos(&18), Some(6));
    assert_eq!(pos(&19), Some(6));
    assert_eq!(pos(&20), Some(6));
    assert_eq!(pos(&21), Some(6));
    assert_eq!(pos(&22), Some(6));
    assert_eq!(pos(&23), Some(6));

    assert_eq!(pos(&24), Some(7));
    assert_eq!(pos(&25), Some(7));
    assert_eq!(pos(&26), Some(7));
    assert_eq!(pos(&100), Some(7));
}

#[test]
fn insertion_position_with_ties() {
    let mut c = |a: &u32, b: &u32| a.cmp(b);

    let fragment: Fragment<u32> = alloc::vec![4, 7, 13, 13, 13, 17, 23].into();
    let mut pos = |val: &u32| find_position_to_insert(&fragment, &mut c, val);
    assert_eq!(pos(&13), Some(2));

    let fragment: Fragment<u32> = alloc::vec![4, 7, 13, 13, 23, 23, 23].into();
    let mut pos = |val: &u32| find_position_to_insert(&fragment, &mut c, val);
    assert_eq!(pos(&23), Some(4));

    let fragment: Fragment<u32> = alloc::vec![4, 4, 13, 13, 23, 23, 23].into();
    let mut pos = |val: &u32| find_position_to_insert(&fragment, &mut c, val);
    assert_eq!(pos(&4), None);
}

#[test]
fn sort_simple() {
    let mut c = |a: &u32, b: &u32| a.cmp(b);

    let mut fragments: Vec<Fragment<u32>> = alloc::vec![
        alloc::vec![2, 4].into(),
        alloc::vec![0, 5, 6].into(),
        alloc::vec![1, 3].into()
    ];

    in_place_sort_by(&mut fragments, &mut c);

    assert_is_sorted(fragments);
}

#[test_case(Doubling)]
#[test_case(Recursive)]
#[test_case(Linear::new(6))]
fn sort_growth(growth: impl Growth) {
    let mut c = |a: &i32, b: &i32| a.cmp(b);

    #[cfg(not(miri))]
    let num_fragments = 10;
    #[cfg(miri)]
    let num_fragments = 3;

    let mut fragments: Vec<Fragment<_>> = alloc::vec![];

    let mut len = 0;
    for _ in 0..num_fragments {
        let fragment_capacities: Vec<_> = fragments.iter().map(|x| x.capacity()).collect();
        let mut fragment =
            Fragment::new(growth.new_fragment_capacity_from(fragment_capacities.into_iter()));
        for i in 0..fragment.capacity() {
            let i = len + i;
            let value = match i % 3 {
                0 => i as i32,
                1 => 42,
                _ => -(i as i32),
            };
            fragment.push(value);
        }

        assert_eq!(fragment.len(), fragment.capacity());
        len += fragment.len();
        fragments.push(fragment);
    }

    assert_eq!(fragments.len(), num_fragments);

    in_place_sort_by(&mut fragments, &mut c);

    assert_is_sorted(fragments);
}

fn assert_is_sorted<T: Ord>(fragments: Vec<Fragment<T>>) {
    let flattened: Vec<T> = fragments.into_iter().flat_map(|x| Vec::from(x)).collect();

    if flattened.is_empty() {
        return;
    }

    let mut curr = &flattened[0];
    for i in 1..flattened.len() {
        let cmp = curr.cmp(&flattened[i]);
        assert!(cmp != Greater);
        curr = &flattened[i];
    }
}
