use crate::{
    fragment::transformations::{fragment_from_raw, fragment_into_raw},
    range_helpers::{range_end, range_start},
    Doubling, Fragment, GrowthWithConstantTimeAccess, SplitVec,
};
use orx_pinned_vec::{ConcurrentPinnedVec, PinnedVec};
use std::{
    cell::UnsafeCell,
    ops::RangeBounds,
    sync::atomic::{AtomicUsize, Ordering},
};

struct FragmentData {
    f: usize,
    len: usize,
    capacity: usize,
}

/// Concurrent wrapper ([`orx_pinned_vec::ConcurrentPinnedVec`]) for the `SplitVec`.
pub struct ConcurrentSplitVec<T, G: GrowthWithConstantTimeAccess = Doubling> {
    growth: G,
    data: Vec<UnsafeCell<*mut T>>,
    num_fragments: AtomicUsize,
    capacity: AtomicUsize,
    maximum_capacity: usize,
    max_num_fragments: usize,
    pinned_vec_len: usize,
}

impl<T, G: GrowthWithConstantTimeAccess> Drop for ConcurrentSplitVec<T, G> {
    fn drop(&mut self) {
        let mut take_fragment = |_fragment: Fragment<T>| {};
        unsafe { self.into_fragments(self.pinned_vec_len, &mut take_fragment) };
        self.zero();
    }
}

impl<T, G: GrowthWithConstantTimeAccess> ConcurrentSplitVec<T, G> {
    unsafe fn get_raw_mut_unchecked_fi(&self, f: usize, i: usize) -> *mut T {
        let p = *self.data[f].get();
        p.add(i)
    }

    unsafe fn get_raw_mut_unchecked_idx(&self, idx: usize) -> *mut T {
        let (f, i) = self.growth.get_fragment_and_inner_indices_unchecked(idx);
        self.get_raw_mut_unchecked_fi(f, i)
    }

    fn capacity_of(&self, f: usize) -> usize {
        self.growth.fragment_capacity_of(f)
    }

    fn layout(len: usize) -> std::alloc::Layout {
        std::alloc::Layout::array::<T>(len).unwrap()
    }

    unsafe fn to_fragment(&self, data: FragmentData) -> Fragment<T> {
        let ptr = *self.data[data.f].get();
        fragment_from_raw(ptr, data.len, data.capacity)
    }

    unsafe fn into_fragments<F>(&mut self, len: usize, take_fragment: &mut F)
    where
        F: FnMut(Fragment<T>),
    {
        let mut process_in_cap = |x: FragmentData| {
            let _fragment_to_drop = self.to_fragment(x);
        };
        let mut process_in_len = |x: FragmentData| {
            let fragment = self.to_fragment(x);
            take_fragment(fragment);
        };

        self.process_fragments(len, &mut process_in_len, &mut process_in_cap);
    }

    unsafe fn process_fragments<P, Q>(
        &self,
        len: usize,
        process_in_len: &mut P,
        process_in_cap: &mut Q,
    ) where
        P: FnMut(FragmentData),
        Q: FnMut(FragmentData),
    {
        let capacity = self.capacity();
        assert!(capacity >= len);

        let mut remaining_len = len;
        let mut f = 0;
        let mut taken_out_capacity = 0;

        while remaining_len > 0 {
            let capacity = self.capacity_of(f);
            taken_out_capacity += capacity;

            let len = match remaining_len <= capacity {
                true => remaining_len,
                false => capacity,
            };

            let fragment = FragmentData { f, len, capacity };
            process_in_len(fragment);
            remaining_len -= len;
            f += 1;
        }

        while capacity > taken_out_capacity {
            let capacity = self.capacity_of(f);
            taken_out_capacity += capacity;
            let len = 0;
            let fragment = FragmentData { f, len, capacity };
            process_in_cap(fragment);
            f += 1;
        }
    }

    fn zero(&mut self) {
        self.num_fragments = 0.into();
        self.capacity = 0.into();
        self.maximum_capacity = 0;
        self.max_num_fragments = 0;
        self.pinned_vec_len = 0;
    }
}

impl<T, G: GrowthWithConstantTimeAccess> From<SplitVec<T, G>> for ConcurrentSplitVec<T, G> {
    fn from(value: SplitVec<T, G>) -> Self {
        let (fragments, growth, pinned_vec_len) = (value.fragments, value.growth, value.len);

        let num_fragments = fragments.len();
        let max_num_fragments = fragments.capacity();

        let mut data = Vec::with_capacity(max_num_fragments);
        let mut total_len = 0;
        let mut maximum_capacity = 0;

        for (f, fragment) in fragments.into_iter().enumerate() {
            let (p, len, cap) = fragment_into_raw(fragment);

            let expected_cap = growth.fragment_capacity_of(f);
            assert_eq!(cap, expected_cap);

            total_len += len;
            maximum_capacity += cap;

            data.push(UnsafeCell::new(p as *mut T));
        }
        assert_eq!(total_len, pinned_vec_len);
        let capacity = maximum_capacity;

        for f in num_fragments..data.capacity() {
            let expected_cap = growth.fragment_capacity_of(f);
            maximum_capacity += expected_cap;

            data.push(UnsafeCell::new(std::ptr::null_mut()));
        }

        Self {
            growth,
            data,
            num_fragments: num_fragments.into(),
            capacity: capacity.into(),
            maximum_capacity,
            max_num_fragments,
            pinned_vec_len,
        }
    }
}

impl<T, G: GrowthWithConstantTimeAccess> ConcurrentPinnedVec<T> for ConcurrentSplitVec<T, G> {
    type P = SplitVec<T, G>;

    unsafe fn into_inner(mut self, len: usize) -> Self::P {
        let mut fragments = Vec::with_capacity(self.max_num_fragments);
        let mut take_fragment = |fragment| fragments.push(fragment);
        self.into_fragments(len, &mut take_fragment);

        self.zero();
        SplitVec::from_raw_parts(len, fragments, self.growth.clone())
    }

    unsafe fn clone_with_len(&self, len: usize) -> Self
    where
        T: Clone,
    {
        let mut fragments = Vec::with_capacity(self.max_num_fragments);
        let mut clone_fragment = |x: FragmentData| {
            let mut fragment = Fragment::new(x.capacity);
            let dst: *mut T = fragment.data.as_mut_ptr();
            let src = *self.data[x.f].get();
            for i in 0..x.len {
                let value = src.add(i).as_ref().expect("must be some");
                dst.add(i).write(value.clone());
            }
            fragment.set_len(x.len);
            fragments.push(fragment);
        };

        self.process_fragments(len, &mut clone_fragment, &mut |_| {});

        let split_vec = SplitVec::from_raw_parts(len, fragments, self.growth.clone());
        split_vec.into()
    }

    fn slices<R: RangeBounds<usize>>(&self, range: R) -> <Self::P as PinnedVec<T>>::SliceIter<'_> {
        use std::slice::from_raw_parts;

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
                        let p = unsafe { self.get_raw_mut_unchecked_fi(sf, si) };
                        let slice = unsafe { from_raw_parts(p, ei - si + 1) };
                        vec![slice]
                    }
                    false => {
                        let mut vec = Vec::with_capacity(ef - sf + 1);

                        let slice_len = self.capacity_of(sf) - si;
                        let p = unsafe { self.get_raw_mut_unchecked_fi(sf, si) };
                        let slice = unsafe { from_raw_parts(p, slice_len) };
                        vec.push(slice);

                        for f in (sf + 1)..ef {
                            let slice_len = self.capacity_of(f);
                            let p = unsafe { self.get_raw_mut_unchecked_fi(f, 0) };
                            let slice = unsafe { from_raw_parts(p, slice_len) };
                            vec.push(slice);
                        }

                        let slice_len = ei + 1;
                        let p = unsafe { self.get_raw_mut_unchecked_fi(ef, 0) };
                        let slice = unsafe { from_raw_parts(p, slice_len) };
                        vec.push(slice);

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
        self.slices(0..len).into_iter().flat_map(|x| x.iter())
    }

    unsafe fn slices_mut<R: RangeBounds<usize>>(
        &self,
        range: R,
    ) -> <Self::P as PinnedVec<T>>::SliceMutIter<'_> {
        use std::slice::from_raw_parts_mut;

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
                        let p = unsafe { self.get_raw_mut_unchecked_fi(sf, si) };
                        let slice = unsafe { from_raw_parts_mut(p, ei - si + 1) };
                        vec![slice]
                    }
                    false => {
                        let mut vec = Vec::with_capacity(ef - sf + 1);

                        let slice_len = self.capacity_of(sf) - si;
                        let p = unsafe { self.get_raw_mut_unchecked_fi(sf, si) };
                        let slice = unsafe { from_raw_parts_mut(p, slice_len) };
                        vec.push(slice);

                        for f in (sf + 1)..ef {
                            let slice_len = self.capacity_of(f);
                            let p = unsafe { self.get_raw_mut_unchecked_fi(f, 0) };
                            let slice = unsafe { from_raw_parts_mut(p, slice_len) };
                            vec.push(slice);
                        }

                        let slice_len = ei + 1;
                        let p = unsafe { self.get_raw_mut_unchecked_fi(ef, 0) };
                        let slice = unsafe { from_raw_parts_mut(p, slice_len) };
                        vec.push(slice);

                        vec
                    }
                }
            }
        }
    }

    unsafe fn iter_mut<'a>(&'a mut self, len: usize) -> impl Iterator<Item = &'a mut T> + 'a
    where
        T: 'a,
    {
        self.slices_mut(0..len)
            .into_iter()
            .flat_map(|x| x.iter_mut())
    }

    unsafe fn get(&self, index: usize) -> Option<&T> {
        match index < self.capacity() {
            true => {
                let p = self.get_raw_mut_unchecked_idx(index);
                Some(&*p)
            }
            false => None,
        }
    }

    unsafe fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index < self.capacity() {
            true => {
                let p = self.get_raw_mut_unchecked_idx(index);
                Some(&mut *p)
            }
            false => None,
        }
    }

    unsafe fn get_ptr_mut(&self, index: usize) -> *mut T {
        self.get_raw_mut_unchecked_idx(index)
    }

    fn max_capacity(&self) -> usize {
        self.maximum_capacity
    }

    fn capacity(&self) -> usize {
        self.capacity.load(Ordering::Acquire)
    }

    fn grow_to(&self, new_capacity: usize) -> Result<usize, orx_pinned_vec::PinnedVecGrowthError> {
        let capacity = self.capacity.load(Ordering::Acquire);
        match new_capacity <= capacity {
            true => Ok(capacity),
            false => {
                let mut f = self.num_fragments.load(Ordering::Relaxed);

                let mut current_capacity = capacity;

                while new_capacity > current_capacity {
                    let new_fragment_capacity = self.capacity_of(f);
                    let layout = Self::layout(new_fragment_capacity);
                    let ptr = unsafe { std::alloc::alloc(layout) } as *mut T;
                    unsafe { *self.data[f].get() = ptr };

                    f += 1;
                    current_capacity += new_fragment_capacity;
                }

                self.num_fragments.store(f, Ordering::SeqCst);
                self.capacity.store(current_capacity, Ordering::Release);

                Ok(current_capacity)
            }
        }
    }

    fn grow_to_and_fill_with<F>(
        &self,
        new_capacity: usize,
        fill_with: F,
    ) -> Result<usize, orx_pinned_vec::PinnedVecGrowthError>
    where
        F: Fn() -> T,
    {
        let capacity = self.capacity.load(Ordering::Acquire);
        match new_capacity <= capacity {
            true => Ok(capacity),
            false => {
                let mut f = self.num_fragments.load(Ordering::Relaxed);

                let mut current_capacity = capacity;

                while new_capacity > current_capacity {
                    let new_fragment_capacity = self.capacity_of(f);
                    let layout = Self::layout(new_fragment_capacity);
                    let ptr = unsafe { std::alloc::alloc(layout) } as *mut T;

                    for i in 0..new_fragment_capacity {
                        unsafe { ptr.add(i).write(fill_with()) };
                    }

                    unsafe { *self.data[f].get() = ptr };

                    f += 1;
                    current_capacity += new_fragment_capacity;
                }

                self.num_fragments.store(f, Ordering::SeqCst);
                self.capacity.store(current_capacity, Ordering::Release);

                Ok(current_capacity)
            }
        }
    }

    fn fill_with<F>(&self, range: std::ops::Range<usize>, fill_with: F)
    where
        F: Fn() -> T,
    {
        for i in range {
            unsafe { self.get_ptr_mut(i).write(fill_with()) };
        }
    }

    unsafe fn reserve_maximum_concurrent_capacity(
        &mut self,
        _current_len: usize,
        new_maximum_capacity: usize,
    ) -> usize {
        assert_eq!(self.max_num_fragments, self.data.len());
        assert_eq!(self.max_num_fragments, self.data.capacity());

        let mut num_required_fragments = 0;
        let mut max_cap = self.maximum_capacity;
        let f = self.data.len();

        while max_cap < new_maximum_capacity {
            max_cap += self.capacity_of(f);
            num_required_fragments += 1;
        }

        if num_required_fragments > 0 {
            self.data.reserve_exact(num_required_fragments);
        }

        for _ in self.max_num_fragments..self.data.capacity() {
            self.data.push(UnsafeCell::new(std::ptr::null_mut()));
        }

        self.maximum_capacity = (0..self.data.len()).map(|f| self.capacity_of(f)).sum();
        self.max_num_fragments = self.data.len();

        while self.maximum_capacity < new_maximum_capacity {
            let f = self.data.len();
            self.data.push(UnsafeCell::new(std::ptr::null_mut()));

            let capacity = self.capacity_of(f);
            self.maximum_capacity += capacity;
            self.max_num_fragments += 1;
        }

        assert_eq!(self.max_num_fragments, self.data.len());
        assert_eq!(self.max_num_fragments, self.data.capacity());

        self.maximum_capacity
    }

    unsafe fn set_pinned_vec_len(&mut self, len: usize) {
        self.pinned_vec_len = len;
    }

    unsafe fn clear(&mut self, len: usize) {
        let mut take_fragment = |_fragment: Fragment<T>| {};
        unsafe { self.into_fragments(len, &mut take_fragment) };
        self.zero();

        let max_num_fragments = self.data.len();
        self.data.clear();

        for _ in 0..max_num_fragments {
            self.data.push(UnsafeCell::new(std::ptr::null_mut()));
        }

        self.maximum_capacity = (0..self.data.len()).map(|f| self.capacity_of(f)).sum();
        self.pinned_vec_len = 0;
    }
}
