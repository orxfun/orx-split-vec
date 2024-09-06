use crate::{Growth, SplitVec};
use core::fmt::Debug;
use orx_pinned_vec::PinnedVec;

impl<T, G> Debug for SplitVec<T, G>
where
    T: Debug,
    G: Growth,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "SplitVec {{ len: {}, capacity:{}, data: [",
            self.len(),
            self.capacity()
        )?;
        for frag in &self.fragments {
            writeln!(f, "    {:?}", frag)?;
        }
        writeln!(f, "] }}")
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use alloc::format;

    #[test]
    fn debug() {
        let mut vec = SplitVec::with_doubling_growth();
        for i in 0..13 {
            vec.push(i);
        }

        let debug_str = format!("{:?}", vec);
        assert_eq!(
            "SplitVec { len: 13, capacity:28, data: [\n    [0, 1, 2, 3]\n    [4, 5, 6, 7, 8, 9, 10, 11]\n    [12]\n] }\n",
            debug_str
        );
    }
}
