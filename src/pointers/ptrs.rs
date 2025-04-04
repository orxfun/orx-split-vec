use crate::{
    Fragment,
    pointers::{Ptr, PtrBackward},
};

#[derive(Copy)]
pub struct Ptrs<T> {
    begin: *const Fragment<T>,
    num_fragments: usize,
}

impl<T> Clone for Ptrs<T> {
    fn clone(&self) -> Self {
        Self {
            begin: self.begin,
            num_fragments: self.num_fragments,
        }
    }
}

impl<'a, T> From<&'a [Fragment<T>]> for Ptrs<T> {
    fn from(value: &'a [Fragment<T>]) -> Self {
        match value.len() {
            0 => Self {
                begin: core::ptr::null(),
                num_fragments: 0,
            },
            num_fragments => Self {
                begin: value.as_ptr(),
                num_fragments,
            },
        }
    }
}

impl<T> Ptrs<T> {
    pub unsafe fn get(&self, f: usize) -> Option<Ptr<T>> {
        (f < self.num_fragments).then(|| Ptr::from(unsafe { &*self.begin.add(f) }))
    }

    pub unsafe fn get_bwd(&self, f: usize) -> Option<PtrBackward<T>> {
        (f < self.num_fragments).then(|| PtrBackward::from(unsafe { &*self.begin.add(f) }))
    }
}

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
        let ptrs = Ptrs::from(fragments.as_slice());

        for i in 0..10 {
            assert!(unsafe { ptrs.get(i) }.is_none());
        }
    }

    #[test]
    fn ptr_one_empty_fragment() {
        let fragment: Fragment<String> = Vec::with_capacity(2).into();
        let mut fragments: Vec<Fragment<String>> = Vec::with_capacity(4);
        fragments.push(fragment);

        let ptrs = Ptrs::from(fragments.as_slice());
        let mut ptr = unsafe { ptrs.get(0) }.unwrap();
        assert_eq!(ptr.next(), None);
        assert_eq!(ptr.next(), None);
    }

    #[test]
    fn ptr_one_non_empty_fragment() {
        let mut fragment: Fragment<String> = Vec::with_capacity(3).into();
        fragment.push(0.to_string());
        fragment.push(1.to_string());

        let mut fragments: Vec<Fragment<String>> = Vec::with_capacity(4);
        fragments.push(fragment);

        let ptrs = Ptrs::from(fragments.as_slice());
        let mut ptr = unsafe { ptrs.get(0) }.unwrap();
        for i in 0..2 {
            assert_eq!(ptr.next().map(|p| unsafe { &*p }), Some(&i.to_string()));
        }
        assert_eq!(ptr.next(), None);
        assert_eq!(ptr.next(), None);
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

        let ptrs = Ptrs::from(fragments.as_slice());
        let mut prior = 0;
        for f in 0..fragments.len() {
            let mut ptr = unsafe { ptrs.get(f) }.unwrap();
            for i in 0..fragments[f].len() {
                assert_eq!(
                    ptr.next().map(|p| unsafe { &*p }),
                    Some(&(prior + i).to_string())
                );
            }
            assert_eq!(ptr.next(), None);
            assert_eq!(ptr.next(), None);
            prior += fragments[f].len();
        }
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

        let ptrs = Ptrs::from(fragments.as_slice());
        let mut prior = 0;
        for f in 0..fragments.len() {
            let mut ptr = unsafe { ptrs.get(f) }.unwrap();
            for i in 0..fragments[f].len() {
                assert_eq!(
                    ptr.next().map(|p| unsafe { &*p }),
                    Some(&(prior + i).to_string())
                );
            }
            assert_eq!(ptr.next(), None);
            assert_eq!(ptr.next(), None);
            prior += fragments[f].len();
        }
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

        let ptrs = Ptrs::from(fragments.as_slice());
        let mut prior = 0;
        for f in 0..fragments.len() {
            let mut ptr = unsafe { ptrs.get(f) }.unwrap();
            for i in 0..fragments[f].len() {
                assert_eq!(
                    ptr.next().map(|p| unsafe { &*p }),
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
            assert_eq!(ptr.next(), None);
            assert_eq!(ptr.next(), None);
            prior += fragments[f].len();
        }
    }
}
