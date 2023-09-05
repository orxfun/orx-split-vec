use crate::Iter;

impl<'a, T: PartialEq> PartialEq for Iter<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        let iter1 = self.clone();
        let mut iter2 = other.clone();
        for x1 in iter1 {
            match iter2.next() {
                None => return false,
                Some(x2) => {
                    if x1 != x2 {
                        return false;
                    }
                }
            }
        }
        iter2.next().is_none()
    }
}

impl<'a, T: PartialEq> PartialEq<Iter<'a, T>> for [T] {
    fn eq(&self, other: &Iter<'a, T>) -> bool {
        is_iter_eq_to_slice(other, self)
    }
}
impl<'a, T: PartialEq> PartialEq<Iter<'a, T>> for Vec<T> {
    fn eq(&self, other: &Iter<'a, T>) -> bool {
        is_iter_eq_to_slice(other, self)
    }
}
impl<'a, T: PartialEq, const N: usize> PartialEq<Iter<'a, T>> for [T; N] {
    fn eq(&self, other: &Iter<'a, T>) -> bool {
        is_iter_eq_to_slice(other, self)
    }
}

fn is_iter_eq_to_slice<T: PartialEq>(iter: &Iter<T>, slice: &[T]) -> bool {
    let mut slice_beg = 0;

    let mut fragments = iter.fragments.iter().skip(iter.f);

    if let Some(fragment) = fragments.next() {
        let slice_end = slice_beg + fragment.len() - iter.i;
        let slice_of_slice = &slice[slice_beg..slice_end];
        if fragment.data != slice_of_slice {
            return false;
        }
        slice_beg = slice_end;

        for fragment in fragments {
            let slice_end = slice_beg + fragment.len();
            let slice_of_slice = &slice[slice_beg..slice_end];
            if fragment.data != slice_of_slice {
                return false;
            }
            slice_beg = slice_end;
        }

        true
    } else {
        slice.is_empty()
    }
}
