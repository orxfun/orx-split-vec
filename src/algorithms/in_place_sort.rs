use crate::Fragment;
use core::cmp::Ordering::{self, *};

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
            let pa = core::ptr::addr_of_mut!(fragments[r][c]);
            let pb = core::ptr::addr_of_mut!(fragments[target_row][0]);
            // SAFETY: `pa` and `pb` have been created from safe mutable references and refer
            // to elements in the slice and therefore are guaranteed to be valid and aligned.
            // Note that accessing the elements behind `a` and `b` is checked and will
            // panic when out of bounds.
            unsafe { core::ptr::swap(pa, pb) };

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

pub(super) fn find_position_to_insert<T, F>(
    fragment: &[T],
    compare: &mut F,
    value: &T,
) -> Option<usize>
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
