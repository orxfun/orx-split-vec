use orx_concurrent_iter::{ConcurrentIter, IntoConcurrentIter};
use orx_split_vec::{PinnedVec, SplitVec};

fn into_con_iter_first<I>(x: I) -> Option<I::Item>
where
    I: IntoConcurrentIter,
{
    let con_iter = x.into_con_iter();
    con_iter.next()
}

#[test]
fn into_con_iter() {
    let mut vec = SplitVec::<usize>::new();
    vec.push(42);

    let first = into_con_iter_first(vec);
    assert_eq!(first, Some(42));
}

#[test]
fn ref_into_con_iter() {
    let mut vec = SplitVec::<usize>::new();
    vec.push(42);

    let first = into_con_iter_first(&vec);
    assert_eq!(first, Some(&42));
}

#[test]
fn con_iterable() {
    fn fun<I>(x: I) -> Option<I::Item>
    where
        I: orx_concurrent_iter::ConcurrentIterable,
    {
        let con_iter = orx_concurrent_iter::ConcurrentIterable::con_iter(&x);
        con_iter.next()
    }

    let mut vec = SplitVec::<usize>::new();
    vec.push(42);

    let first = fun(&vec);
    assert_eq!(first, Some(&42));
}

#[test]
fn con_collection() {
    fn fun<I>(x: &I) -> Option<&I::Item>
    where
        I: orx_concurrent_iter::ConcurrentCollection,
    {
        let con_iter = orx_concurrent_iter::ConcurrentCollection::con_iter(x);
        con_iter.next()
    }

    let mut vec = SplitVec::<usize>::new();
    vec.push(42);

    let first = fun(&vec);
    assert_eq!(first, Some(&42));
}
