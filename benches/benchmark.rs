use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use vial::{http_parser::parse, Request};
use pprof::{criterion::{PProfProfiler, Output}, flamegraph::Options};

fn file_to_string(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn parse_fixture(name: &str) -> Request {
    match parse(file_to_string(name).as_bytes().to_vec()) {
        Ok(request) => request,
        _ => panic!("Expected Status::Complete"),
    }
}
fn parse_text(text: &str) -> Request {
    parse(text.as_bytes().to_vec()).unwrap()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple_get_internal", |b|{
        b.iter(||black_box(parse_text(r##"GET / HTTP/1.1
Host: www.codecademy.com

"##)))
    });
    c.bench_function("simple_get", |b| {
        let str = file_to_string("tests/http/simple_GET.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("another_get", |b| {
        let str = file_to_string("tests/http/another_GET.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("big_get", |b| {
        let str = file_to_string("tests/http/big_GET.txt");
        b.iter(|| parse_text(black_box(&str)))

    });
    c.bench_function("stacked_headers_GET", |b| {
        let str = file_to_string("tests/http/stacked_headers_GET.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("simple_post", |b| {
        let str = file_to_string("tests/http/simple_POST.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("simple_post2", |b| {
        let str = file_to_string("tests/http/simple_POST2.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    // bad benches don't work!
    // `Benchmarking bad_get: Warming up for 3.0000 sthread 'main' panicked at 'Expected Status::Complete', benches/benchmark.rs:12:14`
    // c.bench_function("bad_get", |b| b.iter(|| parse_fixture("tests/http/bad_GET.txt")));
    // c.bench_function("bad_get2", |b| b.iter(|| parse_fixture("tests/http/bad_GET2.txt")));
    // c.bench_function("bad_post", |b| b.iter(|| parse_fixture("tests/http/bad_POST.txt")));
    // c.bench_function("bad_big_headers", |b| b.iter(|| parse_fixture("tests/http/bad_BIG_HEADERS.txt")));
    // c.bench_function("bad_post2", |b| b.iter(|| parse_fixture("tests/http/bad_POST2.txt")));
    c.bench_function("basic_cookies", |b| {
        let str = file_to_string("tests/http/cookies_basic.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("cookies_only_once", |b| {
        let str = file_to_string("tests/http/cookies_assign_only_once.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("cookies_escape", |b| {
        let str = file_to_string("tests/http/cookies_escaping.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("cookies_ignore_escaping_error_and_return_orig_value", |b| {
        let str = file_to_string("tests/http/cookies_ignore_escaping_error_and_return_orig_value.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("cookies_ignore_non_values", |b| {
        let str = file_to_string("tests/http/cookies_ignore_non_values.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("cookies_unencoded", |b| {
        let str = file_to_string("tests/http/cookies_unencoded.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
    c.bench_function("json_post", |b| {
        let str = file_to_string("tests/http/json_POST.txt");
        b.iter(|| parse_text(black_box(&str)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);