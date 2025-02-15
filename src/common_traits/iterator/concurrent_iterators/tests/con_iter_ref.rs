use crate::*;
use alloc::string::{String, ToString};
use orx_concurrent_iter::*;
use test_case::test_matrix;

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [32, 1024],
    [2, 4, 8],
    [1, 4, 5, 64]
)]
fn con_iter_ref_primitive(
    mut vec: SplitVec<i64, impl Growth>,
    len: usize,
    num_threads: usize,
    batch: usize,
) {
    vec.clear();
    vec.extend(0i64..(len as i64));
    let expected_sum: i64 = vec.iter().sum();

    let con_iter = vec.con_iter();
    let iter = &con_iter;

    let sum: i64 = std::thread::scope(|s| {
        let mut handles = alloc::vec![];
        for _ in 0..num_threads {
            handles.push(s.spawn(move || {
                let mut sum = 0i64;
                match batch {
                    1 => {
                        while let Some(next) = iter.next() {
                            sum += next;
                        }
                    }
                    n => {
                        let mut chunks_iter = iter.buffered_iter_x(n);
                        while let Some(next) = chunks_iter.next() {
                            sum += next.values.sum::<i64>();
                        }
                    }
                }

                sum
            }));
        }

        handles.into_iter().map(|x| x.join().expect("-")).sum()
    });

    assert_eq!(sum, expected_sum);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [32, 1024],
    [2, 4, 8],
    [1, 4, 5, 64]
)]
fn con_iter_ref_heap(
    mut vec: SplitVec<String, impl Growth>,
    len: usize,
    num_threads: usize,
    batch: usize,
) {
    vec.clear();
    vec.extend((0..len).map(|x| x.to_string()));
    let expected_sum: usize = vec.iter().map(|x| x.len()).sum();

    let con_iter = vec.con_iter();
    let iter = &con_iter;

    let sum: usize = std::thread::scope(|s| {
        let mut handles = alloc::vec![];
        for _ in 0..num_threads {
            handles.push(s.spawn(move || {
                let mut sum = 0usize;
                match batch {
                    1 => {
                        while let Some(next) = iter.next() {
                            sum += next.len();
                        }
                    }
                    n => {
                        let mut chunks_iter = iter.buffered_iter_x(n);
                        while let Some(next) = chunks_iter.next() {
                            sum += next.values.map(|x| x.len()).sum::<usize>();
                        }
                    }
                }
                sum
            }));
        }

        handles.into_iter().map(|x| x.join().expect("-")).sum()
    });

    assert_eq!(sum, expected_sum);
}
