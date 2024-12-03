use std::{env::var, fs::File, io::Write, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let mut lut = vec![255u8; 1 << 26].into_boxed_slice();

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
