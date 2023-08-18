use crate::SplitVec;

impl<T: PartialEq, U> PartialEq<U> for SplitVec<T>
where
    U: AsRef<[T]>,
{
    fn eq(&self, other: &U) -> bool {
        let other = other.as_ref();
        let mut beg = 0;
        for fragment in &self.fragments {
            let end = beg + fragment.len();
            let other_slice = &other[beg..end];
            if fragment.data != other_slice {
                return false;
            }
            beg = end;
        }
        true
    }
}
impl<T: PartialEq> PartialEq<SplitVec<T>> for SplitVec<T> {
    fn eq(&self, other: &SplitVec<T>) -> bool {
        self.len() == other.len() && self.into_iter().zip(other.into_iter()).all(|(x, y)| x == y)
    }
}
impl<T: PartialEq> Eq for SplitVec<T> {}
