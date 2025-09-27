use crate::{
    Doubling, Fragment, GrowthWithConstantTimeAccess, SplitVec,
    common_traits::iterator::{IterOfSlicesOfCon, SliceBorrowAsMut, SliceBorrowAsRef},
    concurrent_pinned_vec::{into_iter::ConcurrentSplitVecIntoIter, iter_ptr::IterPtrOfCon},
    fragment::transformations::{fragment_from_raw, fragment_into_raw},
};
use alloc::vec::Vec;
use core::ops::RangeBounds;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::{cell::UnsafeCell, ops::Range};
use orx_pinned_vec::ConcurrentPinnedVec;

pub struct FragmentData {
    pub f: usize,
    pub len: usize,
    pub capacity: usize,
}

/// Concurrent wrapper ([`orx_pinned_vec::ConcurrentPinnedVec`]) for the `SplitVec`.
pub struct ConcurrentSplitVec<T, G: GrowthWithConstantTimeAccess = Doubling> {
    growth: G,
    data: Vec<UnsafeCell<*mut T>>,
    capacity: AtomicUsize,
    maximum_capacity: usize,
    max_num_fragments: usize,
    pinned_vec_len: usize,
}

impl<T, G: GrowthWithConstantTimeAccess> Drop for ConcurrentSplitVec<T, G> {
    fn drop(&mut self) {
        fn take_fragment<T>(_fragment: Fragment<T>) {}
        unsafe { self.process_into_fragments(self.pinned_vec_len, &mut take_fragment) };
        self.zero();
    }
}

impl<T, G: GrowthWithConstantTimeAccess> ConcurrentSplitVec<T, G> {
    pub(super) fn destruct(mut self) -> (G, Vec<UnsafeCell<*mut T>>, usize) {
        let mut data = Vec::new();
        core::mem::swap(&mut self.data, &mut data);
        (
            self.growth.clone(),
            data,
            self.capacity.load(Ordering::Relaxed),
        )
    }

    unsafe fn get_raw_mut_unchecked_fi(&self, f: usize, i: usize) -> *mut T {
        let p = unsafe { *self.data[f].get() };
        unsafe { p.add(i) }
    }

    unsafe fn get_raw_mut_unchecked_idx(&self, idx: usize) -> *mut T {
        let (f, i) = self.growth.get_fragment_and_inner_indices_unchecked(idx);
        unsafe { self.get_raw_mut_unchecked_fi(f, i) }
    }

    fn capacity_of(&self, f: usize) -> usize {
        self.growth.fragment_capacity_of(f)
    }

    fn layout(len: usize) -> alloc::alloc::Layout {
        alloc::alloc::Layout::array::<T>(len).expect("len must not overflow")
    }

    unsafe fn to_fragment(&self, data: FragmentData) -> Fragment<T> {
        let ptr = unsafe { *self.data[data.f].get() };
        unsafe { fragment_from_raw(ptr, data.len, data.capacity) }
    }

    unsafe fn process_into_fragments<F>(&mut self, len: usize, take_fragment: &mut F)
    where
        F: FnMut(Fragment<T>),
    {
        let mut process_in_cap = |x: FragmentData| {
            let _fragment_to_drop = unsafe { self.to_fragment(x) };
        };
        let mut process_in_len = |x: FragmentData| {
            let fragment = unsafe { self.to_fragment(x) };
            take_fragment(fragment);
        };

        unsafe { self.process_fragments(len, &mut process_in_len, &mut process_in_cap) };
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
        self.capacity = 0.into();
        self.maximum_capacity = 0;
        self.max_num_fragments = 0;
        self.pinned_vec_len = 0;
    }

    fn num_fragments_for_capacity(&self, capacity: usize) -> usize {
        match capacity {
            0 => 0,
            _ => {
                self.growth
                    .get_fragment_and_inner_indices_unchecked(capacity - 1)
                    .0
                    + 1
            }
        }
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

            data.push(UnsafeCell::new(p));
        }
        assert_eq!(total_len, pinned_vec_len);
        let capacity = maximum_capacity;

        for f in num_fragments..data.capacity() {
            let expected_cap = growth.fragment_capacity_of(f);
            maximum_capacity += expected_cap;

            data.push(UnsafeCell::new(core::ptr::null_mut()));
        }

        Self {
            growth,
            data,
            capacity: capacity.into(),
            maximum_capacity,
            max_num_fragments,
            pinned_vec_len,
        }
    }
}

impl<T, G: GrowthWithConstantTimeAccess> ConcurrentPinnedVec<T> for ConcurrentSplitVec<T, G> {
    type P = SplitVec<T, G>;

    type SliceIter<'a>
        = IterOfSlicesOfCon<'a, T, G, SliceBorrowAsRef>
    where
        Self: 'a;

    type SliceMutIter<'a>
        = IterOfSlicesOfCon<'a, T, G, SliceBorrowAsMut>
    where
        Self: 'a;

    type PtrIter<'a>
        = IterPtrOfCon<'a, T, G>
    where
        Self: 'a;

    type IntoIter = ConcurrentSplitVecIntoIter<T, G>;

    unsafe fn into_inner(mut self, len: usize) -> Self::P {
        let mut fragments = Vec::with_capacity(self.max_num_fragments);
        let mut take_fragment = |fragment| fragments.push(fragment);
        unsafe { self.process_into_fragments(len, &mut take_fragment) };

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
            let src = unsafe { *self.data[x.f].get() };
            for i in 0..x.len {
                let value = unsafe { src.add(i).as_ref() }.expect("must be some");
                unsafe { dst.add(i).write(value.clone()) };
            }
            unsafe { fragment.set_len(x.len) };
            fragments.push(fragment);
        };

        unsafe { self.process_fragments(len, &mut clone_fragment, &mut |_| {}) };

        let split_vec = SplitVec::from_raw_parts(len, fragments, self.growth.clone());
        split_vec.into()
    }

    fn slices<R: RangeBounds<usize>>(&self, range: R) -> Self::SliceIter<'_> {
        Self::SliceIter::new(self.capacity(), &self.data, self.growth.clone(), range)
    }

    unsafe fn iter<'a>(&'a self, len: usize) -> impl Iterator<Item = &'a T> + 'a
    where
        T: 'a,
    {
        self.slices(0..len).flat_map(|x| x.iter())
    }

    unsafe fn iter_over_range<'a, R: RangeBounds<usize>>(
        &'a self,
        range: R,
    ) -> impl Iterator<Item = &'a T> + 'a
    where
        T: 'a,
    {
        let [a, b] = orx_pinned_vec::utils::slice::vec_range_limits(&range, None);
        self.slices(a..b).flat_map(|x| x.iter())
    }

    unsafe fn slices_mut<R: RangeBounds<usize>>(&self, range: R) -> Self::SliceMutIter<'_> {
        Self::SliceMutIter::new(self.capacity(), &self.data, self.growth.clone(), range)
    }

    unsafe fn iter_mut<'a>(&'a mut self, len: usize) -> impl Iterator<Item = &'a mut T> + 'a
    where
        T: 'a,
    {
        unsafe { self.slices_mut(0..len) }.flat_map(|x| x.iter_mut())
    }

    unsafe fn get(&self, index: usize) -> Option<&T> {
        match index < self.capacity() {
            true => {
                let p = unsafe { self.get_raw_mut_unchecked_idx(index) };
                Some(unsafe { &*p })
            }
            false => None,
        }
    }

    unsafe fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index < self.capacity() {
            true => {
                let p = unsafe { self.get_raw_mut_unchecked_idx(index) };
                Some(unsafe { &mut *p })
            }
            false => None,
        }
    }

    unsafe fn get_ptr_mut(&self, index: usize) -> *mut T {
        unsafe { self.get_raw_mut_unchecked_idx(index) }
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
                let mut f = self.num_fragments_for_capacity(capacity);
                let mut current_capacity = capacity;

                while new_capacity > current_capacity {
                    let new_fragment_capacity = self.capacity_of(f);
                    let layout = Self::layout(new_fragment_capacity);
                    let ptr = unsafe { alloc::alloc::alloc(layout) } as *mut T;
                    unsafe { *self.data[f].get() = ptr };

                    f += 1;
                    current_capacity += new_fragment_capacity;
                }

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
                let mut f = self.num_fragments_for_capacity(capacity);

                let mut current_capacity = capacity;

                while new_capacity > current_capacity {
                    let new_fragment_capacity = self.capacity_of(f);
                    let layout = Self::layout(new_fragment_capacity);
                    let ptr = unsafe { alloc::alloc::alloc(layout) } as *mut T;

                    for i in 0..new_fragment_capacity {
                        unsafe { ptr.add(i).write(fill_with()) };
                    }

                    unsafe { *self.data[f].get() = ptr };

                    f += 1;
                    current_capacity += new_fragment_capacity;
                }

                self.capacity.store(current_capacity, Ordering::Release);

                Ok(current_capacity)
            }
        }
    }

    fn fill_with<F>(&self, range: core::ops::Range<usize>, fill_with: F)
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
        let mut f = self.data.len();

        while max_cap < new_maximum_capacity {
            max_cap += self.capacity_of(f);
            num_required_fragments += 1;
            f += 1;
        }

        if num_required_fragments > 0 {
            self.data.reserve_exact(num_required_fragments);
        }

        for _ in self.max_num_fragments..self.data.capacity() {
            self.data.push(UnsafeCell::new(core::ptr::null_mut()));
        }

        self.maximum_capacity = (0..self.data.len()).map(|f| self.capacity_of(f)).sum();
        self.max_num_fragments = self.data.len();

        assert_eq!(self.max_num_fragments, self.data.len());
        assert_eq!(self.max_num_fragments, self.data.capacity());

        self.maximum_capacity
    }

    unsafe fn reserve_maximum_concurrent_capacity_fill_with<F>(
        &mut self,
        current_len: usize,
        new_maximum_capacity: usize,
        _fill_with: F,
    ) -> usize
    where
        F: Fn() -> T,
    {
        unsafe { self.reserve_maximum_concurrent_capacity(current_len, new_maximum_capacity) }
    }

    unsafe fn set_pinned_vec_len(&mut self, len: usize) {
        self.pinned_vec_len = len;
    }

    unsafe fn clear(&mut self, len: usize) {
        let mut take_fragment = |_fragment: Fragment<T>| {};
        unsafe { self.process_into_fragments(len, &mut take_fragment) };
        self.zero();

        let max_num_fragments = self.data.len();
        self.data.clear();

        for _ in 0..max_num_fragments {
            self.data.push(UnsafeCell::new(core::ptr::null_mut()));
        }

        self.maximum_capacity = (0..self.data.len()).map(|f| self.capacity_of(f)).sum();
        self.pinned_vec_len = 0;
    }

    unsafe fn ptr_iter_unchecked(&self, range: Range<usize>) -> Self::PtrIter<'_> {
        IterPtrOfCon::new(self.capacity(), &self.data, self.growth.clone(), range)
    }

    unsafe fn into_iter(self, range: Range<usize>) -> Self::IntoIter {
        let (growth, data, capacity) = self.destruct();
        ConcurrentSplitVecIntoIter::new(capacity, data, growth, range)
    }
}
