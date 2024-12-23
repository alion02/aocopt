use super::*;

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u64 {
    0
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
                history = transmute(
                    mask8x32::from_bitmask(0b11101110111011101110111011101110)
                        .select(transmute::<_, u8x32>(history), transmute::<_, u8x32>(diff)),
                );
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
                        19 * 19,
                        1,
                        19 * 19,
                        1,
                        19 * 19,
                        1,
                        19 * 19,
                        1,
                        19 * 19,
                        1,
                        19 * 19,
                        1,
                        19 * 19,
                        1,
                        19 * 19,
                        1,
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
                let last_sold = last_sold.get_unchecked_mut(seq_id);
                if *last_sold != monkey_id {
                    *last_sold = monkey_id;
                    *bananas.get_unchecked_mut(seq_id) += curr[i] as u16;
                }
            }
        }
        monkey_id += 1;
        if done {
            break;
        }
    }
    *bananas.iter().max().unwrap_unchecked() as u32
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
        let s = read_to_string("./inputs/22.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/22p1.txt").unwrap(),);
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/22shuf.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/22p2.txt").unwrap(),);
    }
}
