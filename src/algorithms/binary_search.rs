use crate::Fragment;
use core::cmp::Ordering;

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
