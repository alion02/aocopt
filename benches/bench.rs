#![feature(portable_simd)]

use std::{fs::read_to_string, hint::black_box};

use aocopt::day25::part1;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn day25(c: &mut Criterion) {
    let s = read_to_string("./inputs/25.txt").unwrap();
    let s = s.as_str();

    c.bench_function("day25 part1", |b| {
        b.iter(
            #[inline(never)]
            || part1(black_box(s)),
        )
    });

    assert_eq!(part1(s).to_string(), read_to_string("./outputs/25p1.txt").unwrap());
}

criterion_group!(benches, day25);
criterion_main!(benches);
