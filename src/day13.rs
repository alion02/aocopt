use super::*;

unsafe fn inner1(s: &[u8]) -> u64 {
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
    let i = s.len() as isize;
    let lut = LUT.as_ptr();
    let mults10 = u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]);
    let mults100 = u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]);
    let mults10000 = u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]);
    let swar_mask = 0xF0_F0_FF_FF_FF_FF_F0_F0u64;
    let swar_bextr = 8 | 8 << 8;
    let sum;

    asm!(
    "20:",
        "vpaddb {chunk}, {neg_ascii_zero}, xmmword ptr[{start} + {i} - 17]",
        "vpminub {xtmp}, {chunk}, {_9}",
        "vpcmpeqb {xtmp}, {xtmp}, {chunk}",
        "vpmovmskb {mask}, {xtmp}",
        "tzcnt {r1}, {mask}",
        "lea {i}, [{i} + {r1} - 69]",
        "andn {r2}, {swar_mask}, qword ptr[{start} + {i} + 13]",
        "imul {r2}, {r2}, 2561",
        "bextr {ax}, {r2}, {swar_bextr:r}",
        "shr {r2}, 56",
        "andn rax, {swar_mask}, qword ptr[{start} + {i} + 34]",
        "imul rax, rax, 2561",
        "bextr {bx}, rax, {swar_bextr:r}",
        "shr rax, 56",
        "imul {r2}, {bx}",
        "mov {r1}, rax",
        "imul {r1}, {ax}",
        "sub {r1}, {r2}",
        "jz 21f",
        "and {mask}, 0x1FC",
        "vpshufb {chunk}, {chunk}, xmmword ptr[{lut} + {mask} * 4]",
        "vpmaddubsw {chunk}, {chunk}, {mults10}",
        "vpmaddwd {chunk}, {chunk}, {mults100}",
        "vpackusdw {chunk}, {chunk}, {chunk}",
        "vpmaddwd {chunk}, {chunk}, {mults10000}",
        "vmovd {px:e}, {chunk}",
        "vpextrd edx, {chunk}, 1",
        "imul rax, {px}",
        "imul rdx, {bx}",
        "sub rax, rdx",
        "imul {ax}, rax",
        "cqo",
        "idiv {r1}",
        "test rdx, rdx",
        "jnz 21f",
        "imul {bx}, {r1}",
        "imul {r1}, {px}",
        "add {sum_a}, rax",
        "mov rax, {r1}",
        "sub rax, {ax}",
        "cqo",
        "idiv {bx}",
        "add {sum_b}, rax",
    "21:",
        "test {i}, {i}",
        "jns 20b",
        "lea rax, [{sum_a} + {sum_a} * 2]",
        "add rax, {sum_b}",
        chunk = out(xmm_reg) _,
        neg_ascii_zero = in(xmm_reg) u8x16::splat(b'0'.wrapping_neg()),
        xtmp = out(xmm_reg) _,
        _9 = in(xmm_reg) u8x16::splat(9),
        start = in(reg) start,
        i = inout(reg) i => _,
        mask = out(reg) _,
        r1 = out(reg) _,
        ax = out(reg) _,
        r2 = out(reg) _, // ay
        bx = out(reg) _,
        out("rax") sum, // by
        px = out(reg) _,
        out("rdx") _, // py, rem
        sum_a = inout(reg) 0u64 => _,
        sum_b = inout(reg) 0u64 => _,
        swar_mask = in(reg) swar_mask,
        swar_bextr = in(reg) swar_bextr,
        lut = in(reg) lut,
        mults10 = in(xmm_reg) mults10,
        mults100 = in(xmm_reg) mults100,
        mults10000 = in(xmm_reg) mults10000,
        options(nostack),
    );

    sum
}

unsafe fn inner2(s: &[u8]) -> u64 {
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
    let i = s.len() as isize;
    let lut = LUT.as_ptr();
    let mults10 = u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]);
    let mults100 = u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]);
    let mults10000 = u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]);
    let offset = u64x2::splat(10000000000000);
    let shuffle = i8x16::from_array([0, 1, 4, 5, -1, -1, -1, -1, 8, 9, 12, 13, -1, -1, -1, -1]);
    let swar_mask = 0xF0_F0_FF_FF_FF_FF_F0_F0u64;
    let swar_bextr = 8 | 8 << 8;
    let sum;

    asm!(
    "20:",
        "vpaddb {chunk}, {neg_ascii_zero}, xmmword ptr[{start} + {i} - 17]",
        "vpminub {xtmp}, {chunk}, {_9}",
        "vpcmpeqb {xtmp}, {xtmp}, {chunk}",
        "vpmovmskb {mask}, {xtmp}",
        "tzcnt {r1}, {mask}",
        "lea {i}, [{i} + {r1} - 69]",
        "andn {r2}, {swar_mask}, qword ptr[{start} + {i} + 13]",
        "imul {r2}, {r2}, 2561",
        "bextr {ax}, {r2}, {swar_bextr:r}",
        "shr {r2}, 56",
        "andn rax, {swar_mask}, qword ptr[{start} + {i} + 34]",
        "imul rax, rax, 2561",
        "bextr {bx}, rax, {swar_bextr:r}",
        "shr rax, 56",
        "imul {r2}, {bx}",
        "mov {r1}, rax",
        "imul {r1}, {ax}",
        "sub {r1}, {r2}",
        "jz 21f",
        "and {mask}, 0x1FC",
        "vpshufb {chunk}, {chunk}, xmmword ptr[{lut} + {mask} * 4]",
        "vpmaddubsw {chunk}, {chunk}, {mults10}",
        "vpmaddwd {chunk}, {chunk}, {mults100}",
        "vpshufb {chunk}, {chunk}, {shuffle}",
        "vpmaddwd {chunk}, {chunk}, {mults10000}",
        "vpaddq {chunk}, {chunk}, {offset}",
        "vmovq {px}, {chunk}",
        "vpextrq rdx, {chunk}, 1",
        "imul rax, {px}",
        "imul rdx, {bx}",
        "sub rax, rdx",
        "imul {ax}, rax",
        "cqo",
        "idiv {r1}",
        "test rdx, rdx",
        "jnz 21f",
        "imul {bx}, {r1}",
        "imul {r1}, {px}",
        "add {sum_a}, rax",
        "mov rax, {r1}",
        "sub rax, {ax}",
        "cqo",
        "idiv {bx}",
        "add {sum_b}, rax",
    "21:",
        "test {i}, {i}",
        "jns 20b",
        "lea rax, [{sum_a} + {sum_a} * 2]",
        "add rax, {sum_b}",
        chunk = out(xmm_reg) _,
        neg_ascii_zero = in(xmm_reg) u8x16::splat(b'0'.wrapping_neg()),
        xtmp = out(xmm_reg) _,
        _9 = in(xmm_reg) u8x16::splat(9),
        start = in(reg) start,
        i = inout(reg) i => _,
        mask = out(reg) _,
        r1 = out(reg) _,
        ax = out(reg) _,
        r2 = out(reg) _, // ay
        bx = out(reg) _,
        out("rax") sum, // by
        px = out(reg) _,
        out("rdx") _, // py, rem
        sum_a = inout(reg) 0u64 => _,
        sum_b = inout(reg) 0u64 => _,
        swar_mask = in(reg) swar_mask,
        swar_bextr = in(reg) swar_bextr,
        lut = in(reg) lut,
        mults10 = in(xmm_reg) mults10,
        mults100 = in(xmm_reg) mults100,
        mults10000 = in(xmm_reg) mults10000,
        offset = in(xmm_reg) offset,
        shuffle = in(xmm_reg) shuffle,
        options(nostack),
    );

    sum
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
