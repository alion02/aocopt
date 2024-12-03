use std::intrinsics::unlikely;

use super::*;

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[no_mangle]
unsafe fn inner1(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut i = 0;

    let mut sum = 0;

    loop {
        macro_rules! step {
            ($value:pat) => {
                let digit1 = *s.get_unchecked(i) as u32;
                let char2 = *s.get_unchecked(i + 1) as u32;

                let ($value, step) = if char2 < 48 {
                    (digit1 - 48, 2)
                } else {
                    (digit1 * 10 + char2 - 528, 3)
                };

                i += step;
            };
        }

        step!(mut v0);
        step!(mut v1);

        if v1 as i32 - v0 as i32 > 0 {
            loop {
                let diff = v1.wrapping_sub(v0).wrapping_sub(1);

                if diff > 2 {
                    let chunk =
                        (s.get_unchecked(i - 1) as *const _ as *const u8x32).read_unaligned();

                    let newlines = chunk.simd_eq(Simd::splat(b'\n')).to_bitmask() as u32;

                    i += newlines.trailing_zeros() as usize;

                    break;
                }

                if *s.get_unchecked(i - 1) == b'\n' {
                    sum += 1;
                    break;
                }

                step!(next);
                v0 = v1;
                v1 = next;
            }
        } else {
            loop {
                let diff = v0.wrapping_sub(v1).wrapping_sub(1);

                if diff > 2 {
                    let chunk =
                        (s.get_unchecked(i - 1) as *const _ as *const u8x32).read_unaligned();

                    let newlines = chunk.simd_eq(Simd::splat(b'\n')).to_bitmask() as u32;

                    i += newlines.trailing_zeros() as usize;

                    break;
                }

                if *s.get_unchecked(i - 1) == b'\n' {
                    sum += 1;
                    break;
                }

                step!(next);
                v0 = v1;
                v1 = next;
            }
        }

        if i == s.len() {
            break;
        }
    }

    sum
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut i = 0;

    let mut sum = 0;

    // loop {
    //     macro_rules! step {
    //         ($value:pat) => {
    //             let digit1 = *s.get_unchecked(i) as u32;
    //             let char2 = *s.get_unchecked(i + 1) as u32;

    //             let ($value, step) = if char2 < 48 {
    //                 (digit1 - 48, 2)
    //             } else {
    //                 (digit1 * 10 + char2 - 528, 3)
    //             };

    //             i += step;
    //         };
    //     }

    //     step!(v0);
    //     step!(v1);
    //     step!(v2);

    //     let mut prev = 0;
    //     let mut sign = 0;

    //     for num_idx in 0.. {
    //         step!(value);

    //         if num_idx > 0 {
    //             let diff = value.wrapping_sub(prev) as i32;

    //             if num_idx == 1 {
    //                 sign = diff;
    //             }

    //             if diff ^ sign < 0 || value.abs_diff(prev).wrapping_sub(1) > 2 {
    //                 let chunk =
    //                     (s.get_unchecked(i - 1) as *const _ as *const u8x32).read_unaligned();

    //                 let newlines = chunk.simd_eq(Simd::splat(b'\n')).to_bitmask() as u32;

    //                 i += newlines.trailing_zeros() as usize;

    //                 break;
    //             }
    //         }

    //         prev = value;

    //         if *s.get_unchecked(i - 1) == b'\n' {
    //             sum += 1;
    //             break;
    //         }
    //     }

    //     if i == s.len() {
    //         break;
    //     }
    // }

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
        let s = read_to_string("./inputs/2.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/2p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/2p2.txt").unwrap(),
        );
    }
}
