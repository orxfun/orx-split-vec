use super::iter::Iter;

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
