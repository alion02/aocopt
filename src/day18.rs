use std::fmt::Write;

use super::*;

static LUT: [i8x16; 512] = unsafe {
    let mut lut = [[-1i8; 16]; 512];

    let mut idx = 0;
    while idx < 512 {
        let shuffle = &mut lut[idx];

        let mut mask = idx << 2;
        if idx & 1 == 0 {
            mask |= 2;
        }
        mask |= 0x800;

        let mut slot = 0;
        let mut byte = 0;
        while slot < 8 {
            let zeros = mask.trailing_zeros();
            match zeros {
                1 => {
                    shuffle[slot + 1] = byte;
                    byte += 2;
                }
                2 => {
                    shuffle[slot] = byte;
                    shuffle[slot + 1] = byte + 1;
                    byte += 3;
                }
                _ => break,
            }
            mask >>= zeros + 1;
            slot += 2;
        }

        idx += 1;
    }

    transmute(lut)
};

#[inline]
unsafe fn inner1(s: &[u8]) -> u32 {
    static LUT: [i8x16; 512] = unsafe {
        let mut lut = [[-1i8; 16]; 512];

        let mut idx = 0;
        while idx < 512 {
            let shuffle = &mut lut[idx];

            let mut mask = idx << 2;
            if idx & 1 == 0 {
                mask |= 2;
            }
            mask |= 0x800;

            let mut slot = 0;
            let mut byte = 0;
            while slot < 16 {
                let zeros = mask.trailing_zeros();
                match zeros {
                    1 => {
                        shuffle[slot + 2] = byte;
                        byte += 2;
                    }
                    2 => {
                        shuffle[slot] = byte;
                        shuffle[slot + 2] = byte + 1;
                        byte += 3;
                    }
                    _ => break,
                }
                mask >>= zeros + 1;
                slot += 4;
            }

            idx += 1;
        }

        transmute(lut)
    };

    let mut ptr = s.as_ptr().cast::<i8x16>();
    let lut = &LUT;

    static mut MAP: [i8; 73 * 72 / 8] = [-1; 73 * 72 / 8];

    let map = MAP.as_mut_ptr();
    for i in 0..23 {
        map.add(72 / 8 + i * 72 / 8 * 3)
            .cast::<i8x32>()
            .write_unaligned(i8x32::from_array([
                0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, -128, -1, -1, -1,
                -1, -1,
            ]));
    }
    map.add(69 * 72 / 8).cast::<i8x32>().write_unaligned(i8x32::from_array([
        0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, -128, -1, -1, -1, -1, -1,
    ]));

    macro_rules! bts {
        ($idx:expr) => {
            asm!(
                "bts dword ptr[{map} + {offset}], {idx:e}",
                map = in(reg) map,
                idx = in(reg) $idx,
                offset = const 72 / 8,
                options(nostack),
            );
        };
    }

    for _ in 0..256 {
        macro_rules! half_chunk {
            () => {{
                let chunk = ptr.read_unaligned();
                let chunk = chunk - Simd::splat(b'0' as _);
                let mask = chunk.simd_lt(Simd::splat(0)).to_bitmask() as u32;
                let step = _pdep_u32(8, mask).trailing_zeros() + 1;
                let shuffle = lut.as_ptr().byte_add(((mask & 0x7FC) * 4) as usize).read();
                let chunk = _mm_shuffle_epi8(chunk.into(), shuffle.into());
                ptr = ptr.byte_add(step as usize);
                chunk
            }};
        }
        let a = half_chunk!();
        let b = half_chunk!();
        let chunk = _mm_packus_epi16(a, b);
        let chunk = _mm_maddubs_epi16(chunk, u16x8::splat(u16::from_ne_bytes([10, 1])).into());
        let chunk: u32x4 = _mm_madd_epi16(chunk, u16x8::from_array([72, 1, 72, 1, 72, 1, 72, 1]).into()).into();
        bts!(chunk[0]);
        bts!(chunk[1]);
        bts!(chunk[2]);
        bts!(chunk[3]);
    }

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
        "bts dword ptr[{map} + 9], {pos:e}",
        "jc 20f",
        "lea {next:e}, [{pos} + 72]",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "inc {j:l}",
    "20:",
        "lea {next:e}, [{pos} - 1]",
        "bts dword ptr[{map}], {next:e}",
        "jc 20f",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "inc {j:l}",
    "20:",
        "bts dword ptr[{map} - 9], {pos:e}",
        "jc 20f",
        "lea {next:e}, [{pos} - 72]",
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
    let mut ptr = s.as_ptr().cast::<i8x16>();
    let lut = &LUT;

    static mut MAP: [i8; 73 * 72 / 8] = [-1; 73 * 72 / 8];
    static mut FRONT: [i8; 73 * 72 / 8] = [-1; 73 * 72 / 8];
    static GOAL: [i8; 73 * 72 / 8] = {
        let mut goal = [0; 73 * 72 / 8];
        let mut i = 0;
        while i < goal.len() {
            if i % 9 == 8 {
                goal[i] = -64;
            }
            if i / 9 == 1 {
                goal[i] = -1;
            }
            i += 1;
        }
        goal
    };

    let map = MAP.as_mut_ptr();
    for i in 0..23 {
        map.add(72 / 8 + i * 72 / 8 * 3)
            .cast::<i8x32>()
            .write_unaligned(i8x32::from_array([
                0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, -128, -1, -1, -1,
                -1, -1,
            ]));
    }
    map.add(69 * 72 / 8).cast::<i8x32>().write_unaligned(i8x32::from_array([
        0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, -128, -1, -1, -1, -1, -1,
    ]));

    let front = FRONT.as_mut_ptr();
    for i in 0..23 {
        front
            .add(72 / 8 + i * 72 / 8 * 3)
            .cast::<i8x32>()
            .write_unaligned(i8x32::from_array([
                1, 0, 0, 0, 0, 0, 0, 0, -128, 1, 0, 0, 0, 0, 0, 0, 0, -128, 1, 0, 0, 0, 0, 0, 0, 0, -128, -1, -1, -1,
                -1, -1,
            ]));
    }
    front
        .add(69 * 72 / 8)
        .cast::<i8x32>()
        .write_unaligned(i8x32::from_array([
            1, 0, 0, 0, 0, 0, 0, 0, -128, 1, 0, 0, 0, 0, 0, 0, 0, -128, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1,
        ]));

    let len: usize;
    asm!(
        "mov {saved_rsp}, rsp",
        "jmp 201f",
    "200:",
        "lea {ptr}, [{ptr} + {len} + 1]",
    "201:",
        "vpaddb {chunk}, {negascii0}, [{ptr}]",
        "vpmovmskb {mask:e}, {chunk}",
        "pdep {len:e}, {const2:e}, {mask:e}",
        "tzcnt {len:e}, {len:e}",
        "and {mask:e}, 0x7fc",
        "vpshufb {chunk}, {chunk}, [{lut} + {mask} * 4]",
        "vpmaddubsw {chunk}, {chunk}, {mul10}",
        "vpmaddwd {chunk}, {chunk}, {mul_coords}",
        "vmovd {pos:e}, {chunk}",
        "add {pos:e}, 72",
        "bts dword ptr[{map}], {pos:e}",
        "bt dword ptr[{front}], {pos:e}",
        "jnc 200b",
        "jmp 31f",
    "30:",
        "pop {pos}",
    "31:",
        "bt dword ptr[{goal}], {pos:e}",
        "jc 40f",
        "lea {next:e}, [{pos} + 71]",
        "bts dword ptr[{front}], {next:e}",
        "jc 50f",
        "bt dword ptr[{map}], {next:e}",
        "jnc 50f",
        "push {next}",
    "50:",
        "lea {next:e}, [{pos} + 72]",
        "bts dword ptr[{front}], {next:e}",
        "jc 50f",
        "bt dword ptr[{map}], {next:e}",
        "jnc 50f",
        "push {next}",
    "50:",
        "lea {next:e}, [{pos} - 1]",
        "bts dword ptr[{front}], {next:e}",
        "jc 50f",
        "bt dword ptr[{map}], {next:e}",
        "jnc 50f",
        "push {next}",
    "50:",
        "lea {next:e}, [{pos} + 73]",
        "bts dword ptr[{front}], {next:e}",
        "jc 50f",
        "bt dword ptr[{map}], {next:e}",
        "jnc 50f",
        "push {next}",
    "50:",
        "lea {next:e}, [{pos} - 73]",
        "bts dword ptr[{front}], {next:e}",
        "jc 50f",
        "bt dword ptr[{map}], {next:e}",
        "jnc 50f",
        "push {next}",
    "50:",
        "lea {next:e}, [{pos} + 1]",
        "bts dword ptr[{front}], {next:e}",
        "jc 50f",
        "bt dword ptr[{map}], {next:e}",
        "jnc 50f",
        "push {next}",
    "50:",
        "lea {next:e}, [{pos} - 72]",
        "bts dword ptr[{front}], {next:e}",
        "jc 50f",
        "bt dword ptr[{map}], {next:e}",
        "jnc 50f",
        "push {next}",
    "50:",
        "lea {next:e}, [{pos} - 71]",
        "bts dword ptr[{front}], {next:e}",
        "jc 50f",
        "bt dword ptr[{map}], {next:e}",
        "jnc 50f",
        "push {next}",
    "50:",
        "cmp {saved_rsp}, rsp",
        "jne 30b",
        "jmp 200b",
    "40:",
        "mov rsp, {saved_rsp}",
        map = in(reg) map,
        front = in(reg) front,
        goal = in(reg) &GOAL,
        pos = out(reg) _,
        chunk = out(xmm_reg) _,
        negascii0 = in(xmm_reg) u8x16::splat(b'0'.wrapping_neg()),
        mul10 = in(xmm_reg) u16x8::splat(u16::from_ne_bytes([10, 1])),
        mul_coords = in(xmm_reg) u16x8::from_array([72, 1, 72, 1, 72, 1, 72, 1]),
        mask = out(reg) _,
        len = out(reg) len,
        const2 = in(reg) 2,
        lut = in(reg) lut,
        ptr = inout(reg) ptr,
        next = out(reg) _,
        saved_rsp = out(reg) _,
    );

    std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr.cast(), len))
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
