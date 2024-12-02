#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::identity_op
)]

use std::{fmt::Display, simd::prelude::*};

#[repr(align(32))]
#[derive(Clone, Copy)]
struct Arr([u8; 90032]);

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

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &str) -> impl Display {
    let s = s.as_bytes();

    let left = get_arr!();
    let right = get_arr!();

    for i in (0..).step_by(14).take(1000) {
        let a = *s.get_unchecked(i + 0) as u32 * 10000
            + *s.get_unchecked(i + 1) as u32 * 1000
            + *s.get_unchecked(i + 2) as u32 * 100
            + *s.get_unchecked(i + 3) as u32 * 10
            + *s.get_unchecked(i + 4) as u32 * 1
            - 543328;

        *left.get_unchecked_mut(a as usize) += 1;

        let b = *s.get_unchecked(i + 8) as u32 * 10000
            + *s.get_unchecked(i + 9) as u32 * 1000
            + *s.get_unchecked(i + 10) as u32 * 100
            + *s.get_unchecked(i + 11) as u32 * 10
            + *s.get_unchecked(i + 12) as u32 * 1
            - 543328;

        *right.get_unchecked_mut(b as usize) += 1;
    }

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
    let s = s.as_bytes();

    let left = get_arr!();
    let right = get_arr!();

    for i in (0..).step_by(14).take(1000) {
        let a = *s.get_unchecked(i + 0) as u32 * 10000
            + *s.get_unchecked(i + 1) as u32 * 1000
            + *s.get_unchecked(i + 2) as u32 * 100
            + *s.get_unchecked(i + 3) as u32 * 10
            + *s.get_unchecked(i + 4) as u32 * 1
            - 543328;

        *left.get_unchecked_mut(a as usize) += 1;

        let b = *s.get_unchecked(i + 8) as u32 * 10000
            + *s.get_unchecked(i + 9) as u32 * 1000
            + *s.get_unchecked(i + 10) as u32 * 100
            + *s.get_unchecked(i + 11) as u32 * 10
            + *s.get_unchecked(i + 12) as u32 * 1
            - 543328;

        *right.get_unchecked_mut(b as usize) += 1;
    }

    let mut i = 0;

    let mut sum = 0u32;

    for _ in 0..1000 {
        while *left.get_unchecked(i) == 0 {
            i += 1;
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
