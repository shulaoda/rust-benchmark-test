use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use phf::phf_set;

use rust_bench_test::generate_cases;

static ARRAY_BASIC_CASES: [&str; 13] = [
    "apply",
    "construct",
    "defineProperty",
    "deleteProperty",
    "get",
    "getOwnPropertyDescriptor",
    "getPrototypeOf",
    "has",
    "isExtensible",
    "ownKeys",
    "preventExtensions",
    "set",
    "setPrototypeOf",
];

static PHF_BASIC_CASES: phf::Set<&'static str> = phf_set! {
    "apply",
    "construct",
    "defineProperty",
    "deleteProperty",
    "get",
    "getOwnPropertyDescriptor",
    "getPrototypeOf",
    "has",
    "isExtensible",
    "ownKeys",
    "preventExtensions",
    "set",
    "setPrototypeOf",
};

fn phf(s: &str) -> bool {
    PHF_BASIC_CASES.contains(s)
}

fn array(s: &str) -> bool {
    ARRAY_BASIC_CASES.contains(&s)
}

fn benchmark(c: &mut Criterion) {
    let cases = generate_cases(&ARRAY_BASIC_CASES);

    println!("Benchmark Cases: \n{}\n", cases.join("\n"));

    let [most, worst, first, middle, last] = cases;

    let mut c = c.benchmark_group("basic");

    c.bench_with_input(BenchmarkId::new("phf", "most"), &most, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "most"), &most, |b, s| {
        b.iter(|| array(s))
    });

    c.bench_with_input(BenchmarkId::new("phf", "worst"), &worst, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "worst"), &worst, |b, s| {
        b.iter(|| array(s))
    });

    c.bench_with_input(BenchmarkId::new("phf", "first"), &first, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "first"), &first, |b, s| {
        b.iter(|| array(s))
    });

    c.bench_with_input(BenchmarkId::new("phf", "middle"), &middle, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "middle"), &middle, |b, s| {
        b.iter(|| array(s))
    });

    c.bench_with_input(BenchmarkId::new("phf", "last"), &last, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "last"), &last, |b, s| {
        b.iter(|| array(s))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
