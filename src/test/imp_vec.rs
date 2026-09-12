use crate::{Growth, PinnedVec, SplitVec};

#[test]
fn as_imp_vec() {
    crate::test_all_growth_types!(test_as_imp_vec);
}

fn test_as_imp_vec<G>(mut vec: SplitVec<usize, G>)
where
    G: Growth,
{
    vec.push(0);

    {
        let imp_vec = vec.as_imp_vec();
        imp_vec.imp_push(1);
        imp_vec.imp_extend_from_slice(&[2, 3]);
    }

    assert!(vec.iter_over(..).copied().eq([0, 1, 2, 3]));
}

#[test]
fn into_imp_vec() {
    crate::test_all_growth_types!(test_into_imp_vec);
}

fn test_into_imp_vec<G>(vec: SplitVec<usize, G>)
where
    G: Growth,
{
    let imp_vec = vec.into_imp_vec();
    imp_vec.imp_push(1);
    imp_vec.imp_extend_from_slice(&[2, 3]);

    let vec = imp_vec.into_inner();
    assert!(vec.iter_over(..).copied().eq([1, 2, 3]));
}
