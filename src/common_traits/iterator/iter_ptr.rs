use crate::{
    pointers::{Ptr, Ptrs},
    Fragment,
};
use core::iter::FusedIterator;

#[derive(Copy)]
pub struct IterPtr<T> {
    ptrs: Ptrs<T>,
    current_f: usize,
    current: Ptr<T>,
}

impl<T> Clone for IterPtr<T> {
    fn clone(&self) -> Self {
        Self {
            ptrs: self.ptrs.clone(),
            current_f: self.current_f,
            current: self.current.clone(),
        }
    }
}

impl<'a, T> From<&'a [Fragment<T>]> for IterPtr<T> {
    fn from(value: &'a [Fragment<T>]) -> Self {
        let current_f = 0;
        let current = match value.get(current_f) {
            Some(fragment) => Ptr::from(fragment),
            None => Ptr::default(),
        };
        let ptrs = Ptrs::from(value);
        Self {
            ptrs,
            current,
            current_f,
        }
    }
}

impl<T> Iterator for IterPtr<T> {
    type Item = *const T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.next() {
            Some(x) => Some(x),
            None => {
                self.current_f += 1;
                match unsafe { self.ptrs.get(self.current_f) } {
                    Some(ptr) => {
                        self.current = ptr;
                        self.current.next()
                    }
                    None => None,
                }
            }
        }
    }
}

impl<T> FusedIterator for IterPtr<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    #[test]
    fn ptr_zero_fragments() {
        let fragments: Vec<Fragment<String>> = Vec::with_capacity(4);
        let mut iter = IterPtr::from(fragments.as_slice());

        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }

    #[test]
    fn ptr_one_empty_fragment() {
        let fragment: Fragment<String> = Vec::with_capacity(2).into();
        let mut fragments: Vec<Fragment<String>> = Vec::with_capacity(4);
        fragments.push(fragment);

        let mut iter = IterPtr::from(fragments.as_slice());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }

    #[test]
    fn ptr_one_non_empty_fragment() {
        let mut fragment: Fragment<String> = Vec::with_capacity(3).into();
        fragment.push(0.to_string());
        fragment.push(1.to_string());

        let mut fragments: Vec<Fragment<String>> = Vec::with_capacity(4);
        fragments.push(fragment);

        let mut iter = IterPtr::from(fragments.as_slice());
        assert_eq!(iter.next().map(|p| unsafe { &*p }), Some(&0.to_string()));
        assert_eq!(iter.next().map(|p| unsafe { &*p }), Some(&1.to_string()));
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }

    #[test]
    fn ptr_many_non_empty_fragments() {
        let mut fragments: Vec<Fragment<String>> = Vec::with_capacity(4);

        let prior = 0;
        let n = 4;
        let mut fragment: Fragment<String> = Vec::with_capacity(n).into();
        for i in 0..n {
            fragment.push((prior + i).to_string());
        }
        fragments.push(fragment);

        let prior = prior + n;
        let n = 8;
        let mut fragment: Fragment<String> = Vec::with_capacity(n).into();
        for i in 0..n {
            fragment.push((prior + i).to_string());
        }
        fragments.push(fragment);

        let prior = prior + n;
        let n = 16;
        let mut fragment: Fragment<String> = Vec::with_capacity(n).into();
        for i in 0..n {
            fragment.push((prior + i).to_string());
        }
        fragments.push(fragment);

        let mut iter = IterPtr::from(fragments.as_slice());
        let mut prior = 0;
        for f in 0..fragments.len() {
            for i in 0..fragments[f].len() {
                assert_eq!(
                    iter.next().map(|p| unsafe { &*p }),
                    Some(&(prior + i).to_string())
                );
            }
            prior += fragments[f].len();
        }
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_many_non_empty_fragments_half_last() {
        let mut fragments: Vec<Fragment<String>> = Vec::with_capacity(4);

        let prior = 0;
        let n = 4;
        let mut fragment: Fragment<String> = Vec::with_capacity(n).into();
        for i in 0..n {
            fragment.push((prior + i).to_string());
        }
        fragments.push(fragment);

        let prior = prior + n;
        let n = 8;
        let mut fragment: Fragment<String> = Vec::with_capacity(n).into();
        for i in 0..n {
            fragment.push((prior + i).to_string());
        }
        fragments.push(fragment);

        let prior = prior + n;
        let n = 16;
        let mut fragment: Fragment<String> = Vec::with_capacity(n).into();
        for i in 0..(n / 2) {
            fragment.push((prior + i).to_string());
        }
        fragments.push(fragment);

        let mut iter = IterPtr::from(fragments.as_slice());
        let mut prior = 0;
        for f in 0..fragments.len() {
            for i in 0..fragments[f].len() {
                assert_eq!(
                    iter.next().map(|p| unsafe { &*p }),
                    Some(&(prior + i).to_string())
                );
            }
            prior += fragments[f].len();
        }
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_mutate_nodes_while_iterating() {
        let mut fragments: Vec<Fragment<String>> = Vec::with_capacity(4);

        let prior = 0;
        let n = 4;
        let mut fragment: Fragment<String> = Vec::with_capacity(n).into();
        for i in 0..n {
            fragment.push((prior + i).to_string());
        }
        fragments.push(fragment);

        let prior = prior + n;
        let n = 8;
        let mut fragment: Fragment<String> = Vec::with_capacity(n).into();
        for i in 0..n {
            fragment.push((prior + i).to_string());
        }
        fragments.push(fragment);

        let prior = prior + n;
        let n = 16;
        let mut fragment: Fragment<String> = Vec::with_capacity(n).into();
        for i in 0..(n / 2) {
            fragment.push((prior + i).to_string());
        }
        fragments.push(fragment);

        let mut iter = IterPtr::from(fragments.as_slice());
        let mut prior = 0;
        for f in 0..fragments.len() {
            for i in 0..fragments[f].len() {
                assert_eq!(
                    iter.next().map(|p| unsafe { &*p }),
                    Some(&(prior + i).to_string())
                );

                if f > 0 {
                    let first = {
                        let f = unsafe { &*fragments.as_ptr().add(f - 1) };
                        let p = unsafe { f.as_ptr().add(0) } as *mut String;
                        p
                    };
                    let second = {
                        let f = unsafe { &*fragments.as_ptr().add(f) };
                        let p = unsafe { f.as_ptr().add(i) } as *mut String;
                        p
                    };
                    unsafe { first.swap(second) };
                }

                // fragments[f][i] = 0.to_string(); => miri error!!! but below is OK
                let f = unsafe { &*fragments.as_ptr().add(f) };
                let p = unsafe { f.as_ptr().add(i) } as *mut String;
                let mut new_str = 0.to_string();
                let mut_ref = &mut new_str;
                let mut_ptr = mut_ref as *mut String;
                unsafe { p.swap(mut_ptr) };
            }
            prior += fragments[f].len();
        }
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
