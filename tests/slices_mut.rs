use orx_split_vec::*;

fn slices_mut<G: Growth>(mut vec: SplitVec<String, G>, len: usize) {
    vec.clear();
    vec.extend((0..len).map(|x| x.to_string()));

    for i in 0..len {
        let begin = i + 1;
        let end = len.saturating_sub(1);
        let slices = vec.slices_mut(begin..end);
        for slice in slices {
            for (i, x) in slice.iter_mut().enumerate() {
                let _ = *x;
                *x = i.to_string();
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
fn test_slices_mut() {
    for len in LENGTHS {
        slices_mut(SplitVec::with_doubling_growth(), len);
        slices_mut(SplitVec::with_recursive_growth(), len);
        slices_mut(SplitVec::with_linear_growth(4), len);
    }
}
