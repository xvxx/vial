use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use vial::{http_parser::parse, Request};
use pprof::{criterion::{PProfProfiler, Output}, flamegraph::Options};

fn fixture(name: &str) -> String {
    fs::read_to_string(name).unwrap()
}

fn parse_fixture(name: &str) -> Request {
    match parse(fixture(name).as_bytes().to_vec()) {
        Ok(request) => request,
        _ => panic!("Expected Status::Complete"),
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple_get", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/simple_GET.txt")))
    });
    c.bench_function("another_get", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/another_GET.txt")))
    });
    c.bench_function("big_get", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/big_GET.txt")))
    });
    c.bench_function("stacked_headers_get", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/stacked_headers_GET.txt")))
    });
    c.bench_function("simple_post", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/simple_POST.txt")))
    });
    c.bench_function("simple_post2", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/simple_POST2.txt")))
    });
    // bad benches don't work!
    // `Benchmarking bad_get: Warming up for 3.0000 sthread 'main' panicked at 'Expected Status::Complete', benches/benchmark.rs:12:14`
    // c.bench_function("bad_get", |b| b.iter(|| parse_fixture("tests/http/bad_GET.txt")));
    // c.bench_function("bad_get2", |b| b.iter(|| parse_fixture("tests/http/bad_GET2.txt")));
    // c.bench_function("bad_post", |b| b.iter(|| parse_fixture("tests/http/bad_POST.txt")));
    // c.bench_function("bad_big_headers", |b| b.iter(|| parse_fixture("tests/http/bad_BIG_HEADERS.txt")));
    // c.bench_function("bad_post2", |b| b.iter(|| parse_fixture("tests/http/bad_POST2.txt")));
    c.bench_function("basic_cookies", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/cookies_basic.txt")))
    });
    c.bench_function("cookies_only_once", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/cookies_assign_only_once.txt")))
    });
    c.bench_function("cookies_escape", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/cookies_escaping.txt")))
    });
    c.bench_function("cookies_ignore_escaping_error_and_return_orig_value", |b| {
        b.iter(|| {
            black_box(parse_fixture(
                "tests/http/cookies_ignore_escaping_error_and_return_orig_value.txt",
            ))
        })
    });
    c.bench_function("cookies_ignore_non_values", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/cookies_ignore_non_values.txt")))
    });
    c.bench_function("cookies_unencoded", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/cookies_unencoded.txt")))
    });
    c.bench_function("json_post", |b| {
        b.iter(|| black_box(parse_fixture("tests/http/json_POST.txt")))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);