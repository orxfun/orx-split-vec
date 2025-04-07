use orx_split_vec::*;

fn slices<G: Growth>(mut vec: SplitVec<String, G>, len: usize) {
    vec.clear();
    vec.extend((0..len).map(|i| i.to_string()));

    for i in 0..len {
        let begin = i + 1;
        let end = len.saturating_sub(1);
        let slices = vec.slices(begin..end);
        let mut val = begin;
        for slice in slices {
            for x in slice.iter() {
                assert_eq!(x, &val.to_string());
                val += 1;
            }
        }

        if len > 0 {
            assert_eq!(vec.get(0), Some(&(0.to_string())));
        }
    }
}

#[cfg(not(miri))]
const LENGTHS: [usize; 8] = [0, 1, 4, 5, 15, 16, 17, 1033];
#[cfg(miri)]
const LENGTHS: [usize; 8] = [0, 1, 4, 5, 15, 16, 17, 37];

#[test]
fn test_slices() {
    for len in LENGTHS {
        slices(SplitVec::with_doubling_growth(), len);
        slices(SplitVec::with_recursive_growth(), len);
        slices(SplitVec::with_linear_growth(4), len);
    }
}
