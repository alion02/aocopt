#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::identity_op
)]

use std::fmt::Display;

static mut ARRAYS: [[u8; 90000]; 128] = [[0; 90000]; 128];
static mut CLEAN_ARR: usize = 128;

static mut BITS: [[u64; 1407]; 128] = [[0; 1407]; 128];
static mut CLEAN_BITS: usize = 128;

macro_rules! get_arr {
    () => {
        if CLEAN_ARR > 0 {
            CLEAN_ARR -= 1;
            &mut ARRAYS[CLEAN_ARR]
        } else {
            ARRAYS[0].fill(0);
            &mut ARRAYS[0]
        }
    };
}

macro_rules! get_bits {
    () => {
        if CLEAN_BITS > 0 {
            CLEAN_BITS -= 1;
            &mut BITS[CLEAN_BITS]
        } else {
            BITS[0].fill(0);
            &mut BITS[0]
        }
    };
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &str) -> impl Display {
    let s = s.as_bytes();

    let left = get_arr!();
    let right = get_arr!();

    let left_bits = get_bits!();
    let right_bits = get_bits!();

    for i in (0..).step_by(14).take(1000) {
        let a = *s.get_unchecked(i + 0) as u32 * 10000
            + *s.get_unchecked(i + 1) as u32 * 1000
            + *s.get_unchecked(i + 2) as u32 * 100
            + *s.get_unchecked(i + 3) as u32 * 10
            + *s.get_unchecked(i + 4) as u32 * 1
            - 543328;

        *left.get_unchecked_mut(a as usize) += 1;
        *left_bits.get_unchecked_mut(a as usize / 64) |= 1u64.wrapping_shl(a);

        let b = *s.get_unchecked(i + 8) as u32 * 10000
            + *s.get_unchecked(i + 9) as u32 * 1000
            + *s.get_unchecked(i + 10) as u32 * 100
            + *s.get_unchecked(i + 11) as u32 * 10
            + *s.get_unchecked(i + 12) as u32 * 1
            - 543328;

        *right.get_unchecked_mut(b as usize) += 1;
        *right_bits.get_unchecked_mut(a as usize / 64) |= 1u64.wrapping_shl(b);
    }

    let mut i = 0;
    let mut j = 0;

    let mut sum = 0;

    for _ in 0..1000 {
        while *left.get_unchecked(i) == 0 {
            i += 1;
        }

        while *right.get_unchecked(j) == 0 {
            j += 1;
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
        while left[i] == 0 {
            i += 1;
        }

        sum += (i as u32 + 10000) * right[i] as u32;

        left[i] -= 1;
    }

    sum
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s) }
}

pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s) }
}
