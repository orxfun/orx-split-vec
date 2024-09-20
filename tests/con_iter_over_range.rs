use orx_split_vec::*;

#[test]
fn iter_over_range() {
    let vec = SplitVec::<_, Doubling>::from_iter([0, 1, 2, 3, 4, 5, 6].into_iter());
    let con_vec = vec.into_concurrent();

    unsafe {
        let vec: Vec<_> = con_vec.iter_over_range(..7).copied().collect();
        assert_eq!(vec, &[0, 1, 2, 3, 4, 5, 6]);

        let vec: Vec<_> = con_vec.iter_over_range(..4).copied().collect();
        assert_eq!(vec, &[0, 1, 2, 3]);

        let vec: Vec<_> = con_vec.iter_over_range(1..7).copied().collect();
        assert_eq!(vec, &[1, 2, 3, 4, 5, 6]);

        let vec: Vec<_> = con_vec.iter_over_range(1..4).copied().collect();
        assert_eq!(vec, &[1, 2, 3]);

        let vec: Vec<_> = con_vec.iter_over_range(4..4).copied().collect();
        assert_eq!(vec, &[]);

        let vec: Vec<_> = con_vec.iter_over_range(4..3).copied().collect();
        assert_eq!(vec, &[]);

        let vec: Vec<_> = con_vec.iter_over_range(1..=4).copied().collect();
        assert_eq!(vec, &[1, 2, 3, 4]);
    }
}
