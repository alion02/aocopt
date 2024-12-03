use std::{
    arch::x86_64::{
        _mm256_movemask_epi8, _mm256_shuffle_epi8, _mm_maddubs_epi16, _mm_movemask_epi8,
        _mm_shuffle_epi8, _pext_u32,
    },
    hint::black_box,
};

use super::*;

static LUT: [u8x32; 1 << 21] =
    unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day2lut.bin"))) };

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[no_mangle]
unsafe fn inner1(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut i = 0;

    let mut passed = 1000;

    let lut = LUT.as_ptr();

    loop {
        passed -= 1;
        let chunk = (s.get_unchecked(i) as *const _ as *const u8x32).read_unaligned();
        let is_newline = chunk.simd_eq(Simd::splat(b'\n'));
        let newline_mask = is_newline.to_bitmask() as u32;
        let line_len = newline_mask.trailing_zeros();
        let normalized = chunk - Simd::splat(b'0');
        let non_digit_mask = _mm256_movemask_epi8(normalized.into()) as u32;
        let line_mask = !newline_mask & (newline_mask - 1);
        let space_line_mask = non_digit_mask & line_mask;
        let pext_mask = space_line_mask + (space_line_mask >> 1);
        let lane_mask = _pext_u32(space_line_mask, pext_mask);
        let lut_offset = (non_digit_mask & 0x7FFFFC) << 3;
        let shuf_idx = lut.byte_add(lut_offset as usize).read();
        let shuffled: u8x32 = _mm256_shuffle_epi8(normalized.into(), shuf_idx.into()).into();
        let shuffled: u8x16 = simd_swizzle!(shuffled, [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        ]) | simd_swizzle!(shuffled, [
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        ]);
        let numbers: i16x8 = _mm_maddubs_epi16(
            shuffled.into(),
            u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]).into(),
        )
        .into();
        let shifted_down = simd_swizzle!(numbers, Simd::splat(0), [1, 2, 3, 4, 5, 6, 7, 8]);
        let diffs = numbers - shifted_down;
        let lt = diffs.simd_lt(Simd::splat(4));
        let gt = diffs.simd_gt(Simd::splat(-4));
        let nonzero = diffs.simd_ne(Simd::splat(0));
        let signs = _mm_movemask_epi8(diffs.into()) as u32 & lane_mask;
        let pass = _mm_movemask_epi8((lt & gt & nonzero).to_int().into()) as u32 & lane_mask;
        i += line_len as usize + 1;
        if (signs == lane_mask || signs == 0) && pass == lane_mask {
            break;
        }
    }

    passed
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
