#![allow(clippy::pointers_in_nomem_asm_block)]
use super::*;

// perhaps for later use
macro_rules! black_box {
    ($thing:expr) => {{
        let mut thing = $thing;
        asm!(
            "/*{t}*/",
            t = inout(reg) thing,
            options(pure, nomem, preserves_flags, nostack)
        );
        thing
    }};
}

unsafe fn process<const P2: bool>(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    let mut cy = 0usize;

    #[repr(C, align(32))]
    struct Tables {
        _padding1: [u8; 16],
        antinodes: [u64; 150],
        _padding2: [u8; 16],
        frequencies: [[[u8; 2]; 4]; 75],
    }

    static mut TABLES: Tables = Tables {
        _padding1: [0; 16],
        antinodes: [0; 150],
        _padding2: [0; 16],
        frequencies: [[[0; 2]; 4]; 75],
    };

    let Tables {
        antinodes,
        frequencies,
        ..
    } = &mut TABLES;

    antinodes[50..100].fill(0);
    frequencies.fill(Default::default());

    loop {
        let c1 = ptr.cast::<u8x32>().read_unaligned() + Simd::splat(127 - b'.');
        let c2 = ptr.add(18).cast::<u8x32>().read_unaligned() + Simd::splat(127 - b'.');
        let m1 = c1.simd_ge(Simd::splat(128)).to_bitmask();
        let m2 = c2.simd_ge(Simd::splat(128)).to_bitmask();
        let mut mask = m1 | m2 << 18;
        if P2 {
            *antinodes.get_unchecked_mut(50 + cy) |= mask;
        }
        while mask != 0 {
            let cx = mask.trailing_zeros() as usize;
            let bucket = frequencies
                .get_unchecked_mut((ptr.add(cx).read() as usize).unchecked_sub(b'0' as usize));
            let count_bucket = bucket.get_unchecked_mut(3).get_unchecked_mut(0);
            let count = *count_bucket as usize;
            *count_bucket += 1;
            let [nx, ny] = bucket.get_unchecked_mut(count);
            *nx = cx as u8;
            *ny = cy as u8;
            for i in 0..count {
                let [sx, sy] = *bucket.get_unchecked(i);
                let sx = sx as usize;
                let sy = sy as usize;
                let dx = cx as isize - sx as isize;
                let dy = cy - sy;
                let sbit = 1 << sx;
                let cbit = 1 << cx;
                if dx > 0 {
                    let dx = dx as usize;
                    if P2 {
                        let mut bit = cbit << dx;
                        let mut idx = cy + dy;
                        while bit < 1 << 50 && idx < 50 {
                            *antinodes.get_unchecked_mut(50 + idx) |= bit;
                            bit <<= dx;
                            idx += dy;
                        }
                        let mut bit = sbit >> dx;
                        let mut idx = sy as isize - dy as isize;
                        while bit > 0 && idx >= 0 {
                            *antinodes.get_unchecked_mut(50 + idx as usize) |= bit;
                            bit >>= dx;
                            idx -= dy as isize;
                        }
                    } else {
                        *antinodes.get_unchecked_mut(50 + cy + dy) |= cbit << dx;
                        *antinodes.get_unchecked_mut(50 + sy - dy) |= sbit >> dx;
                    }
                } else {
                    let dx = -dx as usize;
                    if P2 {
                        let mut bit = cbit >> dx;
                        let mut idx = cy + dy;
                        while bit > 0 && idx < 50 {
                            *antinodes.get_unchecked_mut(50 + idx) |= bit;
                            bit >>= dx;
                            idx += dy;
                        }
                        let mut bit = sbit << dx;
                        let mut idx = sy as isize - dy as isize;
                        while bit < 1 << 50 && idx >= 0 {
                            *antinodes.get_unchecked_mut(50 + idx as usize) |= bit;
                            bit <<= dx;
                            idx -= dy as isize;
                        }
                    } else {
                        *antinodes.get_unchecked_mut(50 + cy + dy) |= cbit >> dx;
                        *antinodes.get_unchecked_mut(50 + sy - dy) |= sbit << dx;
                    }
                }
            }

            mask &= mask - 1;
        }

        ptr = ptr.add(51);
        cy += 1;
        if ptr == r.end {
            break;
        }
    }

    antinodes
        .get_unchecked(50..100)
        .iter()
        .map(|&row| if P2 { row } else { row & 0x3FFFFFFFFFFFF }.count_ones())
        .sum()
}

unsafe fn inner1(s: &[u8]) -> u32 {
    process::<false>(s)
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

unsafe fn inner2(s: &[u8]) -> u32 {
    process::<true>(s)
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
        let s = read_to_string("./inputs/8.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/8p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/8p2.txt").unwrap(),
        );
    }
}
