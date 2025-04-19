use std::fs;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use phf::phf_set;

static DISALLOW_NEW_FOR_BUILTINS_1: [&str; 25] = [
    "2112e",
    "2dwqqd",
    "5169",
    "AABBWW",
    "BBCCDD",
    "BigInt",
    "Boolean",
    "HHAWD",
    "Number",
    "Q51515",
    "QQ",
    "QQDD",
    "String",
    "Symbol",
    "__+!@!",
    "Tencent",
    "adwad",
    "alibaba",
    "awf",
    "bytedance",
    "ccaw252",
    "dawgfaw",
    "dwqf",
    "mayi",
    "meituan",
];

static DISALLOW_NEW_FOR_BUILTINS_2: phf::Set<&'static str> = phf_set! {
    "2112e",
    "2dwqqd",
    "5169",
    "AABBWW",
    "BBCCDD",
    "BigInt",
    "Boolean",
    "HHAWD",
    "Number",
    "Q51515",
    "QQ",
    "QQDD",
    "String",
    "Symbol",
    "__+!@!",
    "Tencent",
    "adwad",
    "alibaba",
    "awf",
    "bytedance",
    "ccaw252",
    "dawgfaw",
    "dwqf",
    "mayi",
    "meituan",
};

fn phf(s: &str) -> bool {
    DISALLOW_NEW_FOR_BUILTINS_2.contains(s)
}

fn array(s: &str) -> bool {
    DISALLOW_NEW_FOR_BUILTINS_1.contains(&s)
}

fn array_binary(s: &str) -> bool {
    DISALLOW_NEW_FOR_BUILTINS_1.binary_search(&s).is_ok()
}

fn benchmark(c: &mut Criterion) {
    let input_data = fs::read_to_string("./benches/inputs_basic.txt").unwrap();
    let [bad_input, first_input, middle_input, last_input] =
        input_data.trim().split('\n').collect::<Vec<&str>>()[..]
    else {
        panic!("Invalid input data")
    };

    let mut c = c.benchmark_group("basic");

    c.bench_with_input(BenchmarkId::new("phf", "bad"), &bad_input, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "bad"), &bad_input, |b, s| {
        b.iter(|| array(s))
    });
    c.bench_with_input(
        BenchmarkId::new("array_binary", "bad"),
        &bad_input,
        |b, s| b.iter(|| array_binary(s)),
    );

    c.bench_with_input(BenchmarkId::new("phf", "first"), &first_input, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "first"), &first_input, |b, s| {
        b.iter(|| array(s))
    });
    c.bench_with_input(
        BenchmarkId::new("array_binary", "first"),
        &first_input,
        |b, s| b.iter(|| array_binary(s)),
    );

    c.bench_with_input(BenchmarkId::new("phf", "middle"), &middle_input, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(
        BenchmarkId::new("array", "middle"),
        &middle_input,
        |b, s| b.iter(|| array(s)),
    );
    c.bench_with_input(
        BenchmarkId::new("array_binary", "middle"),
        &middle_input,
        |b, s| b.iter(|| array_binary(s)),
    );

    c.bench_with_input(BenchmarkId::new("phf", "last"), &last_input, |b, s| {
        b.iter(|| phf(s))
    });
    c.bench_with_input(BenchmarkId::new("array", "last"), &last_input, |b, s| {
        b.iter(|| array(s))
    });
    c.bench_with_input(
        BenchmarkId::new("array_binary", "last"),
        &last_input,
        |b, s| b.iter(|| array_binary(s)),
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
