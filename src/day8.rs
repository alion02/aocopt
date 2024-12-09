#![allow(clippy::pointers_in_nomem_asm_block)]
use std::mem::offset_of;

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

unsafe fn process(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    let mut cy = 0usize;

    #[repr(C, align(32))]
    struct Tables {
        antinodes: [u64; 50],
        _padding2: [u8; 16],
        frequencies: [[[u8; 2]; 4]; 75],
    }

    static mut TABLES: Tables = Tables {
        antinodes: [0; 50],
        _padding2: [0; 16],
        frequencies: [[[0; 2]; 4]; 75],
    };

    let Tables {
        antinodes,
        frequencies,
        ..
    } = &mut TABLES;

    antinodes.fill(0);
    frequencies.fill(Default::default());

    loop {
        let c1 = ptr.cast::<u8x32>().read_unaligned() + Simd::splat(127 - b'.');
        let c2 = ptr.add(18).cast::<u8x32>().read_unaligned() + Simd::splat(127 - b'.');
        let m1 = c1.simd_ge(Simd::splat(128)).to_bitmask();
        let m2 = c2.simd_ge(Simd::splat(128)).to_bitmask();
        let mut mask = m1 | m2 << 18;
        *antinodes.get_unchecked_mut(cy) |= mask;
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
                    let mut bit = cbit << dx;
                    let mut idx = cy + dy;
                    while bit < 1 << 50 && idx < 50 {
                        *antinodes.get_unchecked_mut(idx) |= bit;
                        bit <<= dx;
                        idx += dy;
                    }
                    let mut bit = sbit >> dx;
                    let mut idx = sy as isize - dy as isize;
                    while bit > 0 && idx >= 0 {
                        *antinodes.get_unchecked_mut(idx as usize) |= bit;
                        bit >>= dx;
                        idx -= dy as isize;
                    }
                } else {
                    let dx = -dx as usize;
                    let mut bit = cbit >> dx;
                    let mut idx = cy + dy;
                    while bit > 0 && idx < 50 {
                        *antinodes.get_unchecked_mut(idx) |= bit;
                        bit >>= dx;
                        idx += dy;
                    }
                    let mut bit = sbit << dx;
                    let mut idx = sy as isize - dy as isize;
                    while bit < 1 << 50 && idx >= 0 {
                        *antinodes.get_unchecked_mut(idx as usize) |= bit;
                        bit <<= dx;
                        idx -= dy as isize;
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

    antinodes.iter().map(|&row| row.count_ones()).sum()
}

unsafe fn inner1(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();

    #[repr(C, align(32))]
    struct Tables {
        _padding1: [u8; 16],
        antinodes: [u64; 150],
        frequencies: [[[u8; 2]; 4]; 76],
    }

    static mut TABLES: Tables = Tables {
        _padding1: [0; 16],
        antinodes: [0; 150],
        frequencies: [[[0; 2]; 4]; 76],
    };

    let tables = &mut TABLES;

    tables.antinodes[50..100].fill(0);

    asm!(
        "vmovdqa ymmword ptr[{table}], {ones}",
        "vmovdqa ymmword ptr[{table} + 32], {ones}",
        "vmovdqa ymmword ptr[{table} + 64], {ones}",
        "vmovdqa ymmword ptr[{table} + 96], {ones}",
        "vmovdqa ymmword ptr[{table} + 128], {ones}",
        "vmovdqa ymmword ptr[{table} + 160], {ones}",
        "vmovdqa ymmword ptr[{table} + 192], {ones}",
        "vmovdqa ymmword ptr[{table} + 224], {ones}",
        "vmovdqa ymmword ptr[{table} + 256], {ones}",
        "vmovdqa ymmword ptr[{table} + 288], {ones}",
        "vmovdqa ymmword ptr[{table} + 320], {ones}",
        "vmovdqa ymmword ptr[{table} + 352], {ones}",
        "vmovdqa ymmword ptr[{table} + 384], {ones}",
        "vmovdqa ymmword ptr[{table} + 416], {ones}",
        "vmovdqa ymmword ptr[{table} + 448], {ones}",
        "vmovdqa ymmword ptr[{table} + 480], {ones}",
        "vmovdqa ymmword ptr[{table} + 512], {ones}",
        "vmovdqa ymmword ptr[{table} + 544], {ones}",
        "vmovdqa ymmword ptr[{table} + 576], {ones}",
        ones = in(ymm_reg) i8x32::splat(-1),
        table = in(reg) &raw mut tables.frequencies,
    );

    asm!(
    "21:",
        "vpaddb {y1}, {offset}, ymmword ptr[{ptr}]",
        "vpaddb {y2}, {offset}, ymmword ptr[{ptr} + 18]",
        "vpmovmskb {r1:e}, {y1}",
        "vpmovmskb {r2:e}, {y2}",
        "shl {r2}, 18",
        "or {r1}, {r2}",
        "jz 20f",
    "23:",
        "tzcnt {cx}, {r1}",
        "movzx {r2:e}, byte ptr[{ptr} + {cx}]",
        "lea {r2}, [{table} + {r2} * 8 + 416]",
        "movsx {count:e}, byte ptr[{r2} + 7]",
        "inc {count:e}",
        "mov byte ptr[{r2} + 7], {count:l}",
        "mov byte ptr[{r2} + {count} * 2], {cx:l}",
        "mov byte ptr[{r2} + {count} * 2 + 1], {cy:l}",
        "jz 22f",
        "shlx {cbit}, {one:r}, {cx}",
    "26:",
        "movzx {sx:e}, byte ptr[{r2} + {count} * 2 - 2]",
        "movzx {sy:e}, byte ptr[{r2} + {count} * 2 - 1]",
        "shlx {sbit}, {one:r}, {sx}",
        "mov {dy:e}, {cy:e}",
        "sub {dy}, {sy}",
        "mov {dx:e}, {cx:e}",
        "sub {sy:e}, {dy:e}",
        "sub {dx}, {sx}",
        "lea {sx}, [{cy} + {dy}]",
        "jbe 24f",
        "shlx {dy}, {cbit}, {dx}",
        "shrx {sbit}, {sbit}, {dx}",
        "jmp 25f",
    "24:",
        "neg {dx}",
        "shrx {dy}, {cbit}, {dx}",
        "shlx {sbit}, {sbit}, {dx}",
    "25:",
        "or qword ptr[{table} + {sx} * 8], {dy}",
        "or qword ptr[{table} + {sy} * 8], {sbit}",
        "dec {count:e}",
        "jnz 26b",
    "22:",
        "blsr {r1}, {r1}",
        "jnz 23b",
    "20:",
        "add {ptr}, -51",
        "dec {cy:e}",
        "jns 21b",
        y1 = out(ymm_reg) _,
        y2 = out(ymm_reg) _,
        offset = in(ymm_reg) u8x32::splat(127 - b'.'),
        ptr = inout(reg) r.end.sub(51) => _,
        r1 = out(reg) _,
        r2 = out(reg) _,
        count = out(reg) _,
        cx = out(reg) _,
        cy = inout(reg) 49usize => _,
        sx = out(reg) _,
        sy = out(reg) _,
        dx = out(reg) _,
        dy = out(reg) _,
        cbit = out(reg) _,
        sbit = out(reg) _,
        table = in(reg) (tables as *mut Tables).byte_add(offset_of!(Tables, antinodes) + size_of::<u64>() * 50),
        one = in(reg) 1,
        options(nostack),
    );

    tables
        .antinodes
        .get_unchecked(50..100)
        .iter()
        .map(|&row| (row & 0x3FFFFFFFFFFFF).count_ones())
        .sum()
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

unsafe fn inner2(s: &[u8]) -> u32 {
    process(s)
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
