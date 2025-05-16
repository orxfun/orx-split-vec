// use crate::*;
// use alloc::vec::Vec;
// use orx_concurrent_iter::*;
// use test_case::test_matrix;

// #[test_matrix([SplitVec::with_doubling_growth_and_fragments_capacity(16), SplitVec::with_linear_growth_and_fragments_capacity(10, 33)])]
// fn enumerate_item<G: Growth>(mut vec: SplitVec<usize, G>) {
//     for i in 2..6 {
//         vec.push(i);
//     }
//     let iter = vec.into_con_iter().enumerate();
//     let mut j = 0;
//     while let Some((i, x)) = iter.next() {
//         assert_eq!((i, x), (j, &(j + 2)));
//         j += 1;
//     }

//     vec.clear();
//     for i in 2..6 {
//         vec.push(i);
//     }
//     let iter = vec.into_con_iter().enumerate();
//     let mut j = 0;
//     while let Some((i2, (i, x))) = iter.next_with_idx() {
//         assert_eq!((i, x), (j, &(j + 2)));
//         assert_eq!(i, i2);
//         j += 1;
//     }
// }

// #[test_matrix([SplitVec::with_doubling_growth_and_fragments_capacity(16), SplitVec::with_linear_growth_and_fragments_capacity(10, 33)])]
// fn enumerate_item_puller<G: Growth>(mut vec: SplitVec<usize, G>) {
//     for i in 2..6 {
//         vec.push(i);
//     }
//     let iter = vec.into_con_iter().enumerate();
//     let puller = iter.item_puller();
//     let collected: Vec<_> = puller.collect();
//     assert_eq!(collected, [0, 1, 2, 3].map(|x| (x, &vec[x])));

//     vec.clear();
//     for i in 2..6 {
//         vec.push(i);
//     }
//     let iter = vec.into_con_iter().enumerate();
//     let puller = iter.item_puller_with_idx();
//     let collected: Vec<_> = puller.collect();
//     assert_eq!(collected, [0, 1, 2, 3].map(|x| (x, (x, &vec[x]))));
// }

// #[test_matrix([SplitVec::with_doubling_growth_and_fragments_capacity(16), SplitVec::with_linear_growth_and_fragments_capacity(10, 33)])]
// fn enumerate_chunk_puller<G: Growth>(mut vec: SplitVec<usize, G>) {
//     for i in 2..6 {
//         vec.push(i);
//     }
//     let iter = vec.into_con_iter().enumerate();

//     let mut j = 0;

//     let mut puller = iter.chunk_puller(2);
//     while let Some(chunk) = puller.pull() {
//         assert_eq!(chunk.len(), 2);
//         for (i, x) in chunk {
//             assert_eq!((i, x), (j, &(j + 2)));
//             j += 1;
//         }
//     }
// }

// #[test_matrix([SplitVec::with_doubling_growth_and_fragments_capacity(16), SplitVec::with_linear_growth_and_fragments_capacity(10, 33)])]
// fn copied<G: Growth>(mut vec: SplitVec<usize, G>) {
//     for i in 2..6 {
//         vec.push(i);
//     }
//     let iter = vec.into_con_iter().copied();
//     let values: Vec<_> = iter.item_puller().collect();
//     assert_eq!(values, vec);
// }

// #[test_matrix([SplitVec::with_doubling_growth_and_fragments_capacity(16), SplitVec::with_linear_growth_and_fragments_capacity(10, 33)])]
// fn cloned<G: Growth>(mut vec: SplitVec<usize, G>) {
//     for i in 2..6 {
//         vec.push(i);
//     }
//     let iter = vec.into_con_iter().cloned();
//     let values: Vec<_> = iter.item_puller().collect();
//     assert_eq!(values, vec);
// }
