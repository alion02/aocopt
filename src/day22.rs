use super::*;

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u64 {
    let r = s.as_ptr_range();
    let mut end = r.end.sub(16).cast::<u8x16>();
    let mut ptr = r.start.cast::<u8x16>();
    static mut SCRATCH: u8x32 = Simd::from_array([b'\n'; 32]);
    let mut finishing = false;
    let mut done = false;
    let mut sums = u64x4::splat(0);
    loop {
        let mut state = u32x8::splat(0);
        let mut i = 7;
        loop {
            let chunk = ptr.read_unaligned() - Simd::splat(b'0');
            let mask = _mm_movemask_epi8(chunk.into()) as u32;
            let len = mask.trailing_zeros() as usize;
            ptr = ptr.byte_add(len + 1);
            static SHUFFLE: [u8; 32] = [
                !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
                12, 13, 14, 15,
            ];
            let chunk = _mm_shuffle_epi8(
                chunk.into(),
                SHUFFLE.as_ptr().add(len).cast::<__m128i>().read_unaligned(),
            );
            let chunk = _mm_maddubs_epi16(
                chunk,
                u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]).into(),
            );
            let chunk = _mm_madd_epi16(chunk, u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]).into());
            let chunk = _mm_packus_epi32(chunk, chunk);
            let chunk: u32x4 = _mm_madd_epi16(
                chunk,
                u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]).into(),
            )
            .into();
            state[i] = chunk[3];
            if ptr >= end {
                if finishing {
                    done = true;
                    break;
                }
                finishing = true;
                let scratch = (&raw mut SCRATCH).cast::<u8x16>();
                scratch.write(end.read_unaligned());
                ptr = scratch.byte_offset(ptr.byte_offset_from(end));
                end = scratch.byte_add(16);
            }
            i = i.wrapping_sub(1);
            if i == !0 {
                break;
            }
        }

        let state_0_8 = state >> 0 | state << 24;
        let state_8_16 = state >> 8 | state << 16;
        let state_16_24 = state >> 16 | state << 8;

        let end_state = state_0_8 >> 0 & Simd::splat(0x61A765)
            ^ state_0_8 >> 1 & Simd::splat(0xC2F82D)
            ^ state_0_8 >> 2 & Simd::splat(0x286D53)
            ^ state_0_8 >> 3 & Simd::splat(0x44F679)
            ^ state_0_8 >> 4 & Simd::splat(0x4D6BE8)
            ^ state_0_8 >> 5 & Simd::splat(0x118005)
            ^ state_0_8 >> 6 & Simd::splat(0x5F19F2)
            ^ state_0_8 >> 7 & Simd::splat(0xF03667)
            ^ state_8_16 >> 0 & Simd::splat(0xCEA653)
            ^ state_8_16 >> 1 & Simd::splat(0xAFA201)
            ^ state_8_16 >> 2 & Simd::splat(0xFD0D29)
            ^ state_8_16 >> 3 & Simd::splat(0x949200)
            ^ state_8_16 >> 4 & Simd::splat(0x49A994)
            ^ state_8_16 >> 5 & Simd::splat(0x021673)
            ^ state_8_16 >> 6 & Simd::splat(0xB4C5BF)
            ^ state_8_16 >> 7 & Simd::splat(0x1E0AAF)
            ^ state_16_24 >> 0 & Simd::splat(0x7CAB00)
            ^ state_16_24 >> 1 & Simd::splat(0x95BA48)
            ^ state_16_24 >> 2 & Simd::splat(0x49F04C)
            ^ state_16_24 >> 3 & Simd::splat(0x9A8320)
            ^ state_16_24 >> 4 & Simd::splat(0xB69D39)
            ^ state_16_24 >> 5 & Simd::splat(0x6A2085)
            ^ state_16_24 >> 6 & Simd::splat(0xD13C84)
            ^ state_16_24 >> 7 & Simd::splat(0x1C9E15);

        let a = simd_swizzle!(end_state, [0, 2, 4, 6]);
        let b = simd_swizzle!(end_state, [1, 3, 5, 7]);
        sums += (a + b).cast();

        if done {
            break;
        }
    }
    sums.reduce_sum()
}

#[inline]
#[repr(align(64))]
unsafe fn inner2(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut end = r.end.sub(16).cast::<u8x16>();
    let mut ptr = r.start.cast::<u8x16>();
    let mut bananas = [0u16; 19usize.pow(4)];
    static mut LAST_SOLD: [[u16; 130321]; 8] = [[0u16; 19usize.pow(4)]; 8];
    let last_sold = &mut LAST_SOLD;
    let mut monkey_id = 1;
    static mut SCRATCH: u8x32 = Simd::from_array([b'\n'; 32]);
    let mut finishing = false;
    let mut done = false;
    loop {
        let mut state = u32x8::splat(0);
        let mut i = 7;
        loop {
            let chunk = ptr.read_unaligned() - Simd::splat(b'0');
            let mask = _mm_movemask_epi8(chunk.into()) as u32;
            let len = mask.trailing_zeros() as usize;
            ptr = ptr.byte_add(len + 1);
            static SHUFFLE: [u8; 32] = [
                !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, !0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
                12, 13, 14, 15,
            ];
            let chunk = _mm_shuffle_epi8(
                chunk.into(),
                SHUFFLE.as_ptr().add(len).cast::<__m128i>().read_unaligned(),
            );
            let chunk = _mm_maddubs_epi16(
                chunk,
                u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]).into(),
            );
            let chunk = _mm_madd_epi16(chunk, u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]).into());
            let chunk = _mm_packus_epi32(chunk, chunk);
            let chunk: u32x4 = _mm_madd_epi16(
                chunk,
                u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]).into(),
            )
            .into();
            state[i] = chunk[3];
            if ptr >= end {
                if finishing {
                    done = true;
                    break;
                }
                finishing = true;
                let scratch = (&raw mut SCRATCH).cast::<u8x16>();
                scratch.write(end.read_unaligned());
                ptr = scratch.byte_offset(ptr.byte_offset_from(end));
                end = scratch.byte_add(16);
            }
            i = i.wrapping_sub(1);
            if i == !0 {
                break;
            }
        }
        let mut history = u32x8::splat(0);
        let mut prev;
        let mut curr = state % Simd::splat(10);
        macro_rules! step {
            () => {{
                state ^= state << 6 & Simd::splat(0xFFFFFF);
                state ^= state >> 5;
                state ^= state << 11 & Simd::splat(0xFFFFFF);
                prev = curr;
                curr = state % Simd::splat(10);
                let diff = Simd::splat(9) + curr - prev;
                history <<= 8;
                history |= diff;
                let chunk = _mm256_maddubs_epi16(
                    history.into(),
                    u8x32::from_array([
                        19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19,
                        1, 19, 1, 19, 1,
                    ])
                    .into(),
                );
                let chunk: u32x8 = _mm256_madd_epi16(
                    chunk,
                    u16x16::from_array([
                        19 * 19 * 2,
                        1 * 2,
                        19 * 19 * 2,
                        1 * 2,
                        19 * 19 * 2,
                        1 * 2,
                        19 * 19 * 2,
                        1 * 2,
                        19 * 19 * 2,
                        1 * 2,
                        19 * 19 * 2,
                        1 * 2,
                        19 * 19 * 2,
                        1 * 2,
                        19 * 19 * 2,
                        1 * 2,
                    ])
                    .into(),
                )
                .into();
                chunk
            }};
        }

        step!();
        step!();
        step!();
        for _ in 0..1997 {
            let seq_ids = step!();
            for i in 0..8 {
                let last_sold = &mut last_sold[i];
                let seq_id = seq_ids[i] as usize;
                let last_sold = last_sold.as_mut_ptr().byte_add(seq_id);
                if likely(*last_sold != monkey_id) {
                    *last_sold = monkey_id;
                    *bananas.as_mut_ptr().byte_add(seq_id) += curr[i] as u16;
                }
            }
        }
        monkey_id += 1;
        if done {
            break;
        }
    }
    bananas.iter().fold(0, |a, &b| a.max(b)) as u32
}

#[inline]
pub fn part1(s: &str) -> u64 {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> u32 {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/22shuf.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/22p1.txt").unwrap(),);
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/22.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/22p2.txt").unwrap(),);
    }
}
