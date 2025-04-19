use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use phf::phf_set;

use rust_bench_test::generate_cases;

static DISALLOW_NEW_FOR_BUILTINS_1: [&str; 13] = [
    "abcdefabcdefabcdefabcdefabcdefapply",
    "abcdefabcdefabcdefabcdefabcdefconstruct",
    "abcdefabcdefabcdefabcdefabcdefdefineProperty",
    "abcdefabcdefabcdefabcdefabcdefdeleteProperty",
    "abcdefabcdefabcdefabcdefabcdefget",
    "abcdefabcdefabcdefabcdefabcdefgetOwnPropertyDescriptor",
    "abcdefabcdefabcdefabcdefabcdefgetPrototypeOf",
    "abcdefabcdefabcdefabcdefabcdefhas",
    "abcdefabcdefabcdefabcdefabcdefisExtensible",
    "abcdefabcdefabcdefabcdefabcdefownKeys",
    "abcdefabcdefabcdefabcdefabcdefpreventExtensions",
    "abcdefabcdefabcdefabcdefabcdefset",
    "abcdefabcdefabcdefabcdefabcdefsetPrototypeOf",
];

static DISALLOW_NEW_FOR_BUILTINS_2: phf::Set<&'static str> = phf_set! {
    "abcdefabcdefabcdefabcdefabcdefapply",
    "abcdefabcdefabcdefabcdefabcdefconstruct",
    "abcdefabcdefabcdefabcdefabcdefdefineProperty",
    "abcdefabcdefabcdefabcdefabcdefdeleteProperty",
    "abcdefabcdefabcdefabcdefabcdefget",
    "abcdefabcdefabcdefabcdefabcdefgetOwnPropertyDescriptor",
    "abcdefabcdefabcdefabcdefabcdefgetPrototypeOf",
    "abcdefabcdefabcdefabcdefabcdefhas",
    "abcdefabcdefabcdefabcdefabcdefisExtensible",
    "abcdefabcdefabcdefabcdefabcdefownKeys",
    "abcdefabcdefabcdefabcdefabcdefpreventExtensions",
    "abcdefabcdefabcdefabcdefabcdefset",
    "abcdefabcdefabcdefabcdefabcdefsetPrototypeOf",
};

fn phf(s: &str) -> bool {
    DISALLOW_NEW_FOR_BUILTINS_2.contains(s)
}

fn array(s: &str) -> bool {
    DISALLOW_NEW_FOR_BUILTINS_1.contains(&s)
}

fn benchmark(c: &mut Criterion) {
    let cases = generate_cases(&DISALLOW_NEW_FOR_BUILTINS_1);

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
