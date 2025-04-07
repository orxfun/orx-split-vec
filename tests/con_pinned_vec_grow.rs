use std::arch::x86_64::_CMP_TRUE_UQ;

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

    test(SplitVec::with_doubling_growth_and_max_concurrent_capacity());
    test(SplitVec::with_linear_growth_and_fragments_capacity(10, 32));
}

#[test]
fn con_pin_vec_grow_filled_xyz() {
    #[cfg(not(miri))]
    const TARGET_LEN: usize = 35478;

    #[cfg(miri)]
    const TARGET_LEN: usize = 37;

    // const TARGET_LEN: usize = 51111;

    fn test<G: GrowthWithConstantTimeAccess>(mut vec: SplitVec<String, G>) {
        assert_eq!(vec.fragments().len(), 1);

        let growth = vec.growth().clone();

        let mut num_fragments = 1;
        let initial_capacity = growth.fragment_capacity_of(0);
        let mut capacity = initial_capacity;

        // # SAFETY: the vector must have no gaps within its capacity before grow_to_and_fill_with call.
        for _ in 0..capacity {
            vec.push("before".to_string());
        }

        let con_pinned_vec = vec.into_concurrent();

        for i in 0..TARGET_LEN {
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
            let _ = unsafe { ptr.replace(i.to_string()) };
        }

        for i in 0..TARGET_LEN {
            let x = unsafe { con_pinned_vec.get(i) };
            assert_eq!(x, Some(&i.to_string()));
        }

        // # SAFETY: cannot leave elements in TARGET_LEN..capacity out, it would lead to memory leak
        let mut vec = unsafe { con_pinned_vec.into_inner(capacity) };

        assert_eq!(vec.len(), capacity);
        for i in 0..vec.len() {
            let x = vec.get(i);
            let is_within_len = i < TARGET_LEN;
            match is_within_len {
                true => assert_eq!(x, Some(&i.to_string())),
                false => assert_eq!(x, Some(&"x".to_string())),
            }
        }

        // # SAFETY: drop the allocated elements within TARGET_LEN..capacity
        vec.truncate(TARGET_LEN);
        assert_eq!(vec.len(), TARGET_LEN);
        for i in 0..vec.len() {
            let x = vec.get(i);
            assert_eq!(x, Some(&i.to_string()));
        }
    }

    test(SplitVec::with_doubling_growth_and_max_concurrent_capacity());
    // test(SplitVec::with_linear_growth_and_fragments_capacity(10, 32));
}

#[test]
fn abc() {
    const LEN: usize = 5;

    let mut vec = SplitVec::with_doubling_growth_and_max_concurrent_capacity();

    let growth = vec.growth().clone();

    let mut num_fragments = 1;
    let mut capacity = growth.fragment_capacity_of(0);
    for _ in 0..capacity {
        vec.push("y".to_string());
    }

    let con_pinned_vec = vec.into_concurrent();

    let new_fragment_capacity = growth.fragment_capacity_of(num_fragments);
    let expected_new_capacity = capacity + new_fragment_capacity;

    dbg!(capacity, new_fragment_capacity, expected_new_capacity);

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

    // let ptr = unsafe { con_pinned_vec.get_ptr_mut(i) };
    // unsafe { ptr.write(i.to_string()) };

    // for i in 0..LEN {
    //     if i == capacity {
    //         let new_fragment_capacity = growth.fragment_capacity_of(num_fragments);
    //         let expected_new_capacity = capacity + new_fragment_capacity;

    //         let new_capacity = con_pinned_vec
    //             .grow_to_and_fill_with(capacity + 1, || "x".to_string())
    //             .unwrap();
    //         assert_eq!(new_capacity, expected_new_capacity);
    //         assert_eq!(con_pinned_vec.capacity(), expected_new_capacity);

    //         for i in capacity..new_capacity {
    //             let x = unsafe { con_pinned_vec.get(i) };
    //             assert_eq!(x, Some(&"x".to_string()));
    //         }

    //         num_fragments += 1;
    //         capacity = new_capacity;
    //     }

    //     // let ptr = unsafe { con_pinned_vec.get_ptr_mut(i) };
    //     // unsafe { ptr.write(i.to_string()) };
    // }

    // for i in 0..LEN {
    //     let x = unsafe { con_pinned_vec.get(i) };
    //     assert_eq!(x, Some(&i.to_string()));
    // }

    let vec = unsafe { con_pinned_vec.into_inner(new_capacity) };

    assert_eq!(vec.len(), new_capacity);
    for i in 0..vec.len() {
        // assert_eq!(&vec[i], &i.to_string());
    }
}

#[test]
fn reserve() {
    fn test<G: GrowthWithConstantTimeAccess>(vec: SplitVec<String, G>) {
        let initial_capacity = vec.capacity();

        let mut con_vec = vec.into_concurrent();
        let max_cap = con_vec.max_capacity();

        unsafe { con_vec.get_ptr_mut(0).write("first".to_string()) };

        assert_eq!(con_vec.capacity(), initial_capacity);

        unsafe { con_vec.reserve_maximum_concurrent_capacity(0, max_cap + 1) };
        let new_capacity = con_vec.capacity();
        assert_eq!(new_capacity, initial_capacity);
        assert!(con_vec.max_capacity() >= max_cap + 1);

        let vec = unsafe { con_vec.into_inner(1) };

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.capacity(), initial_capacity);
        assert_eq!(&vec[0], &"first".to_string());
    }

    test(SplitVec::with_doubling_growth_and_fragments_capacity(16));
    test(SplitVec::with_linear_growth_and_fragments_capacity(10, 32));
}

#[test]
fn into_concurrent_fill_with() {
    fn test<G: GrowthWithConstantTimeAccess>(vec: SplitVec<String, G>) {
        let initial_capacity = vec.capacity();
        let vec2 = vec.clone();

        let con_vec = vec.into_concurrent_filled_with(|| "x".to_string());
        let vec = unsafe { con_vec.into_inner(initial_capacity) };
        assert_eq!(
            vec,
            (0..initial_capacity)
                .map(|_| "x".to_string())
                .collect::<Vec<_>>()
        );

        let mut vec = vec2;
        vec.push("y".to_string());
        let con_vec = vec.into_concurrent_filled_with(|| "x".to_string());
        let vec = unsafe { con_vec.into_inner(initial_capacity) };
        assert_eq!(&vec[0], &"y".to_string());
        assert_eq!(
            vec.iter().skip(1).cloned().collect::<Vec<_>>(),
            (1..initial_capacity)
                .map(|_| "x".to_string())
                .collect::<Vec<_>>()
        );
    }
    test(SplitVec::with_doubling_growth_and_max_concurrent_capacity());
    test(SplitVec::with_linear_growth_and_fragments_capacity(10, 32));
}

#[test]
fn reserve_fill_with() {
    fn test<G: GrowthWithConstantTimeAccess>(vec: SplitVec<String, G>) {
        let initial_capacity = vec.capacity();

        let mut con_vec = vec.into_concurrent_filled_with(|| "x".to_string());
        let max_cap = con_vec.max_capacity();

        assert_eq!(con_vec.capacity(), initial_capacity);

        unsafe {
            con_vec.reserve_maximum_concurrent_capacity_fill_with(
                initial_capacity,
                max_cap + 1,
                || "y".to_string(),
            )
        };
        let new_capacity = con_vec.capacity();
        assert_eq!(new_capacity, initial_capacity);
        assert!(con_vec.max_capacity() >= max_cap + 1);

        let vec = unsafe { con_vec.into_inner(initial_capacity) };

        assert_eq!(vec.len(), initial_capacity);
        assert_eq!(vec.capacity(), initial_capacity);
        assert_eq!(
            vec,
            (0..initial_capacity)
                .map(|_| "x".to_string())
                .collect::<Vec<_>>()
        );
    }

    test(SplitVec::with_doubling_growth_and_fragments_capacity(16));
    test(SplitVec::with_linear_growth_and_fragments_capacity(10, 32));
}
