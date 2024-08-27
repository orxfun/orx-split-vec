use orx_split_vec::*;

#[test]
fn con_pin_vec_grow() {
    const LEN: usize = 1486;

    fn test<G: GrowthWithConstantTimeAccess>(vec: SplitVec<String, G>) {
        assert_eq!(vec.fragments().len(), 1);

        let growth = vec.growth().clone();

        let mut num_fragments = 1;
        let mut capacity = growth.fragment_capacity_of(0);

        let con_pinned_vec = vec.into_concurrent();

        for i in 0..LEN {
            if i == capacity {
                let new_fragment_capacity = growth.fragment_capacity_of(num_fragments);
                let expected_new_capacity = capacity + new_fragment_capacity;

                let new_capacity = con_pinned_vec.grow_to(capacity + 1).unwrap();
                assert_eq!(new_capacity, expected_new_capacity);
                assert_eq!(con_pinned_vec.capacity(), expected_new_capacity);

                num_fragments += 1;
                capacity = new_capacity;
            }

            let ptr = unsafe { con_pinned_vec.get_ptr_mut(i) };
            unsafe { ptr.write(i.to_string()) };
        }

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

    test(SplitVec::with_doubling_growth_and_fragments_capacity(32));
    test(SplitVec::with_linear_growth_and_fragments_capacity(10, 32));
}

#[test]
fn con_pin_vec_grow_filled() {
    const LEN: usize = 1486;

    fn test<G: GrowthWithConstantTimeAccess>(vec: SplitVec<String, G>) {
        assert_eq!(vec.fragments().len(), 1);

        let growth = vec.growth().clone();

        let mut num_fragments = 1;
        let mut capacity = growth.fragment_capacity_of(0);

        let con_pinned_vec = vec.into_concurrent();

        for i in 0..LEN {
            if i == capacity {
                let new_fragment_capacity = growth.fragment_capacity_of(num_fragments);
                let expected_new_capacity = capacity + new_fragment_capacity;

                let new_capacity = con_pinned_vec
                    .grow_to_and_fill_with(capacity + 1, || "x".to_string())
                    .unwrap();
                assert_eq!(new_capacity, expected_new_capacity);
                assert_eq!(con_pinned_vec.capacity(), expected_new_capacity);

                for i in capacity..new_capacity {
                    let x = unsafe { con_pinned_vec.get(i) };
                    assert_eq!(x, Some(&"x".to_string()));
                }

                num_fragments += 1;
                capacity = new_capacity;
            }

            let ptr = unsafe { con_pinned_vec.get_ptr_mut(i) };
            unsafe { ptr.write(i.to_string()) };
        }

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

    test(SplitVec::with_doubling_growth_and_fragments_capacity(32));
    test(SplitVec::with_linear_growth_and_fragments_capacity(10, 32));
}
