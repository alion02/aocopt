#![feature(thread_local, portable_simd, core_intrinsics)]
#![allow(
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

aoc_lib! { year = 2024 }

use std::{
    arch::x86_64::{__m256i, _mm256_madd_epi16, _mm256_maddubs_epi16},
    fmt::Display,
    mem::{transmute, MaybeUninit},
    simd::prelude::*,
};
