use crate::Fragment;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub(crate) struct FragmentIter<'a, T> {
    fragment: &'a [T],
    i: usize,
    phantom: PhantomData<&'a ()>,
}

impl<'a, T> FragmentIter<'a, T> {
    pub(crate) fn new(fragment: &'a Fragment<T>, len: usize) -> Self {
        let fragment = unsafe { std::slice::from_raw_parts(fragment.as_ptr(), len) };
        Self {
            fragment,
            i: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a> Iterator for FragmentIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.i < self.fragment.len() {
            true => {
                let element = unsafe { self.fragment.get_unchecked(self.i) };
                self.i += 1;
                Some(element)
            }
            false => None,
        }
    }
}
