use super::*;

#[inline]
unsafe fn inner1(s: &[u8]) -> &str {
    static mut BUF: u8x64 = Simd::from_array([
        10,
        1,
        10,
        1,
        10,
        1,
        10,
        1,
        100,
        0,
        1,
        0,
        100,
        0,
        1,
        0,
        16,
        39,
        1,
        0,
        16,
        39,
        1,
        0,
        b'0'.wrapping_neg(),
        0,
        0,
        0,
        b'1',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ]);

    let r: *const u8;

    asm!(
        "vpbroadcastb {chunk}, [rip + {buf}+24]",
        "vpaddb {chunk}, {chunk}, [{ptr} + 12]",
        "vpmaddubsw {chunk}, {chunk}, [rip + {buf}]",
        "vpmaddwd {chunk}, {chunk}, [rip + {buf}+8]",
        "vpackusdw {chunk}, {chunk}, {chunk}",
        "vpmaddwd {chunk}, {chunk}, [rip + {buf}+16]",
        "vmovd edx, {chunk}",
        "movzx {imm1:e}, byte ptr[{ptr} + 65]",
        "sub {imm1:e}, {ascii0}",
        "vpbroadcastd {chunk}, [rip + {buf}+25]",
        "vpcmpeqb {chunk}, {chunk}, [{ptr} + 64]",
        "vpmovmskb {imm2:e}, {chunk}",
        "tzcnt {imm2:e}, {imm2:e}",
        "movzx {imm2:e}, byte ptr[{ptr} + {imm2} + 66]",
        "lea {out}, [rip + {buf}+29]",
    "20:",
        "bextr ecx, edx, {mask:e}",
        "xor ecx, {imm1:e}",
        "shrx {c:e}, edx, ecx",
        "xor ecx, {c:e}",
        "xor ecx, {imm2:e}",
        "and ecx, 7",
        "add ecx, 48",
        "mov byte ptr[{out} + {len} - 91], cl",
        "add {len:e}, 2",
        "shr edx, 3",
        "jnz 20b",
        imm1 = out(reg) _,
        imm2 = out(reg) _,
        out("edx") _,
        out("ecx") _,
        c = out(reg) _,
        mask = in(reg) 3 << 8,
        chunk = out(xmm_reg) _,
        buf = sym BUF,
        ptr = inout(reg) s.as_ptr() => _,
        len = inout(reg) s.len() => _,
        ascii0 = const b'0',
        out = out(reg) r,
        options(nostack),
    );

    std::str::from_utf8_unchecked(std::slice::from_raw_parts(r, 17))
}

static LUT: [u64; 1 << 14] = unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day17.bin"))) };

#[inline]
unsafe fn inner2(s: &[u8]) -> u64 {
    let out: u64;
    asm!(
        "movzx {out:e}, byte ptr[{ptr} + 73]",
        "shl {out:e}, 16",
        "xor {out:l}, byte ptr[{ptr} + 65]",
        "xor {out}, qword ptr[{ptr} + 74]",
        "pext {out}, {out}, {mask}",
        "mov {out}, qword ptr[rdx + {out} * 8]",
        out = out(reg) out,
        out("ecx") _,
        inout("rdx") &LUT => _,
        ptr = in(reg) s.as_ptr(),
        options(nostack),
        mask = in(reg) 0x07_00_04_00_07_07_04_07u64,
    );
    out
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
