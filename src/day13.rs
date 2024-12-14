use super::*;

unsafe fn inner1(s: &[u8]) -> i32 {
    static LUT: [i8x16; 128] = {
        let mut lut = [[-1i8; 16]; 128];
        let mut y = 3;
        while y < 6 {
            let mut x = 3;
            while x < 6 {
                let mut y_end = 16;
                let y_start = y_end - y;
                let mut x_end = y_start - 4;
                let x_start = x_end - x;
                let index = (((1 << x_end) - 1 ^ (1 << x_start) - 1) & 0x1FC) / 4;
                let entry = &mut lut[index];
                let mut i = 16;
                while y_start < y_end {
                    y_end -= 1;
                    i -= 1;
                    entry[i] = y_end;
                }
                let mut i = 8;
                while x_start < x_end {
                    x_end -= 1;
                    i -= 1;
                    entry[i] = x_end;
                }
                x += 1;
            }
            y += 1;
        }

        unsafe { transmute(lut) }
    };

    let start = s.as_ptr();
    let mut i = s.len() as isize;
    let mut sum_a = 0;
    let mut sum_b = 0;
    let lut = LUT.as_ptr();
    let mults10 = u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]).into();
    let mults100 = u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]).into();
    let mults10000 = u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]).into();
    let swar_mask = 0xF0_F0_FF_FF_FF_FF_F0_F0;
    let swar_mult = (1 << 8) * 10 | (1 << 0);
    let swar_bextr = 8 | 8 << 8;
    assert_unchecked(i >= 0);
    while i >= 0 {
        let chunk = start.offset(i - 17).cast::<u8x16>().read_unaligned();
        let chunk = chunk - Simd::splat(b'0');
        let mask = chunk.simd_lt(Simd::splat(10)).to_bitmask() as usize;
        i = i + mask.trailing_zeros() as isize - 69;
        let shuffle = lut.byte_add((mask & 0x1FC) * 4).read();
        let chunk = _mm_shuffle_epi8(chunk.into(), shuffle.into());
        let chunk = _mm_maddubs_epi16(chunk, mults10);
        let chunk = _mm_madd_epi16(chunk, mults100);
        let chunk = _mm_packus_epi32(chunk, chunk);
        let chunk: i32x4 = _mm_madd_epi16(chunk, mults10000).into();
        let px = chunk[0];
        let py = chunk[1];
        let a = start.offset(i + 13).cast::<u64>().read_unaligned();
        let a = a & !swar_mask;
        let a = a.wrapping_mul(swar_mult);
        let ax = _bextr2_u32(a as u32, swar_bextr) as i32;
        let ay = (a >> 56) as i32;
        let b = start.offset(i + 34).cast::<u64>().read_unaligned();
        let b = b & !swar_mask;
        let b = b.wrapping_mul(swar_mult);
        let bx = _bextr2_u32(b as u32, swar_bextr) as i32;
        let by = (b >> 56) as i32;
        let subexpr1 = by * ax - bx * ay;
        if subexpr1 == 0 {
            continue;
        }
        let subexpr2 = by * px - bx * py;
        let a_rem = subexpr2 % subexpr1;
        if a_rem != 0 {
            continue;
        }
        let a_quot = subexpr2 / subexpr1;
        let subexpr3 = bx * subexpr1;
        assert_unchecked(subexpr3 != 0);
        let b_quot = (px * subexpr1 - ax * subexpr2) / subexpr3;
        sum_a += a_quot;
        sum_b += b_quot;
    }

    sum_a * 3 + sum_b
}

unsafe fn inner2(s: &[u8]) -> u64 {
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
        let s = read_to_string("./inputs/13.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/13p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/13.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/13p2.txt").unwrap(),);
    }
}
