use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use orx_split_vec::{Doubling, Linear, Recursive, SplitVec};
use std::hint::black_box;

fn get_vec(len: usize) -> Vec<u64> {
    let modulo = len % 3;
    if modulo == 0 {
        vec![len as u64; len]
    } else if modulo == 1 {
        vec![(len + 1) as u64; len]
    } else {
        vec![(len + 2) as u64; len]
    }
}

fn doubling_from_std_vec<T>(std_vec: Vec<T>) -> SplitVec<T, Doubling> {
    SplitVec::from(std_vec)
}

fn linear_from_std_vec<T>(std_vec: Vec<T>) -> SplitVec<T, Linear> {
    SplitVec::from(std_vec)
}

fn recursive_from_std_vec<T>(std_vec: Vec<T>) -> SplitVec<T, Recursive> {
    SplitVec::from(std_vec)
}

fn test_for_type<T>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    num_u64s: usize,
    treatments: &[usize],
    value: fn(usize) -> Vec<T>,
) {
    for n in treatments {
        let treatment = format!("n={},elem-type=[u64;{}]", n, num_u64s);

        group.bench_with_input(
            BenchmarkId::new("doubling_from_std_vec", &treatment),
            n,
            |b, _| b.iter(|| doubling_from_std_vec(value(black_box(*n)))),
        );

        group.bench_with_input(
            BenchmarkId::new("linear_from_std_vec", &treatment),
            n,
            |b, _| b.iter(|| linear_from_std_vec(value(black_box(*n)))),
        );
        group.bench_with_input(
            BenchmarkId::new("recursive_from_std_vec", &treatment),
            n,
            |b, _| b.iter(|| recursive_from_std_vec(value(black_box(*n)))),
        );
    }
}

fn bench_from(c: &mut Criterion) {
    let treatments = vec![1_024, 16_384, 262_144, 4_194_304];

    let mut group = c.benchmark_group("from");

    const N: usize = 16;

    test_for_type(&mut group, N, &treatments, get_vec);

    group.finish();
}

criterion_group!(benches, bench_from);
criterion_main!(benches);
