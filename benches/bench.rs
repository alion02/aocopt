#![feature(portable_simd)]

use std::{fs::read_to_string, hint::black_box};

use aocopt::day5::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn day5(c: &mut Criterion) {
    let s = read_to_string("./inputs/5.txt").unwrap();
    let s = s.as_str();

    c.bench_function("day5 part1 wip", |b| b.iter(|| part1(black_box(s))));
    // c.bench_function("day5 part2", |b| b.iter(|| part2(black_box(s))));

    // assert_eq!(
    //     part1(s).to_string(),
    //     read_to_string("./outputs/5p1.txt").unwrap(),
    // );
    // assert_eq!(
    //     part2(s).to_string(),
    //     read_to_string("./outputs/5p2.txt").unwrap(),
    // );
}

criterion_group!(benches, day5);
criterion_main!(benches);
