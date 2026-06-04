use crate::Fragment;
use core::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

impl<T> Index<usize> for Fragment<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<Range<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<Range<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeFrom<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeFrom<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeTo<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeTo<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeInclusive<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeInclusive<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeToInclusive<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeToInclusive<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeFull> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeFull> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.data[index]
    }
}
