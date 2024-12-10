use super::*;

unsafe fn inner1(s: &[u8]) -> u32 {
    0
}

unsafe fn inner2(s: &[u8]) -> u32 {
    let len = s.len();
    assert_unchecked(len < 65536);
    let size: u32 = (len as f32).sqrt().to_int_unchecked();
    let i = len - 33;
    let mut total = 0;

    asm!(
    "50:",
        "vpcmpeqb {y}, {y48}, ymmword ptr[{s} + {i}]",
        "vpmovmskb {mask:e}, {y}",
        "tzcnt {j:e}, {mask:e}",
        "jb 52f",
    "51:",
        "add {j:e}, {i:e}",
        "call 31f",
        "blsr {mask:e}, {mask:e}",
        "tzcnt {j:e}, {mask:e}",
        "jae 51b",
    "52:",
        "add {i:e}, -32",
        "jns 50b",
        "lea {j:e}, [{i} + 31]",
    "20:",
        "cmp byte ptr[{s} + {j}], 48",
        "jne 21f",
        "call 31f",
    "21:",
        "dec {j:e}",
        "jns 20b",
        "jmp 40f",
    "32:",
        "inc {total:e}",
        "ret",
    "31:",
        "cmp {value:l}, 57",
        "je 32b",
        "inc {value:e}",
        "sub {j:e}, {sizep1:e}",
        "js 33f",
        "cmp byte ptr[{s} + {j}], {value:l}",
        "jne 33f",
        "call 31b",
    "33:",
        "add {j:e}, {size:e}",
        "js 34f",
        "cmp byte ptr[{s} + {j}], {value:l}",
        "jne 34f",
        "call 31b",
    "34:",
        "add {j:e}, 2",
        "cmp byte ptr[{s} + {j}], {value:l}",
        "jne 35f",
        "call 31b",
    "35:",
        "add {j:e}, {size:e}",
        "js 36f",
        "cmp byte ptr[{s} + {j}], {value:l}",
        "jne 36f",
        "call 31b",
    "36:",
        "sub {j:e}, {sizep1:e}",
        "dec {value:e}",
        "ret",
    "40:",
        s = in(reg) s.as_ptr(),
        i = inout(reg) i => _,
        j = out(reg) _,
        value = inout(reg) 48 => _,
        size = in(reg) size,
        sizep1 = in(reg) size + 1,
        total = inout(reg) total,
        mask = out(reg) _,
        y = out(ymm_reg) _,
        y48 = in(ymm_reg) u8x32::splat(b'0'),
    );

    total
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
        let s = read_to_string("./inputs/10.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/10p1.txt").unwrap(),
        )
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/10.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/10p2.txt").unwrap(),
        );
    }
}
