use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
    Criterion,
};
use orx_split_vec::prelude::*;

fn get_value_u64(i: usize) -> u64 {
    let modulo = i % 3;
    let value = if modulo == 0 {
        i
    } else if modulo == 1 {
        i + 1
    } else {
        i - 1
    };
    value as u64
}
fn get_value_u64x4(i: usize) -> (u64, u64, u64, u64) {
    let modulo = i % 3;
    let (x, y, z, w) = if modulo == 0 {
        (i, i + 1, i + 2, i + 3)
    } else if modulo == 1 {
        (i + 1, i + 2, i + 3, i + 4)
    } else {
        (i - 1, i, i + 1, i + 2)
    };
    (x as u64, y as u64, z as u64, w as u64)
}
fn get_value_u64x8(i: usize) -> (u64, u64, u64, u64, u64, u64, u64, u64) {
    let (x1, y1, z1, w1) = get_value_u64x4(i);
    let (x2, y2, z2, w2) = get_value_u64x4(i + 1);
    (x1, x2, y1, y2, z1, z2, w1, w2)
}
fn get_value_string(i: usize) -> String {
    get_value_u64(i).to_string()
}

fn std_vec_new<T, F: Fn(usize) -> T>(n: usize, value: F) {
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(value(i))
    }
}
fn std_vec_with_capacity<T, F: Fn(usize) -> T>(n: usize, value: F) {
    let mut vec = Vec::with_capacity(n);
    for i in 0..n {
        vec.push(value(i))
    }
}
fn split_vec_linear<T, F: Fn(usize) -> T>(n: usize, value: F, constant_fragment_capacity: usize) {
    let mut vec = SplitVec::with_linear_growth(constant_fragment_capacity);
    for i in 0..n {
        vec.push(value(i))
    }
}
fn split_vec_doubling<T, F: Fn(usize) -> T>(n: usize, value: F) {
    let mut vec = SplitVec::with_doubling_growth();
    for i in 0..n {
        vec.push(value(i))
    }
}

fn test_for_type<T>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    element_type: &str,
    treatments: &[usize],
    value: fn(usize) -> T,
) {
    for n in treatments {
        let treatment = format!("n={},elem-type={}", n, element_type);

        group.bench_with_input(
            BenchmarkId::new("std_vec_with_capacity", &treatment),
            n,
            |b, _| b.iter(|| std_vec_with_capacity(black_box(*n), value)),
        );

        group.bench_with_input(BenchmarkId::new("std_vec_new", &treatment), n, |b, _| {
            b.iter(|| std_vec_new(black_box(*n), value))
        });

        // linear
        group.bench_with_input(
            BenchmarkId::new("split_vec_linear - 5%", &treatment),
            n,
            |b, _| b.iter(|| split_vec_linear(black_box(*n), value, n / 20)),
        );
        group.bench_with_input(
            BenchmarkId::new("split_vec_linear - 10%", &treatment),
            n,
            |b, _| b.iter(|| split_vec_linear(black_box(*n), value, n / 10)),
        );
        group.bench_with_input(
            BenchmarkId::new("split_vec_linear - 20%", &treatment),
            n,
            |b, _| b.iter(|| split_vec_linear(black_box(*n), value, n / 5)),
        );

        // doubling
        group.bench_with_input(
            BenchmarkId::new("split_vec_doubling", &treatment),
            n,
            |b, _| b.iter(|| split_vec_doubling(black_box(*n), value)),
        );
    }
}

fn bench_grow(c: &mut Criterion) {
    let treatments = vec![100_000, 1_000_000, 10_000_000];

    let mut group = c.benchmark_group("grow");

    test_for_type(&mut group, "u64", &treatments, get_value_u64);
    test_for_type(
        &mut group,
        "(u64,u64,u64,u64)",
        &treatments,
        get_value_u64x4,
    );
    test_for_type(
        &mut group,
        "(u64,u64,u64,u64,u64,u64,u64,u64)",
        &treatments,
        get_value_u64x8,
    );
    test_for_type(&mut group, "String", &treatments, get_value_string);

    group.finish();
}

criterion_group!(benches, bench_grow);
criterion_main!(benches);
