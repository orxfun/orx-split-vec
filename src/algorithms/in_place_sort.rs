use crate::Fragment;
use std::cmp::Ordering::{self, *};

pub fn in_place_sort_by<T, F>(fragments: &mut [Fragment<T>], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if fragments.is_empty() {
        return;
    }

    for fragment in fragments.iter_mut() {
        fragment.sort_by(&mut compare);
    }

    let num_fragments = fragments.len();

    let mut r = 0;
    let mut c = 0;

    while r < num_fragments - 1 {
        let row_to_swap = get_row_to_swap(fragments, &mut compare, r, c);
        if let Some(target_row) = row_to_swap {
            let pa = std::ptr::addr_of_mut!(fragments[r][c]);
            let pb = std::ptr::addr_of_mut!(fragments[target_row][0]);
            // SAFETY: `pa` and `pb` have been created from safe mutable references and refer
            // to elements in the slice and therefore are guaranteed to be valid and aligned.
            // Note that accessing the elements behind `a` and `b` is checked and will
            // panic when out of bounds.
            unsafe { std::ptr::swap(pa, pb) };

            let fragment_right = &fragments[target_row][1..];
            let value = &fragments[target_row][0];
            let position_to_insert = find_position_to_insert(fragment_right, &mut compare, value);
            if let Some(p) = position_to_insert {
                for i in 0..p {
                    fragments[target_row].swap(i, i + 1);
                }
            }
        }

        match c == fragments[r].len() - 1 {
            true => (r, c) = (r + 1, 0),
            false => c += 1,
        }
    }
}

fn get_row_to_swap<T, F>(
    fragments: &[Fragment<T>],
    compare: &mut F,
    r: usize,
    c: usize,
) -> Option<usize>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut r_best = r + 1;
    match r_best == fragments.len() {
        true => None,
        false => {
            let mut best = &fragments[r_best][0];
            for (q, fragment) in fragments.iter().enumerate().skip(r_best + 1) {
                if let Less = compare(&fragment[0], best) {
                    (best, r_best) = (&fragment[0], q);
                }
            }

            match compare(best, &fragments[r][c]) {
                Less => Some(r_best),
                _ => None,
            }
        }
    }
}

fn find_position_to_insert<T, F>(fragment: &[T], compare: &mut F, value: &T) -> Option<usize>
where
    F: FnMut(&T, &T) -> Ordering,
{
    match fragment.len() {
        0 => None,
        _ => match compare(unsafe { fragment.get_unchecked(0) }, value) {
            Equal | Greater => None,
            Less => {
                let mut target = 0;
                let mut size = fragment.len();
                let mut left = 0;
                let mut right = size;
                while left < right {
                    let mid = left + size / 2;

                    match compare(unsafe { fragment.get_unchecked(mid) }, value) {
                        // Equal => return Some(mid),
                        Greater | Equal => right = mid,
                        Less => {
                            target = mid + 1;
                            left = mid;
                        }
                    }
                    if size == 1 {
                        return Some(target);
                    }
                    size = right - left;
                }

                Some(target)
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Doubling, Growth, Linear, Recursive};
    use test_case::test_case;

    #[test]
    fn insertion_position() {
        let fragment: Fragment<u32> = vec![4, 7, 9, 13, 16, 17, 23].into();

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

        let fragment: Fragment<u32> = vec![4, 7, 13, 13, 13, 17, 23].into();
        let mut pos = |val: &u32| find_position_to_insert(&fragment, &mut c, val);
        assert_eq!(pos(&13), Some(2));

        let fragment: Fragment<u32> = vec![4, 7, 13, 13, 23, 23, 23].into();
        let mut pos = |val: &u32| find_position_to_insert(&fragment, &mut c, val);
        assert_eq!(pos(&23), Some(4));

        let fragment: Fragment<u32> = vec![4, 4, 13, 13, 23, 23, 23].into();
        let mut pos = |val: &u32| find_position_to_insert(&fragment, &mut c, val);
        assert_eq!(pos(&4), None);
    }

    #[test]
    fn sort_simple() {
        let mut c = |a: &u32, b: &u32| a.cmp(b);

        let mut fragments: Vec<Fragment<u32>> =
            vec![vec![2, 4].into(), vec![0, 5, 6].into(), vec![1, 3].into()];

        in_place_sort_by(&mut fragments, &mut c);

        assert_is_sorted(fragments);
    }

    #[test_case(Doubling)]
    #[test_case(Recursive)]
    #[test_case(Linear::new(10))]
    fn sort_growth(growth: impl Growth) {
        let mut c = |a: &i32, b: &i32| a.cmp(b);

        let num_fragments = 10;
        let mut fragments: Vec<Fragment<_>> = vec![];

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
}
