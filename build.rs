#![allow(unused)]

use std::{collections::HashMap, env::var, fs::File, io::Write, path::PathBuf};

fn write_lut_d2() {
    let mut lut = vec![255; 1 << 26].into_boxed_slice();

    for (i, vec) in lut.chunks_exact_mut(32).enumerate() {
        let mut mask = (!i & 1) << 1 | i << 2 | 1 << 23;
        let mut i = 0;
        let mut j = 0;

        macro_rules! set {
            ($i:expr, $j:expr) => {
                vec[$j + $i / 16 * 16] = ($i % 16) as u8;
            };
        }

        loop {
            let num_len = mask.trailing_zeros();
            if num_len == 1 {
                set!(i, j + 1);
                i += 2;
            } else if num_len == 2 {
                set!(i, j);
                set!(i + 1, j + 1);
                i += 3;
            } else {
                break;
            }

            j += 2;
            if j == 16 {
                break;
            }

            mask >>= num_len + 1;
        }
    }

    let mut path: PathBuf = var("OUT_DIR").unwrap().into();
    path.push("day2lut.bin");

    File::create(path).unwrap().write_all(&lut).unwrap();
}

fn write_lut_d3() {
    let mut lut = vec![255; 1 << 11].into_boxed_slice();

    for (i, vec) in lut.chunks_exact_mut(16).enumerate() {
        let mut mask = i ^ 0xFF;
        let first_len = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        mask >>= first_len + 1;
        let second_len = mask.trailing_zeros() as usize;
        if !(1..=3).contains(&first_len) || !(1..=3).contains(&second_len) {
            continue;
        }
        let first_num = 4;
        let comma = first_num + first_len;
        let second_num = comma + 1;
        let close = second_num + second_len;
        for i in 0..first_len {
            vec[3 - first_len + i] = i as u8 + first_num as u8;
        }
        for i in 0..second_len {
            vec[4 + 3 - second_len + i] = i as u8 + second_num as u8;
        }
        vec[8] = 0;
        vec[9] = 1;
        vec[10] = 2;
        vec[11] = 3;
        vec[12] = comma as u8;
        vec[13] = close as u8;
    }

    let mut path: PathBuf = var("OUT_DIR").unwrap().into();
    path.push("day3lut.bin");

    File::create(path).unwrap().write_all(&lut).unwrap();
}

fn write_lut_d11() {
    macro_rules! write {
        ($d:expr, $u:ty) => {
            let cache = &mut HashMap::new();
            let lut = (0..10_000_000)
                .flat_map(|s| {
                    fn process_stone(cache: &mut HashMap<(u64, u32), $u>, s: u64, d: u32) -> $u {
                        if d == 0 {
                            return 1;
                        }

                        if let Some(v) = cache.get(&(s, d)) {
                            return *v;
                        }

                        let res = if s == 0 {
                            process_stone(cache, 1, d - 1)
                        } else {
                            let digits = s.ilog10() + 1;
                            if digits % 2 == 0 {
                                let div = 10u64.pow(digits / 2);
                                process_stone(cache, s / div, d - 1) + process_stone(cache, s % div, d - 1)
                            } else {
                                process_stone(cache, s * 2024, d - 1)
                            }
                        };

                        cache.insert((s, d), res);
                        res
                    }

                    process_stone(cache, s, $d).to_ne_bytes()
                })
                .collect::<Vec<_>>();

            let mut path: PathBuf = var("OUT_DIR").unwrap().into();
            path.push(format!("day11_depth{}.bin", $d));
            File::create(path).unwrap().write_all(&lut).unwrap();
        };
    }

    write!(25, u32);
    write!(75, u64);
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    // write_lut_d2();
    // write_lut_d3();
    // write_lut_d11();
}
