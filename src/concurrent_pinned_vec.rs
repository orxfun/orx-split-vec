use crate::{
    common_traits::iterator::iter_con::IterCon,
    fragment::fragment_struct::{
        maximum_concurrent_capacity, num_fragments_for_capacity, set_fragments_len,
    },
    range_helpers::{range_end, range_start},
    Doubling, Fragment, Growth, GrowthWithConstantTimeAccess, SplitVec,
};
use orx_pinned_vec::{ConcurrentPinnedVec, PinnedVec};
use std::{
    fmt::Debug,
    ops::RangeBounds,
    sync::atomic::{AtomicUsize, Ordering},
};

/// Concurrent wrapper ([`orx_pinned_vec::ConcurrentPinnedVec`]) for the `SplitVec`.
pub struct ConcurrentSplitVec<T, G: GrowthWithConstantTimeAccess = Doubling> {
    capacity: AtomicUsize,
    maximum_capacity: usize,
    num_fragments: AtomicUsize,
    growth: G,
    fragments: Vec<Fragment<T>>,
    ptr_fragments: *mut Fragment<T>,
    fragment_pointers: Vec<*const T>,
    ptr_fragments_pointers: *const *const T,
}

impl<T, G: GrowthWithConstantTimeAccess + Debug> Debug for ConcurrentSplitVec<T, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConcurrentSplitVec")
            .field("capacity", &self.capacity)
            .field("maximum_capacity", &self.maximum_capacity)
            .field("num_fragments", &self.num_fragments)
            .field("growth", &self.growth)
            .finish()
    }
}

impl<T, G: GrowthWithConstantTimeAccess> Drop for ConcurrentSplitVec<T, G> {
    fn drop(&mut self) {
        unsafe { self.fragments.set_len(self.num_fragments()) };
    }
}

impl<T, G: GrowthWithConstantTimeAccess> ConcurrentSplitVec<T, G> {
    fn num_fragments(&self) -> usize {
        self.num_fragments.load(Ordering::Relaxed)
    }

    fn fragments(&self) -> &[Fragment<T>] {
        let len = self.num_fragments();
        self.fragments_for(len)
    }

    fn fragments_for(&self, num_fragments: usize) -> &[Fragment<T>] {
        unsafe { std::slice::from_raw_parts(self.ptr_fragments, num_fragments) }
    }

    fn push_fragment(&self, fragment: Fragment<T>, fragment_index: usize) {
        let p = unsafe { self.fragment_pointers.as_ptr().add(fragment_index) };
        let p = p as *mut *const T;
        unsafe { p.write(fragment.as_ptr()) };
        unsafe { self.ptr_fragments.add(fragment_index).write(fragment) };
    }

    fn fragment_element_ptr_mut(&self, f: usize, i: usize) -> *mut T {
        let p = unsafe { self.ptr_fragments_pointers.add(f).read() };
        let p = unsafe { p.add(i) };
        p as *mut T
    }

    fn fragment_element_ptr(&self, f: usize, i: usize) -> *const T {
        let p = unsafe { self.ptr_fragments_pointers.add(f).read() };
        unsafe { p.add(i) }
    }
}

fn get_pointers<T>(
    fragments: &[Fragment<T>],
    fragments_capacity: usize,
) -> (Vec<*const T>, *const *const T) {
    let first = fragments[0].as_ptr();
    let mut fragment_pointers = vec![first; fragments_capacity];
    for (f, fragment) in fragments.iter().enumerate() {
        fragment_pointers[f] = fragment.as_ptr();
    }
    let ptr_fragments_pointers = fragment_pointers.as_ptr();

    (fragment_pointers, ptr_fragments_pointers)
}

impl<T, G: GrowthWithConstantTimeAccess> From<SplitVec<T, G>> for ConcurrentSplitVec<T, G> {
    fn from(value: SplitVec<T, G>) -> Self {
        let (mut fragments, growth) = (value.fragments, value.growth);

        let data = data(&mut fragments, &growth);

        Self {
            capacity: data.capacity.into(),
            maximum_capacity: data.maximum_capacity,
            num_fragments: data.num_fragments.into(),
            growth,
            fragments,
            ptr_fragments: data.ptr_fragments,
            fragment_pointers: data.fragment_pointers,
            ptr_fragments_pointers: data.ptr_fragments_pointers,
        }
    }
}

impl<T, G: GrowthWithConstantTimeAccess> ConcurrentPinnedVec<T> for ConcurrentSplitVec<T, G> {
    type P = SplitVec<T, G>;

    unsafe fn into_inner(mut self, len: usize) -> Self::P {
        self.fragments.set_len(self.num_fragments());

        let mut fragments = vec![];
        std::mem::swap(&mut fragments, &mut self.fragments);
        set_fragments_len(&mut fragments, len);

        self.num_fragments.store(0, Ordering::Relaxed);
        self.capacity.store(0, Ordering::Relaxed);

        let growth = self.growth.clone();

        // let (mut fragments, growth) = (self.fragments, self.growth.clone());
        SplitVec::from_raw_parts(len, fragments, growth)
    }

    fn capacity(&self) -> usize {
        self.capacity.load(Ordering::SeqCst)
    }

    fn max_capacity(&self) -> usize {
        self.maximum_capacity
    }

    fn grow_to(&self, new_capacity: usize) -> Result<usize, orx_pinned_vec::PinnedVecGrowthError> {
        let capacity = self.capacity();
        match new_capacity <= capacity {
            true => Ok(capacity),
            false => {
                let mut num_fragments = self.num_fragments();

                let mut current_capacity = capacity;

                while new_capacity > current_capacity {
                    let new_fragment_capacity = self
                        .growth
                        .new_fragment_capacity(self.fragments_for(num_fragments));
                    let new_fragment = Fragment::new(new_fragment_capacity);

                    self.push_fragment(new_fragment, num_fragments);

                    num_fragments += 1;
                    current_capacity += new_fragment_capacity;
                }

                self.num_fragments.store(num_fragments, Ordering::SeqCst);
                self.capacity.store(current_capacity, Ordering::SeqCst);

                Ok(current_capacity)
            }
        }
    }

    unsafe fn slices_mut<R: RangeBounds<usize>>(
        &self,
        range: R,
    ) -> <Self::P as PinnedVec<T>>::SliceMutIter<'_> {
        use std::slice::from_raw_parts_mut;

        let fragments = self.fragments();
        let fragment_and_inner_indices =
            |i| self.growth.get_fragment_and_inner_indices_unchecked(i);

        let a = range_start(&range);
        let b = range_end(&range, self.capacity());

        match b.saturating_sub(a) {
            0 => vec![],
            _ => {
                let (sf, si) = fragment_and_inner_indices(a);
                let (ef, ei) = fragment_and_inner_indices(b - 1);

                match sf == ef {
                    true => {
                        let p = self.fragment_element_ptr_mut(sf, si);
                        let slice = from_raw_parts_mut(p, ei - si + 1);
                        vec![slice]
                    }
                    false => {
                        let mut vec = Vec::with_capacity(ef - sf + 1);

                        let slice_len = fragments[sf].capacity() - si;
                        let ptr_s = self.fragment_element_ptr_mut(sf, si);
                        vec.push(from_raw_parts_mut(ptr_s, slice_len));

                        for (f, fragment) in fragments.iter().enumerate().take(ef).skip(sf + 1) {
                            let slice_len = fragment.capacity();
                            let ptr_s = self.fragment_element_ptr_mut(f, 0);
                            vec.push(from_raw_parts_mut(ptr_s, slice_len));
                        }

                        let slice_len = ei + 1;
                        let ptr_s = self.fragment_element_ptr_mut(ef, 0);
                        vec.push(from_raw_parts_mut(ptr_s, slice_len));

                        vec
                    }
                }
            }
        }
    }

    unsafe fn iter<'a>(&'a self, len: usize) -> impl Iterator<Item = &'a T> + 'a
    where
        T: 'a,
    {
        let fragments = self.fragments();

        match len {
            0 => IterCon::new(&fragments[0..1], 0),
            _ => {
                // let mut num_fragments = 0;
                let mut count = 0;

                for (num_fragments, fragment) in fragments.iter().enumerate() {
                    // for fragment in fragments.iter() {
                    // num_fragments += 1;
                    let capacity = fragment.capacity();
                    let new_count = count + capacity;

                    match new_count >= len {
                        true => {
                            let last_fragment_len = capacity - (new_count - len);
                            return IterCon::new(
                                &fragments[0..(num_fragments + 1)],
                                last_fragment_len,
                            );
                        }
                        false => count = new_count,
                    }
                }

                let last_fragment_len = fragments[fragments.len() - 1].capacity();
                IterCon::new(fragments, last_fragment_len)
            }
        }
    }

    unsafe fn iter_mut<'a>(&'a mut self, len: usize) -> impl Iterator<Item = &'a mut T> + 'a
    where
        T: 'a,
    {
        self.fragments.set_len(self.num_fragments());
        set_fragments_len(&mut self.fragments, len);
        let iter = crate::IterMut::new(&mut self.fragments);
        iter.take(len)
    }

    unsafe fn set_pinned_vec_len(&mut self, len: usize) {
        self.fragments.set_len(self.num_fragments());
        set_fragments_len(&mut self.fragments, len);
    }

    unsafe fn get(&self, index: usize) -> Option<&T> {
        match index < self.capacity() {
            true => {
                let (f, i) = self.growth.get_fragment_and_inner_indices_unchecked(index);
                Some(&*self.fragment_element_ptr(f, i))
            }
            false => None,
        }
    }

    unsafe fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index < self.capacity() {
            true => {
                let (f, i) = self.growth.get_fragment_and_inner_indices_unchecked(index);
                Some(&mut *self.fragment_element_ptr_mut(f, i))
            }
            false => None,
        }
    }

    unsafe fn get_ptr_mut(&self, index: usize) -> *mut T {
        assert!(index < self.capacity());
        let (f, i) = self.growth.get_fragment_and_inner_indices_unchecked(index);
        self.fragment_element_ptr(f, i) as *mut T
    }

    unsafe fn reserve_maximum_concurrent_capacity(
        &mut self,
        current_len: usize,
        new_maximum_capacity: usize,
    ) -> usize {
        self.fragments.set_len(self.num_fragments());
        set_fragments_len(&mut self.fragments, current_len);

        if self.maximum_capacity < new_maximum_capacity {
            let (num_required_fragments, target_capacity) =
                num_fragments_for_capacity(&self.fragments, &self.growth, new_maximum_capacity);

            assert!(target_capacity >= new_maximum_capacity);

            let num_additional_fragments =
                num_required_fragments.saturating_sub(self.fragments.len());
            self.fragments.reserve(num_additional_fragments);

            self.maximum_capacity = target_capacity;
            self.ptr_fragments = self.fragments.as_mut_ptr();

            let (fragment_pointers, ptr_fragments_pointers) =
                get_pointers(&self.fragments, self.fragments.capacity());
            self.fragment_pointers = fragment_pointers;
            self.ptr_fragments_pointers = ptr_fragments_pointers;

            self.fragments.set_len(self.fragments.capacity());
        }

        self.maximum_capacity
    }

    unsafe fn clear(&mut self, prior_len: usize) {
        self.fragments.set_len(self.num_fragments());
        self.set_pinned_vec_len(prior_len);

        if !self.fragments.is_empty() {
            self.fragments.truncate(1);
            self.fragments[0].clear();
        }

        let data = data(&mut self.fragments, &self.growth);

        self.capacity = data.capacity.into();
        self.maximum_capacity = data.maximum_capacity;
        self.num_fragments = data.num_fragments.into();
        self.ptr_fragments = data.ptr_fragments;
        self.fragment_pointers = data.fragment_pointers;
        self.ptr_fragments_pointers = data.ptr_fragments_pointers;
    }
}

fn data<G: Growth, T>(fragments: &mut Vec<Fragment<T>>, growth: &G) -> Data<T> {
    let num_fragments = fragments.len();

    let capacity = fragments.iter().map(|x| x.capacity()).sum::<usize>();

    let maximum_capacity = maximum_concurrent_capacity(fragments, fragments.capacity(), growth);

    let ptr_fragments = fragments.as_mut_ptr();

    let (fragment_pointers, ptr_fragments_pointers) = get_pointers(fragments, fragments.capacity());

    unsafe { fragments.set_len(fragments.capacity()) };

    Data {
        capacity,
        maximum_capacity,
        num_fragments,
        ptr_fragments,
        fragment_pointers,
        ptr_fragments_pointers,
    }
}

struct Data<T> {
    capacity: usize,
    maximum_capacity: usize,
    num_fragments: usize,
    ptr_fragments: *mut Fragment<T>,
    fragment_pointers: Vec<*const T>,
    ptr_fragments_pointers: *const *const T,
}
