use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use orx_split_vec::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::hint::black_box;

fn get_indices(n: usize) -> Vec<usize> {
    let mut rng = ChaCha8Rng::seed_from_u64(7541);
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(rng.random_range(0..n));
    }
    vec
}
fn get_value<const N: usize>(i: usize) -> [u64; N] {
    let modulo = i % 3;
    if modulo == 0 {
        [i as u64; N]
    } else if modulo == 1 {
        [(i + 1) as u64; N]
    } else {
        [(i + 2) as u64; N]
    }
}
fn add<const N: usize>(a: [u64; N], b: &[u64; N]) -> [u64; N] {
    let mut sum = [0u64; N];
    for i in 0..N {
        sum[i] = a[i] + b[i];
    }
    sum
}

fn std_vec_with_capacity<T, F: Fn(usize) -> T>(n: usize, value: F) -> Vec<T> {
    let mut vec = Vec::with_capacity(n);
    for i in 0..n {
        vec.push(value(i))
    }
    vec
}
fn split_vec_linear<T, F: Fn(usize) -> T>(
    n: usize,
    value: F,
    constant_fragment_capacity_power: usize,
) -> SplitVec<T, Linear> {
    let mut vec = SplitVec::with_linear_growth(constant_fragment_capacity_power);
    for i in 0..n {
        vec.push(value(i))
    }
    vec
}
fn split_vec_doubling<T, F: Fn(usize) -> T>(n: usize, value: F) -> SplitVec<T, Doubling> {
    let mut vec = SplitVec::with_doubling_growth();
    for i in 0..n {
        vec.push(value(i))
    }
    vec
}

fn calc_std_vec<T: Default, F: Fn(T, &T) -> T>(add: F, vec: &[T], indices: &[usize]) -> T {
    let mut sum = T::default();
    for i in indices {
        sum = add(sum, &vec[*i]);
    }
    sum
}
fn calc_split_vec<T: Default, F: Fn(T, &T) -> T, G: Growth>(
    add: F,
    vec: &SplitVec<T, G>,
    indices: &[usize],
) -> T {
    let mut sum = T::default();
    for i in indices {
        sum = add(sum, &vec[*i]);
    }
    sum
}

fn test_for_type<T: Default>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    num_u64s: usize,
    treatments: &[usize],
    value: fn(usize) -> T,
    add: fn(T, &T) -> T,
) {
    for n in treatments {
        let treatment = format!("n={},elem-type=[u64;{}]", n, num_u64s);
        let indices = get_indices(*n);

        group.bench_with_input(BenchmarkId::new("std_vec", &treatment), n, |b, _| {
            let vec = std_vec_with_capacity(black_box(*n), value);
            b.iter(|| calc_std_vec(add, &vec, &indices))
        });

        group.bench_with_input(
            BenchmarkId::new("split_vec_linear - 2^10", &treatment),
            n,
            |b, _| {
                let vec = split_vec_linear(black_box(*n), value, 10);
                b.iter(|| calc_split_vec(add, &vec, &indices))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("split_vec_doubling", &treatment),
            n,
            |b, _| {
                let vec = split_vec_doubling(black_box(*n), value);
                b.iter(|| calc_split_vec(add, &vec, &indices))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("split_vec_recursive", &treatment),
            n,
            |b, _| {
                let vec: SplitVec<_, Recursive> = split_vec_doubling(black_box(*n), value).into();
                b.iter(|| calc_split_vec(add, &vec, &indices))
            },
        );
    }
}

fn bench(c: &mut Criterion) {
    let treatments = vec![1_024, 16_384, 262_144, 4_194_304];

    let mut group = c.benchmark_group("random_access");

    const N: usize = 16;
    test_for_type::<[u64; N]>(&mut group, N, &treatments, get_value, add);

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
