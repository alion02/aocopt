//                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           .
#![feature(thread_local, portable_simd, core_intrinsics, fn_align)]
#![allow(
    clippy::precedence,
    clippy::missing_transmute_annotations,
    clippy::pointers_in_nomem_asm_block,
    clippy::erasing_op,
    static_mut_refs,
    internal_features,
    clippy::missing_safety_doc,
    clippy::identity_op,
    clippy::zero_prefixed_literal
)]

#[allow(unused)]
use std::{
    arch::{
        asm,
        x86_64::{
            __m128i, __m256i, _bextr2_u32, _mm256_madd_epi16, _mm256_maddubs_epi16, _mm256_movemask_epi8,
            _mm256_shuffle_epi8, _mm_hadd_epi16, _mm_madd_epi16, _mm_maddubs_epi16, _mm_minpos_epu16,
            _mm_movemask_epi8, _mm_packus_epi16, _mm_packus_epi32, _mm_shuffle_epi8, _mm_testc_si128, _pdep_u32,
            _pext_u32, _pext_u64,
        },
    },
    array,
    fmt::Display,
    hint::assert_unchecked,
    intrinsics::{likely, unlikely},
    mem::{offset_of, transmute, MaybeUninit},
    ptr,
    simd::prelude::*,
    slice,
};

#[allow(unused)]
macro_rules! black_box {
    ($thing:expr) => {{
        #[allow(asm_sub_register)]
        {
            let mut thing = $thing;
            asm!(
                "/*{t}*/",
                t = inout(reg) thing,
                options(pure, nomem, preserves_flags, nostack)
            );
            thing
        }
    }};
}

#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

pub mod day22;

// pub mod day21;

// pub mod day20;

// pub mod day19;

// pub mod day18;

// pub mod day17;

// pub mod day16;

// pub mod day15;

// pub mod day14;

// pub mod day13;

// pub mod day12;

// pub mod day11;

// pub mod day10;

// pub mod day9;

// pub mod day8;

// pub mod day7;

// pub mod day6;

// pub mod day5;

// pub mod day4;

// pub mod day3;

// pub mod day2;

// pub mod day1;

aoc_lib! { year = 2024 }
