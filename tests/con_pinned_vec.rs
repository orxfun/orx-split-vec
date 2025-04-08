use orx_split_vec::*;

#[test]
fn drop_con_pin_vec_after_into_inner() {
    #[cfg(not(miri))]
    const LEN: usize = 1486;
    #[cfg(miri)]
    const LEN: usize = 65;

    fn test<G: GrowthWithConstantTimeAccess>(mut vec: SplitVec<String, G>) {
        for i in 0..LEN {
            vec.push(i.to_string());
        }

        let con_pinned_vec = vec.into_concurrent();

        for i in 0..LEN {
            let x = unsafe { con_pinned_vec.get(i) };
            assert_eq!(x, Some(&i.to_string()));
        }

        let vec = unsafe { con_pinned_vec.into_inner(LEN) };

        assert_eq!(vec.len(), LEN);
        for i in 0..vec.len() {
            assert_eq!(&vec[i], &i.to_string());
        }
    }

    test(SplitVec::new());
    test(SplitVec::with_doubling_growth_and_max_concurrent_capacity());
    test(SplitVec::with_linear_growth_and_fragments_capacity(10, 32));
}

#[test]
fn drop_con_pin_vec_as_con_pin_vec() {
    #[cfg(not(miri))]
    const LEN: usize = 1486;
    #[cfg(miri)]
    const LEN: usize = 65;

    fn test<G: GrowthWithConstantTimeAccess>(mut vec: SplitVec<String, G>) {
        for i in 0..LEN {
            vec.push(i.to_string());
        }

        let con_pinned_vec = vec.into_concurrent();

        for i in 0..LEN {
            let x = unsafe { con_pinned_vec.get(i) };
            assert_eq!(x, Some(&i.to_string()));
        }
    }

    test(SplitVec::new());
    test(SplitVec::with_doubling_growth_and_max_concurrent_capacity());
    test(SplitVec::with_linear_growth_and_fragments_capacity(10, 32));
}
