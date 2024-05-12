use crate::Fragment;
use std::cmp::Ordering;

pub fn binary_search_by<T, F>(fragments: &[Fragment<T>], mut compare: F) -> Result<usize, usize>
where
    F: FnMut(&T) -> Ordering,
{
    let mut f = 0;
    let mut fragment_begin_idx = 0;

    while let Some(fragment) = fragments.get(f) {
        let result = fragment.binary_search_by(&mut compare);

        match result {
            Ok(idx_in_fragment) => {
                let idx = fragment_begin_idx + idx_in_fragment;
                return Ok(idx);
            }
            Err(idx_in_fragment) => match idx_in_fragment {
                x if x == fragment.len() => {}
                _ => {
                    let idx = fragment_begin_idx + idx_in_fragment;
                    return Err(idx);
                }
            },
        }

        fragment_begin_idx += fragment.len();
        f += 1;
    }

    Err(fragment_begin_idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_all_growth_types, Growth, SplitVec};
    use orx_pinned_vec::PinnedVec;

    fn get_compare(value: usize) -> impl FnMut(&usize) -> Ordering {
        move |x: &usize| x.cmp(&value)
    }

    #[test]
    fn bin_search_empty() {
        let cmp = get_compare(42);

        let fragments = vec![];
        let result = binary_search_by(&fragments, cmp);
        assert_eq!(result, Err(0));
    }

    #[test]
    fn bin_search_empty_first_fragment() {
        let cmp = get_compare(42);

        let fragments = vec![vec![].into()];
        let result = binary_search_by(&fragments, cmp);
        assert_eq!(result, Err(0));
    }

    #[test]
    fn bin_search_empty_second_fragment() {
        let fragments = vec![vec![1, 4, 5].into(), vec![].into()];

        let result = binary_search_by(&fragments, get_compare(0));
        assert_eq!(result, Err(0));

        let result = binary_search_by(&fragments, get_compare(2));
        assert_eq!(result, Err(1));

        let result = binary_search_by(&fragments, get_compare(42));
        assert_eq!(result, Err(3));

        let result = binary_search_by(&fragments, get_compare(1));
        assert_eq!(result, Ok(0));

        let result = binary_search_by(&fragments, get_compare(4));
        assert_eq!(result, Ok(1));

        let result = binary_search_by(&fragments, get_compare(5));
        assert_eq!(result, Ok(2));
    }

    #[test]
    fn bin_search_three_fragments() {
        let fragments = vec![vec![1, 4, 5].into(), vec![7].into(), vec![9, 10].into()];

        let search = |x| binary_search_by(&fragments, get_compare(x));

        assert_eq!(search(0), Err(0));
        assert_eq!(search(1), Ok(0));
        assert_eq!(search(2), Err(1));
        assert_eq!(search(3), Err(1));
        assert_eq!(search(4), Ok(1));
        assert_eq!(search(5), Ok(2));
        assert_eq!(search(6), Err(3));
        assert_eq!(search(7), Ok(3));
        assert_eq!(search(8), Err(4));
        assert_eq!(search(9), Ok(4));
        assert_eq!(search(10), Ok(5));
        assert_eq!(search(11), Err(6));
    }

    #[test]
    fn bin_search_randomized() {
        use rand::prelude::*;
        use rand_chacha::ChaCha8Rng;

        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            let mut rng = ChaCha8Rng::seed_from_u64(8654);
            let mut ref_vec = vec![];
            let mut idx = 0;
            while ref_vec.len() < 1033 {
                if rng.gen::<f32>() < 0.85 {
                    ref_vec.push(idx);
                    vec.push(idx);
                }
                idx += 1;
            }

            for i in 0..(idx + 10) {
                assert_eq!(
                    vec.binary_search_by(|x| x.cmp(&i)),
                    ref_vec.binary_search_by(|x| x.cmp(&i)),
                );
                assert_eq!(vec.binary_search(&i), ref_vec.binary_search(&i));
                assert_eq!(
                    vec.binary_search_by_key(&(2 * i), |x| 2 * x),
                    ref_vec.binary_search_by_key(&(2 * i), |x| 2 * x),
                );
            }
        }
        test_all_growth_types!(test);
    }
}
