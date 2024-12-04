#![feature(thread_local, portable_simd, core_intrinsics)]
#![allow(
    static_mut_refs,
    internal_features,
    clippy::missing_safety_doc,
    clippy::identity_op,
    clippy::zero_prefixed_literal
)]

#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

pub mod day1;
pub mod day2;
pub mod day3;

aoc_lib! { year = 2024 }

use std::{
    arch::x86_64::{
        __m256i, _mm256_madd_epi16, _mm256_maddubs_epi16, _mm256_movemask_epi8,
        _mm256_shuffle_epi8, _mm_maddubs_epi16, _mm_movemask_epi8, _pext_u32,
    },
    fmt::Display,
    mem::{transmute, MaybeUninit},
    simd::prelude::*,
};
