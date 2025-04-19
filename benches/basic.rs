use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use phf::phf_set;

use rust_bench_test::generate_cases;

static ARRAY_BASIC_CASES: [&str; 19] = [
    "assert",
    "clear",
    "count",
    "countReset",
    "debug",
    "dir",
    "dirxml",
    "error",
    "group",
    "groupCollapsed",
    "groupEnd",
    "info",
    "log",
    "table",
    "time",
    "timeEnd",
    "timeLog",
    "trace",
    "warn",
];

static PHF_BASIC_CASES: phf::Set<&'static str> = phf_set! {
    "assert",
    "clear",
    "count",
    "countReset",
    "debug",
    "dir",
    "dirxml",
    "error",
    "group",
    "groupCollapsed",
    "groupEnd",
    "info",
    "log",
    "table",
    "time",
    "timeEnd",
    "timeLog",
    "trace",
    "warn",
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

    let [bad_input, first_input, middle_input, last_input] = cases;

    let mut c = c.benchmark_group("basic");

    c.bench_with_input(BenchmarkId::new("phf", "bad"), &bad_input, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "bad"), &bad_input, |b, s| {
        b.iter(|| array(s))
    });

    c.bench_with_input(BenchmarkId::new("phf", "first"), &first_input, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "first"), &first_input, |b, s| {
        b.iter(|| array(s))
    });

    c.bench_with_input(BenchmarkId::new("phf", "middle"), &middle_input, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(
        BenchmarkId::new("array", "middle"),
        &middle_input,
        |b, s| b.iter(|| array(s)),
    );

    c.bench_with_input(BenchmarkId::new("phf", "last"), &last_input, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "last"), &last_input, |b, s| {
        b.iter(|| array(s))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
