use crate::{
    Fragment,
    pointers::{PtrBackward, Ptrs},
};
use core::iter::FusedIterator;

#[derive(Copy)]
pub struct IterPtrBackward<T> {
    ptrs: Ptrs<T>,
    current_f: usize,
    current: PtrBackward<T>,
}

impl<T> Clone for IterPtrBackward<T> {
    fn clone(&self) -> Self {
        Self {
            ptrs: self.ptrs.clone(),
            current_f: self.current_f,
            current: self.current.clone(),
        }
    }
}

impl<'a, T> From<&'a [Fragment<T>]> for IterPtrBackward<T> {
    fn from(value: &'a [Fragment<T>]) -> Self {
        let current_f = value.len().saturating_sub(1);
        let current = match value.last() {
            Some(fragment) => PtrBackward::from(fragment),
            None => PtrBackward::default(),
        };
        let ptrs = Ptrs::from(value);
        Self {
            ptrs,
            current,
            current_f,
        }
    }
}

impl<T> Iterator for IterPtrBackward<T> {
    type Item = *const T;

    #[allow(clippy::unwrap_in_result)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.current.next() {
            Some(x) => Some(x),
            None => match self.current_f {
                0 => None,
                x => {
                    self.current_f = x - 1;
                    let ptr = unsafe { self.ptrs.get_bwd(self.current_f) }.expect("exists");
                    self.current = ptr;
                    self.current.next()
                }
            },
        }
    }
}

impl<T> FusedIterator for IterPtrBackward<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    #[test]
    fn ptr_bwd_zero_fragments() {
        let fragments: Vec<Fragment<String>> = Vec::with_capacity(4);
        let mut iter = IterPtrBackward::from(fragments.as_slice());

        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }

    #[test]
    fn ptr_bwd_one_empty_fragment() {
        let fragment: Fragment<String> = Vec::with_capacity(2).into();
        let mut fragments: Vec<Fragment<String>> = Vec::with_capacity(4);
        fragments.push(fragment);

        let mut iter = IterPtrBackward::from(fragments.as_slice());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }

    #[test]
    fn ptr_bwd_one_non_empty_fragment() {
        let mut fragment: Fragment<String> = Vec::with_capacity(3).into();
        fragment.push(0.to_string());
        fragment.push(1.to_string());

        let mut fragments: Vec<Fragment<String>> = Vec::with_capacity(4);
        fragments.push(fragment);

        let mut iter = IterPtrBackward::from(fragments.as_slice());
        assert_eq!(iter.next().map(|p| unsafe { &*p }), Some(&1.to_string()));
        assert_eq!(iter.next().map(|p| unsafe { &*p }), Some(&0.to_string()));
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }

    #[test]
    fn ptr_bwd_many_non_empty_fragments() {
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

        let mut iter = IterPtrBackward::from(fragments.as_slice());
        let mut value = 4 + 8 + 16 - 1;
        for f in 0..fragments.len() {
            for _ in 0..fragments[f].len() {
                assert_eq!(
                    iter.next().map(|p| unsafe { &*p }),
                    Some(&value.to_string())
                );
                value -= 1;
            }
        }
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_bwd_many_non_empty_fragments_half_last() {
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

        let mut iter = IterPtrBackward::from(fragments.as_slice());
        let mut value = 4 + 8 + 8 - 1;
        for f in 0..fragments.len() {
            for _ in 0..fragments[f].len() {
                assert_eq!(
                    iter.next().map(|p| unsafe { &*p }),
                    Some(&value.to_string())
                );
                value -= 1;
            }
        }
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ptr_bwd_mutate_nodes_while_iterating() {
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

        let mut iter = IterPtrBackward::from(fragments.as_slice());
        let mut value = Some(0.to_string());
        for f in 0..fragments.len() {
            for i in 0..fragments[f].len() {
                value = iter.next().map(|p| unsafe { &*p }).cloned();

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
        }
        assert!(value.is_some());
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
