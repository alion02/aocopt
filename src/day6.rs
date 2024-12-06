use std::hint::black_box;

use super::*;

static mut SCRATCH: [u8x32; 67] = [u8x32::from_array([0; 32]); 67];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut loc = r.start;
    asm!(
        "jmp 21f",
    "20:",
        "add {loc}, 64",
    "21:",
        "vmovdqu {c1}, ymmword ptr[{loc}]",
        "vptest {c1}, {mask}",
        "jnz 22f",
        "vmovdqu {c1}, ymmword ptr[{loc} + 32]",
        "vptest {c1}, {mask}",
        "jz 20b",
        "add {loc}, 32",
    "22:",
        "vpsllw {c1}, {c1}, 1",
        "vpmovmskb {r1:e}, {c1}",
        "tzcnt {r1:e}, {r1:e}",
        "add {loc}, {r1}",
        loc = inout(reg) loc,
        mask = in(ymm_reg) u8x32::splat(0x40),
        c1 = out(ymm_reg) _,
        r1 = out(reg) _,
    );
    let visited = &mut SCRATCH;
    visited.fill(Simd::splat(0));
    let mut loc = loc.offset_from(r.start) as usize;
    let mut total = 0;
    macro_rules! visit {
        () => {
            let bit = 1u32.wrapping_shl(loc as u32);
            let cell = (visited as *mut _ as *mut u32).add(loc / 32);
            let value = cell.read();
            total += (value & bit == 0) as u32;
            cell.write(value | bit);
        };
    }
    'outer: loop {
        loop {
            visit!();
            let next = loc.wrapping_sub(131);
            if next >= s.len() {
                break 'outer;
            }
            if *s.get_unchecked(next) == b'#' {
                loc = loc.wrapping_add(1);
                break;
            }
            loc = next;
        }
        loop {
            visit!();
            let next = loc.wrapping_add(1);
            if *s.get_unchecked(next) == b'\n' {
                break 'outer;
            }
            if *s.get_unchecked(next) == b'#' {
                loc = loc.wrapping_add(131);
                break;
            }
            loc = next;
        }
        loop {
            visit!();
            let next = loc.wrapping_add(131);
            if next >= s.len() {
                break 'outer;
            }
            if *s.get_unchecked(next) == b'#' {
                loc = loc.wrapping_sub(1);
                break;
            }
            loc = next;
        }
        loop {
            visit!();
            let next = loc.wrapping_sub(1);
            if *s.get_unchecked(next) == b'\n' {
                break 'outer;
            }
            if *s.get_unchecked(next) == b'#' {
                loc = loc.wrapping_sub(131);
                break;
            }
            loc = next;
        }
    }
    total
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &[u8]) -> u32 {
    let mut up_lo: [u64; 130] = std::array::from_fn(|i| {
        (0..64).fold(0, |acc, j| {
            acc | ((*s.get_unchecked((i) + (127 - j) * 131) == b'#') as u64) << j
        })
    });
    let mut up_hi: [u64; 130] = std::array::from_fn(|i| {
        (0..64).fold(0, |acc, j| {
            acc | ((*s.get_unchecked((i) + (63 - j) * 131) == b'#') as u64) << j
        })
    });
    let mut right_lo: [u64; 130] = std::array::from_fn(|i| {
        (0..64).fold(0, |acc, j| {
            acc | ((*s.get_unchecked((2 + j) + (i) * 131) == b'#') as u64) << j
        })
    });
    let mut right_hi: [u64; 130] = std::array::from_fn(|i| {
        (0..64).fold(0, |acc, j| {
            acc | ((*s.get_unchecked((66 + j) + (i) * 131) == b'#') as u64) << j
        })
    });
    let mut down_lo: [u64; 130] = std::array::from_fn(|i| {
        (0..64).fold(0, |acc, j| {
            acc | ((*s.get_unchecked((i) + (2 + j) * 131) == b'#') as u64) << j
        })
    });
    let mut down_hi: [u64; 130] = std::array::from_fn(|i| {
        (0..64).fold(0, |acc, j| {
            acc | ((*s.get_unchecked((i) + (66 + j) * 131) == b'#') as u64) << j
        })
    });
    let mut left_lo: [u64; 130] = std::array::from_fn(|i| {
        (0..64).fold(0, |acc, j| {
            acc | ((*s.get_unchecked((127 - j) + (i) * 131) == b'#') as u64) << j
        })
    });
    let mut left_hi: [u64; 130] = std::array::from_fn(|i| {
        (0..64).fold(0, |acc, j| {
            acc | ((*s.get_unchecked((63 - j) + (i) * 131) == b'#') as u64) << j
        })
    });
    black_box((
        &mut up_lo,
        &mut up_hi,
        &mut right_lo,
        &mut right_hi,
        &mut down_lo,
        &mut down_hi,
        &mut left_lo,
        &mut left_hi,
    ));
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
    fn test() {
        let s = read_to_string("./inputs/6.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/6p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/6p2.txt").unwrap(),
        );
    }
}
