#![feature(portable_simd)]

use std::{
    arch::x86_64::_mm256_i32gather_epi32, fs::read_to_string, hint::black_box, simd::prelude::*,
};

use aocopt::day2::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};

// pub fn day2(c: &mut Criterion) {
//     let s = read_to_string("./inputs/2.txt").unwrap();
//     let s = s.as_str();

//     c.bench_function("day2 part1", |b| b.iter(|| part1(black_box(s))));
//     c.bench_function("day2 part2", |b| b.iter(|| part2(black_box(s))));

//     assert_eq!(
//         part1(s).to_string(),
//         read_to_string("./outputs/2p1.txt").unwrap(),
//     );
//     assert_eq!(
//         part2(s).to_string(),
//         read_to_string("./outputs/2p2.txt").unwrap(),
//     );
// }

pub fn silly(c: &mut Criterion) {
    let a = [0; 256].as_slice().as_ptr();
    c.bench_function("thing", |b| {
        b.iter(|| unsafe {
            _mm256_i32gather_epi32::<1>(
                black_box(a),
                u32x8::from_array([0, 4, 8, 12, 16, 20, 24, 28]).into(),
            )
        })
    });
}

criterion_group!(benches, silly);
criterion_main!(benches);
