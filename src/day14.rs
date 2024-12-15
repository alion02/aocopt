use std::simd::StdFloat;

use super::*;

unsafe fn inner1(s: &[u8]) -> u32 {
    struct Data {
        shuffle: [i8x16; 65536],
        mults: [i8x16; 64],
        scratch: [u8; 32],
    }

    static mut DATA: Data = unsafe {
        let mut shuffle = [[-1i8; 16]; 65536];
        let mut mults = [[-1i8; 16]; 64];

        Data {
            shuffle: transmute(shuffle),
            mults: transmute(mults),
            scratch: [0; 32],
        }
    };

    let mut start = s.as_ptr().cast::<u8x16>();
    let mut ptr = start.byte_add(s.len() - 17);
    let data = &mut DATA;
    let mults100 = u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]).into();
    let dims = f32x8::from_array([101., 103., 101., 103., 101., 103., 101., 103.]);
    loop {
        macro_rules! parse_line {
            () => {{
                let chunk = ptr.read_unaligned();
                let minus_mask = chunk.simd_eq(Simd::splat(b'-')).to_bitmask() as usize;
                let minus_mask = (minus_mask & 0x7E00) >> 5;
                let mults10 = data.mults.as_ptr().byte_add(minus_mask).read();
                let chunk = chunk - Simd::splat(b'0');
                let digit_mask = chunk.simd_le(Simd::splat(9)).to_bitmask() as usize;
                ptr = ptr.wrapping_byte_offset(digit_mask.trailing_zeros() as isize - 19);
                let shuffle = data.shuffle.as_ptr().add(digit_mask).read();
                let mut chunk: u8x16 = _mm_shuffle_epi8(chunk.into(), shuffle.into()).into();
                if unlikely(digit_mask == 0b1100110000111011) {
                    chunk[1] = 1;
                }
                let chunk = _mm_maddubs_epi16(chunk.into(), mults10.into());
                let chunk = _mm_madd_epi16(chunk, mults100);
                i32x4::from(chunk)
            }};
        }
        let c1 = parse_line!();
        let c2 = parse_line!();
        let c3 = parse_line!();
        if ptr < start {
            start = data.scratch.as_ptr().cast::<u8x16>().byte_offset(16);
            ptr = start.byte_offset(ptr as *const u8 as isize - start as *const u8 as isize);
        }
        let c4 = parse_line!();
        let pos1 = simd_swizzle!(c1, c2, [0, 1, 4, 5]);
        let pos2 = simd_swizzle!(c3, c4, [0, 1, 4, 5]);
        let vel1 = simd_swizzle!(c1, c2, [2, 3, 6, 7]);
        let vel2 = simd_swizzle!(c3, c4, [2, 3, 6, 7]);
        let pos = simd_swizzle!(pos1, pos2, [0, 1, 2, 3, 4, 5, 6, 7]);
        let vel = simd_swizzle!(vel1, vel2, [0, 1, 2, 3, 4, 5, 6, 7]);
        let sum = pos.cast::<f32>() + vel.cast::<f32>();
        let divided = sum / dims;
        let rem = ((sum - divided.floor()) * dims).round();

        if ptr < start {
            break;
        }
    }

    0
}

unsafe fn inner2(s: &[u8]) -> u32 {
    0
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
    fn p1() {
        let s = read_to_string("./inputs/14.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/14p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/14.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/14p2.txt").unwrap(),);
    }
}
