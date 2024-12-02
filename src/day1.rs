#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::identity_op,
    clippy::zero_prefixed_literal
)]

use std::{
    arch::x86_64::{__m256i, _mm256_madd_epi16, _mm256_maddubs_epi16},
    fmt::Display,
    simd::prelude::*,
};

#[repr(align(32))]
#[derive(Clone, Copy)]
struct Arr([u8; 90032]);

#[inline]
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn preprocess(s: &str, left: &mut [u8; 90032], right: &mut [u8; 90032]) {
    let s = s.as_bytes();

    let mut i = 0;

    while i < 14000 - 28 {
        let chunk = (s.get_unchecked(i) as *const _ as *const u8x32).read_unaligned();
        let adj_chunk = simd_swizzle!(chunk, [
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 13, 13, //
            14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 27, 27, //
        ]) - u8x32::from_array([
            b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0', //
            b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0', //
            b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0', //
            b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0', //
        ]);

        let s1 = _mm256_maddubs_epi16(
            adj_chunk.into(),
            u8x32::from_array([
                10, 1, 10, 1, 1, 0, 0, 0, 10, 1, 10, 1, 1, 0, 0, 0, //
                10, 1, 10, 1, 1, 0, 0, 0, 10, 1, 10, 1, 1, 0, 0, 0, //
            ])
            .into(),
        );

        let s2: u16x16 = _mm256_madd_epi16(
            s1,
            u16x16::from_array([100, 1, 1, 0, 100, 1, 1, 0, 100, 1, 1, 0, 100, 1, 1, 0]).into(),
        )
        .into();

        let s3: __m256i =
            simd_swizzle!(s2, [0, 2, 4, 6, 0, 2, 4, 6, 8, 10, 12, 14, 8, 10, 12, 14]).into();

        let s4: u32x8 = _mm256_madd_epi16(
            s3,
            u16x16::from_array([10, 1, 10, 1, 0, 0, 0, 0, 10, 1, 10, 1, 0, 0, 0, 0]).into(),
        )
        .into();

        let a1 = s4[0];
        let b1 = s4[1];
        let a2 = s4[4];
        let b2 = s4[5];

        *left.get_unchecked_mut(a1 as usize) += 1;
        *left.get_unchecked_mut(a2 as usize) += 1;
        *right.get_unchecked_mut(b1 as usize) += 1;
        *right.get_unchecked_mut(b2 as usize) += 1;

        i += 28;
    }

    let chunk = (s.get_unchecked(i - 4) as *const _ as *const u8x32).read_unaligned();
    let adj_chunk = simd_swizzle!(chunk, [
        00 + 4,
        01 + 4,
        02 + 4,
        03 + 4,
        04 + 4,
        05 + 4,
        06 + 4,
        07 + 4,
        08 + 4,
        09 + 4,
        10 + 4,
        11 + 4,
        12 + 4,
        13 + 4,
        13 + 4,
        13 + 4,
        14 + 4,
        15 + 4,
        16 + 4,
        17 + 4,
        18 + 4,
        19 + 4,
        20 + 4,
        21 + 4,
        22 + 4,
        23 + 4,
        24 + 4,
        25 + 4,
        26 + 4,
        27 + 4,
        27 + 4,
        27 + 4,
    ]) - u8x32::from_array([
        b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0', //
        b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0', //
        b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0', //
        b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0', //
    ]);

    let s1 = _mm256_maddubs_epi16(
        adj_chunk.into(),
        u8x32::from_array([
            10, 1, 10, 1, 1, 0, 0, 0, 10, 1, 10, 1, 1, 0, 0, 0, //
            10, 1, 10, 1, 1, 0, 0, 0, 10, 1, 10, 1, 1, 0, 0, 0, //
        ])
        .into(),
    );

    let s2: u16x16 = _mm256_madd_epi16(
        s1,
        u16x16::from_array([100, 1, 1, 0, 100, 1, 1, 0, 100, 1, 1, 0, 100, 1, 1, 0]).into(),
    )
    .into();

    let s3: __m256i =
        simd_swizzle!(s2, [0, 2, 4, 6, 0, 2, 4, 6, 8, 10, 12, 14, 8, 10, 12, 14]).into();

    let s4: u32x8 = _mm256_madd_epi16(
        s3,
        u16x16::from_array([10, 1, 10, 1, 0, 0, 0, 0, 10, 1, 10, 1, 0, 0, 0, 0]).into(),
    )
    .into();

    let a1 = s4[0];
    let b1 = s4[1];
    let a2 = s4[4];
    let b2 = s4[5];

    *left.get_unchecked_mut(a1 as usize) += 1;
    *left.get_unchecked_mut(a2 as usize) += 1;
    *right.get_unchecked_mut(b1 as usize) += 1;
    *right.get_unchecked_mut(b2 as usize) += 1;
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &str) -> impl Display {
    static mut ARRAYS: [Arr; 128] = [Arr([0; 90032]); 128];
    static mut CLEAN_ARR: usize = 128;

    macro_rules! get_arr {
        () => {
            if CLEAN_ARR > 0 {
                CLEAN_ARR -= 1;
                &mut ARRAYS[CLEAN_ARR].0
            } else {
                ARRAYS[0].0.fill(0);
                &mut ARRAYS[0].0
            }
        };
    }

    let left = get_arr!();
    let right = get_arr!();

    preprocess(s, left, right);

    let mut i = 0;
    let mut j = 0;

    let mut sum = 0;

    for _ in 0..1000 {
        loop {
            let left_chunk = (left.get_unchecked(i) as *const _ as *const u8x32).read_unaligned();
            if left_chunk.reduce_or() != 0 {
                i += left_chunk
                    .simd_ne(Simd::splat(0))
                    .to_bitmask()
                    .trailing_zeros() as usize;
                break;
            }
            i += 32;
        }

        loop {
            let right_chunk = (right.get_unchecked(j) as *const _ as *const u8x32).read_unaligned();
            if right_chunk.reduce_or() != 0 {
                j += right_chunk
                    .simd_ne(Simd::splat(0))
                    .to_bitmask()
                    .trailing_zeros() as usize;
                break;
            }
            j += 32;
        }

        sum += i.abs_diff(j) as u32;

        *left.get_unchecked_mut(i) -= 1;
        *right.get_unchecked_mut(j) -= 1;
    }

    sum
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &str) -> impl Display {
    static mut ARRAYS: [Arr; 128] = [Arr([0; 90032]); 128];
    static mut CLEAN_ARR: usize = 128;

    macro_rules! get_arr {
        () => {
            if CLEAN_ARR > 0 {
                CLEAN_ARR -= 1;
                &mut ARRAYS[CLEAN_ARR].0
            } else {
                ARRAYS[0].0.fill(0);
                &mut ARRAYS[0].0
            }
        };
    }

    let left = get_arr!();
    let right = get_arr!();

    preprocess(s, left, right);

    let mut i = 0;

    let mut sum = 0u32;

    for _ in 0..1000 {
        loop {
            let left_chunk = (left.get_unchecked(i) as *const _ as *const u8x32).read_unaligned();
            if left_chunk.reduce_or() != 0 {
                i += left_chunk
                    .simd_ne(Simd::splat(0))
                    .to_bitmask()
                    .trailing_zeros() as usize;
                break;
            }
            i += 32;
        }

        sum += (i as u32 + 10000) * *right.get_unchecked(i) as u32;

        *left.get_unchecked_mut(i) -= 1;
    }

    sum
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s) }
}

pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test() {
        let s = read_to_string("./inputs/1.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/1p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/1p2.txt").unwrap(),
        );
    }
}
