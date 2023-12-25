use crate::prelude::*;
use crate::test_all_growth_types;
use orx_pinned_vec::PinnedVec;

#[test]
fn iter() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        for i in 0..1000 {
            vec.push(i);
        }

        let mut iter = vec.iter();
        for i in 0..1000 {
            // assert_eq!(1000 - i, iter.len());
            assert_eq!(Some(&i), iter.next());
        }
        assert_eq!(None, iter.next());
    }
    test_all_growth_types!(test);
}
