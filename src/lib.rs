#![feature(thread_local, portable_simd)]
#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::identity_op,
    clippy::zero_prefixed_literal
)]

#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

pub mod day1;

aoc_lib! { year = 2024 }

use std::{
    arch::x86_64::{__m256i, _mm256_madd_epi16, _mm256_maddubs_epi16},
    fmt::Display,
    mem::{transmute, MaybeUninit},
    simd::prelude::*,
};
