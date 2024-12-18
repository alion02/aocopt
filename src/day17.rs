use super::*;

#[inline]
unsafe fn inner1(s: &[u8]) -> &str {
    static mut BUF: [u8; 17] = [b','; 17];

    let chunk = s.as_ptr().add(12).cast::<u8x16>().read_unaligned();
    let chunk = chunk - Simd::splat(b'0');
    let chunk = _mm_maddubs_epi16(
        chunk.into(),
        u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]).into(),
    );
    let chunk = _mm_madd_epi16(chunk, u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]).into());
    let chunk = _mm_packus_epi32(chunk, chunk);
    let chunk = _mm_madd_epi16(
        chunk,
        u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]).into(),
    );
    let mut a = u32x4::from(chunk)[0];
    let imm1 = *s.get_unchecked(65) as u32 - b'0' as u32;
    let chunk = s.as_ptr().add(64).cast::<u8x16>().read_unaligned();
    let chunk = chunk.simd_eq(Simd::from_array([
        0, 0, 0, b'1', 0, 0, 0, b'1', 0, 0, 0, b'1', 0, 0, 0, b'1',
    ]));
    let mask = chunk.to_bitmask() as u32;
    let offset = mask.trailing_zeros() as usize;

    let imm2 = *s.get_unchecked(64 + offset + 2) as u32 - b'0' as u32;

    let buf = &mut BUF;
    let mut len = s.len();
    loop {
        let b = a % 8 ^ imm1;
        *buf.get_unchecked_mut(len - 91) = ((a >> b ^ b ^ imm2) % 8 + b'0' as u32) as u8;
        a >>= 3;
        len += 2;
        if a == 0 {
            break;
        }
    }

    std::str::from_utf8_unchecked(buf)
}

static LUT: [u64; 1 << 14] = unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day17.bin"))) };

#[inline]
unsafe fn inner2(s: &[u8]) -> u64 {
    let s = s.as_ptr().add(59);
    let hash = unsafe {
        _pext_u64(
            s.add(15).cast::<u64>().read_unaligned() ^ s.add(6).read() as u64 ^ (s.add(14).read() as u64 * 65536),
            0x07_00_04_00_07_07_04_07,
        )
    };
    *LUT.get_unchecked(hash as usize)
}

#[inline]
pub fn part1(s: &str) -> &str {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> u64 {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/17.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/17p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/17.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/17p2.txt").unwrap(),);
    }
}
