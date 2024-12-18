use std::arch::x86_64::_pdep_u32;

use super::*;

#[inline]
unsafe fn inner1(s: &[u8]) -> u32 {
    let mut ptr = s.as_ptr().cast::<i8x16>();
    let lut = &[i8x16::splat(0); 32];

    let map: &mut [u8; 73 * 72 / 2] = &mut array::from_fn(|i| {
        if (72 / 8..72 * 72 / 8).contains(&i) {
            if i % 9 == 8 {
                128
            } else {
                0
            }
        } else {
            !0
        }
    });
    let map = map.as_mut_ptr();

    // macro_rules! bts {
    //     ($idx:expr) => {
    //         asm!(
    //             "bts dword ptr[{map} + {offset}], {idx:e}",
    //             map = in(reg) map,
    //             idx = in(reg) $idx,
    //             offset = const 72 / 8,
    //             options(nostack),
    //         );
    //     };
    // }

    // for _ in 0..512 {
    //     let chunk = ptr.read_unaligned();
    //     let chunk = chunk - Simd::splat(b'0' as _);
    //     let mask = chunk.simd_lt(Simd::splat(0)).to_bitmask() as u32;
    //     let step = _pdep_u32(8, mask).trailing_zeros();
    //     let shuffle = lut.as_ptr().byte_add(((mask & 0xFC) * 4) as usize).read();
    //     let chunk = _mm_shuffle_epi8(chunk.into(), shuffle.into());
    //     let chunk = _mm_maddubs_epi16(chunk, u16x8::splat(u16::from_ne_bytes([10, 1])).into());
    //     let chunk: u32x4 = _mm_madd_epi16(chunk, u16x8::from_array([72, 1, 72, 1, 72, 1, 72, 1]).into()).into();
    //     let p1 = chunk[0];
    //     let p2 = chunk[1];
    //     bts!(p1);
    //     bts!(p2);
    //     ptr = ptr.byte_add(step as usize);
    // }

    static mut FRONT: [u16; 256] = [0; 256];

    let res: u32;

    asm!(
    "30:",
        "lea {next:e}, [{pos} + 1]",
        "bts dword ptr[{map}], {next:e}",
        "jc 20f",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "inc {j:l}",
    "20:",
        "lea {next:e}, [{pos} + 72]",
        "bts dword ptr[{map}], {next:e}",
        "jc 20f",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "inc {j:l}",
    "20:",
        "lea {next:e}, [{pos} - 1]",
        "bts dword ptr[{map}], {next:e}",
        "jc 20f",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "inc {j:l}",
    "20:",
        "lea {next:e}, [{pos} - 72]",
        "bts dword ptr[{map}], {next:e}",
        "jc 20f",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "inc {j:l}",
    "20:",
        "cmp {i:l}, {k:l}",
        "jne 20f",
        "mov {k:e}, {j:e}",
        "inc {dist:e}",
    "20:",
        "movzx {pos:e}, word ptr[{front} + {i} * 2]",
        "inc {i:l}",
        "cmp {pos:x}, {end}",
        "jne 30b",
        map = in(reg) map,
        pos = in(reg) 72usize,
        next = out(reg) _,
        front = in(reg) &mut FRONT,
        i = inout(reg) 0usize => _,
        j = inout(reg) 0usize => _,
        k = inout(reg) 0usize => _,
        dist = inout(reg) 0 => res,
        end = const 72 * 72 - 2,
        options(nostack),
    );

    res
}

#[inline]
unsafe fn inner2(s: &[u8]) -> &str {
    ""
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
        let s = read_to_string("./inputs/18.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/18p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/18.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/18p2.txt").unwrap(),);
    }
}
