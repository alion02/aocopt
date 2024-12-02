#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::identity_op
)]

use std::fmt::Display;

#[thread_local]
static mut ARRAYS: [[u8; 90000]; 128] = [[0; 90000]; 128];

#[thread_local]
static mut CLEAN: usize = 128;

macro_rules! get_arr {
    () => {
        if CLEAN > 0 {
            CLEAN -= 1;
            &mut ARRAYS[CLEAN]
        } else {
            ARRAYS[0].fill(0);
            &mut ARRAYS[0]
        }
    };
}

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

    'outer: loop {
        while left[i] == 0 {
            i += 1;

            if i == 90000 {
                break 'outer;
            }
        }

        while right[j] == 0 {
            j += 1;

            if j == 90000 {
                break 'outer;
            }
        }

        sum += i.abs_diff(j) as u32;

        left[i] -= 1;
        right[j] -= 1;
    }

    sum
}

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

    'outer: loop {
        while left[i] == 0 {
            i += 1;

            if i == 90000 {
                break 'outer;
            }
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
