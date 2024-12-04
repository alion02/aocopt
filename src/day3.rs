use std::arch::x86_64::{_mm_madd_epi16, _mm_shuffle_epi8, _mm_testc_si128};

use memchr::{arch::all::packedpair::HeuristicFrequencyRank, memmem::*};

use super::*;

static LUT: [u8x16; 1 << 7] =
    unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day3lut.bin"))) };

static mut SCRATCH: [u8x32; 5] = [u8x32::from_array([0; 32]); 5];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    let mut end = r.end.sub(77);
    let lut = &LUT;
    let mut sum = Simd::splat(0);
    let mut finishing = false;
    'chunk: loop {
        let chunk = (ptr as *const u8x64).read_unaligned();
        let is_u = chunk.simd_eq(Simd::splat(b'u'));
        let mut u_mask = is_u.to_bitmask();
        loop {
            let u_offset = u_mask.trailing_zeros();
            if u_offset == 64 {
                ptr = ptr.add(64);
                if ptr < end {
                    continue 'chunk;
                }
                if finishing {
                    return sum[0] as u32;
                }
                finishing = true;
                let scratch = SCRATCH.as_mut_ptr();
                (scratch).write((r.end.sub(96) as *const u8x32).read_unaligned());
                (scratch.add(1)).write((r.end.sub(64) as *const u8x32).read_unaligned());
                (scratch.add(2)).write((r.end.sub(32) as *const u8x32).read_unaligned());
                ptr = scratch.add(3).byte_offset(ptr.offset_from(r.end)) as _;
                end = scratch.add(3) as _;
                continue 'chunk;
            }
            u_mask &= u_mask - 1;
            let instruction = (ptr.add(u_offset as _).sub(1) as *const u8x16).read_unaligned();
            let normalized = instruction - Simd::splat(b'0');
            let is_digit = normalized.simd_lt(Simd::splat(10));
            let digit_mask = is_digit.to_bitmask() as u32;
            let lut_idx = (digit_mask & 0x7F0) >> 4;
            let shuffle_idx = *lut.get_unchecked(lut_idx as usize);
            let discombobulated: i8x16 =
                _mm_shuffle_epi8(normalized.into(), shuffle_idx.into()).into();
            let is_correct = discombobulated.simd_eq(Simd::from_array([
                0, 0, 0, 0, 0, 0, 0, 0, 61, 69, 60, -8, -4, -7, 0, 0,
            ]));
            if _mm_testc_si128(
                is_correct.to_int().into(),
                i8x16::from_array([0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, 0, 0]).into(),
            ) == 0
            {
                continue;
            }
            let two_digit = _mm_maddubs_epi16(
                discombobulated.into(),
                u8x16::from_array([100, 10, 1, 0, 100, 10, 1, 0, 100, 10, 1, 0, 100, 10, 1, 0])
                    .into(),
            );
            let three_digit: i32x4 = _mm_madd_epi16(two_digit, i8x16::splat(-1).into()).into();
            sum += three_digit * simd_swizzle!(three_digit, [1, 0, 3, 2]);
        }
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(mut s: &[u8]) -> u32 {
    let mut sum = 0;
    let disable = FinderBuilder::new().build_forward_with_ranker(Ranker, b"don't()");
    let enable = FinderBuilder::new().build_forward_with_ranker(Ranker, b"do()");
    struct Ranker;
    impl HeuristicFrequencyRank for Ranker {
        fn rank(&self, byte: u8) -> u8 {
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 14, 17, 0, 17, 17, 15, 16, 20, 255, 250, 14, 17, 123, 18, 0, 20, 42,
                69, 65, 63, 62, 64, 61, 69, 63, 66, 16, 14, 19, 0, 16, 14, 12, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 14, 17, 0, 0, 20,
                0, 16, 9, 78, 16, 0, 95, 0, 0, 0, 117, 117, 18, 56, 0, 0, 32, 16, 41, 100, 0, 95,
                0, 14, 0, 13, 0, 13, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ][byte as usize]
        }
    }
    loop {
        let Some(i) = disable.find(s) else {
            return sum + inner1(s);
        };

        sum += inner1(&s[..i]);
        s = &s[i + 6..];

        let Some(i) = enable.find(s) else {
            return sum;
        };

        s = &s[i + 4..];
    }
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test() {
        let s = read_to_string("./inputs/3.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/3p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/3p2.txt").unwrap(),
        );
    }
}
