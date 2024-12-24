use std::hint::unreachable_unchecked;

use super::*;

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut ptr = r.start.cast::<u8x16>();
    let mut graph_arr = [[u8x32::splat(0); 3]; 26 * 26];
    let graph = graph_arr.as_mut_ptr();
    let mut total = 0;
    for i in 0..3380 / 2 {
        let chunk = if i == 0 {
            let chunk = ptr.read_unaligned() - Simd::splat(b'a');
            simd_swizzle!(chunk, [0, 1, 3, 4, 6, 7, 9, 10, 0, 1, 3, 4, 6, 7, 9, 10])
        } else {
            let chunk = ptr.byte_sub(5).read_unaligned() - Simd::splat(b'a');
            simd_swizzle!(chunk, [5, 6, 8, 9, 11, 12, 14, 15, 5, 6, 8, 9, 11, 12, 14, 15])
        };
        let chunk: u16x8 = _mm_maddubs_epi16(
            chunk.into(),
            u8x16::from_array([26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1]).into(),
        )
        .into();
        let mask = chunk - Simd::splat((b't' - b'a') as u16 * 26);
        let mask = mask.simd_lt(Simd::splat(26)).to_int();
        for i in (0..4).step_by(2) {
            let l = chunk[i] as u32;
            let r = chunk[i + 1] as u32;
            macro_rules! link {
                ($a:expr, $b:expr) => {{
                    let o: usize;
                    asm!(
                        "imul {t:e}, {a:e}, 96",
                        "bts dword ptr[{graph} + {t}], {b:e}",
                        t = out(reg) o,
                        graph = in(reg) graph,
                        a = in(reg) $a,
                        b = in(reg) $b,
                        options(nostack),
                    );
                    o
                }};
            }
            let il = link!(l, r);
            let ir = link!(r, l);
            let s = if mask[i] != 0 || mask[i + 1] != 0 {
                let intersection: [u8x32; 3] = array::from_fn(|i| {
                    graph.byte_add(il).cast::<u8x32>().add(i).read() & graph.byte_add(ir).cast::<u8x32>().add(i).read()
                });
                let s: u32 = intersection
                    .iter()
                    .flat_map(|s| s.as_array())
                    .map(|m| m.count_ones())
                    .sum();
                s
            } else {
                let intersection = (graph.byte_add(il).cast::<u32>().byte_add(61).read()
                    & graph.byte_add(ir).cast::<u32>().byte_add(61).read())
                    >> 6;
                intersection.count_ones()
            };
            total += s;
        }
        ptr = ptr.byte_add(12);
    }
    total
}

#[inline]
#[repr(align(64))]
unsafe fn inner2(s: &[u8]) -> &str {
    static mut OUT: [u8; 38] = [b','; 38];
    let r = s.as_ptr_range();
    let mut ptr = r.start.cast::<u8x16>();
    let mut graph_arr = [[u8x32::splat(0); 3]; 26 * 26];
    let graph = graph_arr.as_mut_ptr();
    for i in 0..3380 / 2 {
        let chunk = if i == 0 {
            let chunk = ptr.read_unaligned() - Simd::splat(b'a');
            simd_swizzle!(chunk, [0, 1, 3, 4, 6, 7, 9, 10, 0, 1, 3, 4, 6, 7, 9, 10])
        } else {
            let chunk = ptr.byte_sub(5).read_unaligned() - Simd::splat(b'a');
            simd_swizzle!(chunk, [5, 6, 8, 9, 11, 12, 14, 15, 5, 6, 8, 9, 11, 12, 14, 15])
        };
        let chunk: u16x8 = _mm_maddubs_epi16(
            chunk.into(),
            u8x16::from_array([26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1]).into(),
        )
        .into();
        'next: for i in (0..4).step_by(2) {
            let l = chunk[i] as u32;
            let r = chunk[i + 1] as u32;
            macro_rules! link {
                ($a:expr, $b:expr) => {{
                    let o: usize;
                    asm!(
                        "imul {t:e}, {a:e}, 96",
                        "bts dword ptr[{graph} + {t}], {b:e}",
                        t = out(reg) o,
                        graph = in(reg) graph,
                        a = in(reg) $a,
                        b = in(reg) $b,
                        options(nostack),
                    );
                    o
                }};
            }
            let il = link!(l, r);
            let ir = link!(r, l);
            let intersection: [u8x32; 3] = array::from_fn(|i| {
                graph.byte_add(il).cast::<u8x32>().add(i).read() & graph.byte_add(ir).cast::<u8x32>().add(i).read()
            });
            let s: u32 = intersection
                .iter()
                .flat_map(|s| s.as_array())
                .map(|m| m.count_ones())
                .sum();
            let mut intersection: [u64; 12] = transmute(intersection);
            if s == 11 {
                let union: [u8x32; 3] = array::from_fn(|i| {
                    graph.byte_add(il).cast::<u8x32>().add(i).read() ^ graph.byte_add(ir).cast::<u8x32>().add(i).read()
                });
                let union: [u64; 12] = transmute(union);
                assert_eq!(union.iter().fold(0, |acc, f| acc + f.count_ones()), 2);
                for i in 0..12 {
                    while intersection[i] != 0 {
                        let bit_idx = intersection[i].trailing_zeros() as usize;
                        let mut union = union;
                        union[i] &= !(1 << bit_idx);
                        intersection[i] &= intersection[i] - 1;
                        let idx = bit_idx + i * 64;
                        #[allow(clippy::needless_range_loop)]
                        for j in 0..12 {
                            let t = union[j];
                            let other = graph.add(idx).cast::<u64>().add(j).read();
                            if t & other != t {
                                continue 'next;
                            }
                        }
                    }
                }
                let out = &mut OUT;
                let mut ptr = out.as_mut_ptr();
                static LUT: [u16; 26 * 26] = {
                    let mut lut = [0; 26 * 26];
                    let mut c1 = b'a';
                    let mut i = 0;
                    while c1 <= b'z' {
                        let mut c2 = b'a';
                        while c2 <= b'z' {
                            lut[i] = u16::from_ne_bytes([c1, c2]);
                            c2 += 1;
                            i += 1;
                        }
                        c1 += 1;
                    }
                    lut
                };
                let lut = &LUT;
                for (i, mut mask) in union.into_iter().enumerate() {
                    while mask != 0 {
                        let bit_idx = mask.trailing_zeros() as usize;
                        mask &= mask - 1;
                        let idx = bit_idx + i * 64;
                        ptr.cast::<u16>().write_unaligned(*lut.get_unchecked(idx));
                        ptr = ptr.add(3);
                    }
                }
                return std::str::from_utf8_unchecked(out);
            }
        }
        ptr = ptr.byte_add(12);
    }
    unreachable_unchecked()
}

#[inline]
pub fn part1(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> &str {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/23.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/23p1.txt").unwrap(),);
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/23.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/23p2.txt").unwrap(),);
    }
}
