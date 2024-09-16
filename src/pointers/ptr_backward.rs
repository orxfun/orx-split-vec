use crate::Fragment;
use core::iter::FusedIterator;

#[derive(PartialEq, Eq, Debug, Copy)]
pub struct PtrBackward<T> {
    current: Option<*const T>,
    stopper: *const T,
}

impl<T> Default for PtrBackward<T> {
    fn default() -> Self {
        Self {
            current: None,
            stopper: core::ptr::null(),
        }
    }
}

impl<T> Clone for PtrBackward<T> {
    fn clone(&self) -> Self {
        Self {
            current: self.current,
            stopper: self.stopper,
        }
    }
}

impl<'a, T> From<&'a Fragment<T>> for PtrBackward<T> {
    fn from(value: &'a Fragment<T>) -> Self {
        match value.len() {
            0 => Self::default(),
            _ => {
                let stopper = value.as_ptr();
                let current = Some(unsafe { value.as_ptr().add(value.len() - 1) });
                Self { current, stopper }
            }
        }
    }
}

impl<T> Iterator for PtrBackward<T> {
    type Item = *const T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.inspect(|&p| match p == self.stopper {
            false => self.current = Some(unsafe { p.sub(1) }),
            true => self.current = None,
        })
    }
}

impl<T> FusedIterator for PtrBackward<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    #[test]
    fn ptr_bwd_empty_fragment() {
        let fragment: Fragment<String> = Vec::with_capacity(0).into();

        let mut iter = PtrBackward::from(&fragment);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_bwd_half_full_fragment() {
        let mut fragment: Fragment<String> = Vec::with_capacity(14).into();
        for i in 0..9 {
            fragment.push(i.to_string());
        }

        let mut iter = PtrBackward::from(&fragment);
        for i in (0..9).rev() {
            let val = iter.next().map(|p| unsafe { &*p });
            assert_eq!(val, Some(&i.to_string()));
        }

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_bwd_full_fragment() {
        let mut fragment: Fragment<String> = Vec::with_capacity(14).into();
        for i in 0..14 {
            fragment.push(i.to_string());
        }

        let mut iter = PtrBackward::from(&fragment);
        for i in (0..14).rev() {
            let val = iter.next().map(|p| unsafe { &*p });
            assert_eq!(val, Some(&i.to_string()));
        }

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
