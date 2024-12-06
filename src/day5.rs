use std::arch::{asm, x86_64::_mm_testc_si128};

use super::*;

// 5-23 numbers in list
// all numbers 2 digit

static mut MATRIX: [u8x32; 50] = [u8x32::from_array([0; 32]); 50];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[allow(unreachable_code)]
unsafe fn inner1(s: &[u8]) -> u32 {
    let matrix = &mut MATRIX;

    asm!(
        "vmovdqa ymmword ptr[{table} + 160], {zero}",
        "vmovdqa ymmword ptr[{table} + 192], {zero}",
        "vmovdqa ymmword ptr[{table} + 224], {zero}",
        "vmovdqa ymmword ptr[{table} + 256], {zero}",
        "vmovdqa ymmword ptr[{table} + 288], {zero}",
        "vmovdqa ymmword ptr[{table} + 320], {zero}",
        "vmovdqa ymmword ptr[{table} + 352], {zero}",
        "vmovdqa ymmword ptr[{table} + 384], {zero}",
        "vmovdqa ymmword ptr[{table} + 416], {zero}",
        "vmovdqa ymmword ptr[{table} + 448], {zero}",
        "vmovdqa ymmword ptr[{table} + 480], {zero}",
        "vmovdqa ymmword ptr[{table} + 512], {zero}",
        "vmovdqa ymmword ptr[{table} + 544], {zero}",
        "vmovdqa ymmword ptr[{table} + 576], {zero}",
        "vmovdqa ymmword ptr[{table} + 608], {zero}",
        "vmovdqa ymmword ptr[{table} + 640], {zero}",
        "vmovdqa ymmword ptr[{table} + 672], {zero}",
        "vmovdqa ymmword ptr[{table} + 704], {zero}",
        "vmovdqa ymmword ptr[{table} + 736], {zero}",
        "vmovdqa ymmword ptr[{table} + 768], {zero}",
        "vmovdqa ymmword ptr[{table} + 800], {zero}",
        "vmovdqa ymmword ptr[{table} + 832], {zero}",
        "vmovdqa ymmword ptr[{table} + 864], {zero}",
        "vmovdqa ymmword ptr[{table} + 896], {zero}",
        "vmovdqa ymmword ptr[{table} + 928], {zero}",
        "vmovdqa ymmword ptr[{table} + 960], {zero}",
        "vmovdqa ymmword ptr[{table} + 992], {zero}",
        "vmovdqa ymmword ptr[{table} + 1024], {zero}",
        "vmovdqa ymmword ptr[{table} + 1056], {zero}",
        "vmovdqa ymmword ptr[{table} + 1088], {zero}",
        "vmovdqa ymmword ptr[{table} + 1120], {zero}",
        "vmovdqa ymmword ptr[{table} + 1152], {zero}",
        "vmovdqa ymmword ptr[{table} + 1184], {zero}",
        "vmovdqa ymmword ptr[{table} + 1216], {zero}",
        "vmovdqa ymmword ptr[{table} + 1248], {zero}",
        "vmovdqa ymmword ptr[{table} + 1280], {zero}",
        "vmovdqa ymmword ptr[{table} + 1312], {zero}",
        "vmovdqa ymmword ptr[{table} + 1344], {zero}",
        "vmovdqa ymmword ptr[{table} + 1376], {zero}",
        "vmovdqa ymmword ptr[{table} + 1408], {zero}",
        "vmovdqa ymmword ptr[{table} + 1440], {zero}",
        "vmovdqa ymmword ptr[{table} + 1472], {zero}",
        "vmovdqa ymmword ptr[{table} + 1504], {zero}",
        "vmovdqa ymmword ptr[{table} + 1536], {zero}",
        "vmovdqa ymmword ptr[{table} + 1568], {zero}",
        table = in(reg) matrix,
        zero = in(ymm_reg) u8x32::splat(0),
    );

    let mut sum = 0;
    let mut i = 0;
    let table: &mut [u8; 1600] = transmute(matrix);

    macro_rules! parse {
        ($off:expr$(, $on_newline:expr)?) => {{
            let v1 = *s.get_unchecked(i + $off) as u32;
            $(if v1 == b'\n' as u32 {
                $on_newline
            })?
            let v2 = *s.get_unchecked(i + $off + 1) as u32;
            v1 * 10 + v2 - 528
        }};
    }

    loop {
        let x = parse!(0, break);
        let y = parse!(3);
        *table.get_unchecked_mut((x * 16 + y / 8) as usize) |= 1u8.wrapping_shl(y);
        i += 6;
    }

    i += 1;

    'outer: loop {
        let [a, b, c, d, e] = [parse!(0), parse!(3), parse!(6), parse!(9), parse!(12)];
        let mut j = i + 6;
        i += 15;
        let mut wrong_order =
            *table.get_unchecked((b * 16 + a / 8) as usize) & 1u8.wrapping_shl(a) != 0;
        wrong_order |= *table.get_unchecked((c * 16 + b / 8) as usize) & 1u8.wrapping_shl(b) != 0;
        wrong_order |= *table.get_unchecked((d * 16 + c / 8) as usize) & 1u8.wrapping_shl(c) != 0;
        wrong_order |= *table.get_unchecked((e * 16 + d / 8) as usize) & 1u8.wrapping_shl(d) != 0;

        let mut p = e;
        loop {
            if *s.get_unchecked(i - 1) == b'\n' {
                if !wrong_order {
                    sum += {
                        let v1 = *s.get_unchecked(j) as u32;
                        let v2 = *s.get_unchecked(j + 1) as u32;
                        v1 * 10 + v2 - 528
                    };
                }

                if i == s.len() {
                    break 'outer;
                }

                break;
            }
            let n1 = parse!(0);
            let n2 = parse!(3);
            i += 6;
            j += 3;
            wrong_order |=
                *table.get_unchecked((n1 * 16 + p / 8) as usize) & 1u8.wrapping_shl(p) != 0;
            wrong_order |=
                *table.get_unchecked((n2 * 16 + n1 / 8) as usize) & 1u8.wrapping_shl(n1) != 0;
            p = n2;
        }
    }

    sum
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &[u8]) -> u32 {
    let matrix = &mut MATRIX;

    asm!(
        "vmovdqa ymmword ptr[{table} + 160], {zero}",
        "vmovdqa ymmword ptr[{table} + 192], {zero}",
        "vmovdqa ymmword ptr[{table} + 224], {zero}",
        "vmovdqa ymmword ptr[{table} + 256], {zero}",
        "vmovdqa ymmword ptr[{table} + 288], {zero}",
        "vmovdqa ymmword ptr[{table} + 320], {zero}",
        "vmovdqa ymmword ptr[{table} + 352], {zero}",
        "vmovdqa ymmword ptr[{table} + 384], {zero}",
        "vmovdqa ymmword ptr[{table} + 416], {zero}",
        "vmovdqa ymmword ptr[{table} + 448], {zero}",
        "vmovdqa ymmword ptr[{table} + 480], {zero}",
        "vmovdqa ymmword ptr[{table} + 512], {zero}",
        "vmovdqa ymmword ptr[{table} + 544], {zero}",
        "vmovdqa ymmword ptr[{table} + 576], {zero}",
        "vmovdqa ymmword ptr[{table} + 608], {zero}",
        "vmovdqa ymmword ptr[{table} + 640], {zero}",
        "vmovdqa ymmword ptr[{table} + 672], {zero}",
        "vmovdqa ymmword ptr[{table} + 704], {zero}",
        "vmovdqa ymmword ptr[{table} + 736], {zero}",
        "vmovdqa ymmword ptr[{table} + 768], {zero}",
        "vmovdqa ymmword ptr[{table} + 800], {zero}",
        "vmovdqa ymmword ptr[{table} + 832], {zero}",
        "vmovdqa ymmword ptr[{table} + 864], {zero}",
        "vmovdqa ymmword ptr[{table} + 896], {zero}",
        "vmovdqa ymmword ptr[{table} + 928], {zero}",
        "vmovdqa ymmword ptr[{table} + 960], {zero}",
        "vmovdqa ymmword ptr[{table} + 992], {zero}",
        "vmovdqa ymmword ptr[{table} + 1024], {zero}",
        "vmovdqa ymmword ptr[{table} + 1056], {zero}",
        "vmovdqa ymmword ptr[{table} + 1088], {zero}",
        "vmovdqa ymmword ptr[{table} + 1120], {zero}",
        "vmovdqa ymmword ptr[{table} + 1152], {zero}",
        "vmovdqa ymmword ptr[{table} + 1184], {zero}",
        "vmovdqa ymmword ptr[{table} + 1216], {zero}",
        "vmovdqa ymmword ptr[{table} + 1248], {zero}",
        "vmovdqa ymmword ptr[{table} + 1280], {zero}",
        "vmovdqa ymmword ptr[{table} + 1312], {zero}",
        "vmovdqa ymmword ptr[{table} + 1344], {zero}",
        "vmovdqa ymmword ptr[{table} + 1376], {zero}",
        "vmovdqa ymmword ptr[{table} + 1408], {zero}",
        "vmovdqa ymmword ptr[{table} + 1440], {zero}",
        "vmovdqa ymmword ptr[{table} + 1472], {zero}",
        "vmovdqa ymmword ptr[{table} + 1504], {zero}",
        "vmovdqa ymmword ptr[{table} + 1536], {zero}",
        "vmovdqa ymmword ptr[{table} + 1568], {zero}",
        table = in(reg) matrix,
        zero = in(ymm_reg) u8x32::splat(0),
    );

    let mut sum = 0;
    let mut i = 0;
    let table: &mut [u8; 1600] = transmute(matrix);

    macro_rules! parse {
        ($off:expr$(, $on_newline:expr)?) => {{
            let v1 = *s.get_unchecked(i + $off) as u32;
            $(if v1 == b'\n' as u32 {
                $on_newline
            })?
            let v2 = *s.get_unchecked(i + $off + 1) as u32;
            v1 * 10 + v2 - 528
        }};
    }

    loop {
        let x = parse!(0, break);
        let y = parse!(3);
        *table.get_unchecked_mut((y * 16 + x / 8) as usize) |= 1u8.wrapping_shl(x);
        i += 6;
    }

    i += 1;

    'outer: loop {
        let orig_i = i;

        let [a, b, c, d, e] = [parse!(0), parse!(3), parse!(6), parse!(9), parse!(12)];
        let mut j = 2;
        i += 15;
        let mut wrong_order =
            *table.get_unchecked((a * 16 + b / 8) as usize) & 1u8.wrapping_shl(b) != 0;
        wrong_order |= *table.get_unchecked((b * 16 + c / 8) as usize) & 1u8.wrapping_shl(c) != 0;
        wrong_order |= *table.get_unchecked((c * 16 + d / 8) as usize) & 1u8.wrapping_shl(d) != 0;
        wrong_order |= *table.get_unchecked((d * 16 + e / 8) as usize) & 1u8.wrapping_shl(e) != 0;
        let mut set = 1u128 << a;
        set |= 1 << b;
        set |= 1 << c;
        set |= 1 << d;
        set |= 1 << e;

        let mut p = e;
        loop {
            if *s.get_unchecked(i - 1) == b'\n' {
                if wrong_order {
                    sum += {
                        let table: &mut [u128; 100] = transmute(&mut *table);
                        let resume = i;
                        i = orig_i;
                        loop {
                            let v = parse!(0);
                            let predecessors = table.get_unchecked(v as usize) & set;
                            let pre_count = predecessors.count_ones();
                            if pre_count == j as u32 {
                                i = resume;
                                break v;
                            }
                            i += 3;
                        }
                    };
                }

                if i == s.len() {
                    break 'outer;
                }

                break;
            }
            let n1 = parse!(0);
            let n2 = parse!(3);
            i += 6;
            j += 1;
            wrong_order |=
                *table.get_unchecked((p * 16 + n1 / 8) as usize) & 1u8.wrapping_shl(n1) != 0;
            wrong_order |=
                *table.get_unchecked((n1 * 16 + n2 / 8) as usize) & 1u8.wrapping_shl(n2) != 0;
            set |= 1 << n1;
            set |= 1 << n2;
            p = n2;
        }
    }

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
    fn test() {
        let s = read_to_string("./inputs/5.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/5p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/5p2.txt").unwrap(),
        );
    }
}
