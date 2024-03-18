use crate::{Growth, SplitVec};
use std::fmt::Debug;

impl<T, G> Debug for SplitVec<T, G>
where
    T: Debug,
    G: Growth,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SplitVec [")?;
        for frag in &self.fragments {
            writeln!(f, "    {:?}", frag)?;
        }
        writeln!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn debug() {
        let mut vec = SplitVec::with_doubling_growth();
        for i in 0..13 {
            vec.push(i);
        }

        let debug_str = format!("{:?}", vec);
        assert_eq!(
            "SplitVec [\n    [0, 1, 2, 3]\n    [4, 5, 6, 7, 8, 9, 10, 11]\n    [12]\n]\n",
            debug_str
        );
    }
}
