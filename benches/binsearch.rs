use std::fmt;

use criterion::black_box;
use criterion::measurement::Measurement;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion};

use binary_search::{custom_binary_search_1, std_binary_search};

criterion_group!(
    benches,
    bench_binsearch,
    bench_binsearch_duplicates,
    bench_binsearch_worstcases,
    bench_random_sorted,
);
criterion_main!(benches);

#[derive(Copy, Clone)]
enum Cache {
    L1,
    L2,
    L3,
}

impl Cache {
    fn size(&self) -> usize {
        match self {
            Cache::L1 => 1000,      // 8kb
            Cache::L2 => 10_000,    // 80kb
            Cache::L3 => 1_000_000, // 8Mb
        }
    }
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Cache::L1 => "L1",
            Cache::L2 => "L2",
            Cache::L3 => "L3",
        };
        f.write_str(s)
    }
}

static CACHE_LEVELS: &[Cache] = &[Cache::L1, Cache::L2, Cache::L3];

fn bench_binsearch(c: &mut Criterion) {
    let mut group = c.benchmark_group("Binary Search Increasing");
    binsearch(&mut group, |i| i * 2);
    group.finish();
}

fn bench_binsearch_duplicates(c: &mut Criterion) {
    let mut group = c.benchmark_group("Binary Search With Duplicates");
    binsearch(&mut group, std::convert::identity);
}

fn bench_binsearch_worstcases(c: &mut Criterion) {
    let mut group = c.benchmark_group("Binary Search Worst cases");
    for cache in CACHE_LEVELS {
        let size = cache.size();
        let mut v: Vec<usize> = vec![0; size];
        let i = 1;
        v[size - 1] = i;

        group.bench_with_input(BenchmarkId::new("std", cache), &i, |b, i| {
            b.iter(|| std_binary_search(&v, &i))
        });
        group.bench_with_input(BenchmarkId::new("custom_1", cache), &i, |b, i| {
            b.iter(|| custom_binary_search_1(&v, &i))
        });
    }
    group.finish();
}

fn binsearch<'a, M, F>(group: &mut BenchmarkGroup<'a, M>, mapper: F)
where
    M: Measurement,
    F: Fn(usize) -> usize,
{
    // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
    let r = black_box(|| 0_usize.wrapping_mul(1664525).wrapping_add(1013904223));
    let r = r();
    for cache in CACHE_LEVELS {
        let size = cache.size();
        let v: Vec<usize> = (0..size).map(&mapper).collect();
        group.bench_with_input(BenchmarkId::new("std", cache), &size, |b, size| {
            // Lookup the whole range to get 50% hits and 50% misses.
            let i = mapper(r % size);
            b.iter(|| std_binary_search(&v, &i))
        });
        group.bench_with_input(BenchmarkId::new("custom_1", cache), &size, |b, size| {
            let i = mapper(r % size);
            b.iter(|| custom_binary_search_1(&v, &i))
        });
    }
}

fn bench_random_sorted(c: &mut Criterion) {
    use rand::Rng;

    // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
    let r = black_box(|| 0_usize.wrapping_mul(1664525).wrapping_add(1013904223));
    let r = r();

    let mut rng = rand::thread_rng();
    let mut group = c.benchmark_group("Binary Search With Random Elements Sorted");
    for cache in CACHE_LEVELS {
        let size = cache.size();
        let i = r % size;
        let mut v: Vec<usize> = (0..size).map(|_| rng.gen_range(1_usize, 256)).collect();
        v.sort_unstable();

        group.bench_with_input(BenchmarkId::new("std", cache), &i, |b, i| {
            b.iter(|| std_binary_search(&v, &i))
        });
        group.bench_with_input(BenchmarkId::new("custom_1", cache), &i, |b, i| {
            b.iter(|| custom_binary_search_1(&v, &i))
        });
    }
    group.finish();
}
