use crate::Fragment;

pub(crate) fn get_fragment_and_inner_indices<T>(
    fragments: &[Fragment<T>],
    element_index: usize,
) -> Option<(usize, usize)> {
    let mut prev_end = 0;
    let mut end = 0;
    for (f, fragment) in fragments.iter().enumerate() {
        end += fragment.len();
        if element_index < end {
            return Some((f, element_index - prev_end));
        }
        prev_end = end;
    }
    None
}
