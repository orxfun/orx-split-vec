use crate::Fragment;
use core::iter::FusedIterator;

#[derive(PartialEq, Eq, Debug, Copy)]
pub struct Ptr<T> {
    current: *const T,
    stopper: *const T,
}

impl<T> Default for Ptr<T> {
    fn default() -> Self {
        Self {
            current: core::ptr::null(),
            stopper: core::ptr::null(),
        }
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self {
            current: self.current,
            stopper: self.stopper,
        }
    }
}

impl<'a, T> From<&'a Fragment<T>> for Ptr<T> {
    fn from(value: &'a Fragment<T>) -> Self {
        match value.len() {
            0 => Self::default(),
            _ => {
                let current = value.as_ptr();
                let stopper = unsafe { current.add(value.len()) };
                Self { current, stopper }
            }
        }
    }
}

impl<T> Iterator for Ptr<T> {
    type Item = *const T;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.current;
        match p == self.stopper {
            false => {
                self.current = unsafe { self.current.add(1) };
                Some(p)
            }
            true => None,
        }
    }
}

impl<T> FusedIterator for Ptr<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    #[test]
    fn ptr_empty_fragment() {
        let fragment: Fragment<String> = Vec::with_capacity(0).into();

        let mut iter = Ptr::from(&fragment);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_half_full_fragment() {
        let mut fragment: Fragment<String> = Vec::with_capacity(14).into();
        for i in 0..9 {
            fragment.push(i.to_string());
        }

        let mut iter = Ptr::from(&fragment);
        for i in 0..9 {
            let val = iter.next().map(|p| unsafe { &*p });
            assert_eq!(val, Some(&i.to_string()));
        }

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_full_fragment() {
        let mut fragment: Fragment<String> = Vec::with_capacity(14).into();
        for i in 0..14 {
            fragment.push(i.to_string());
        }

        let mut iter = Ptr::from(&fragment);
        for i in 0..14 {
            let val = iter.next().map(|p| unsafe { &*p });
            assert_eq!(val, Some(&i.to_string()));
        }

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
