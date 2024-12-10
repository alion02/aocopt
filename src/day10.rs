use super::*;

unsafe fn inner1(s: &[u8]) -> u32 {
    0
}

unsafe fn inner2(s: &[u8]) -> u32 {
    let len = s.len();
    assert_unchecked(len < 65536);
    let size: u32 = (len as f32).sqrt().to_int_unchecked();
    let i = len - 2;
    let mut total = 0;

    asm!(
    "20:",
        "cmp byte ptr[{s} + {i}], 48",
        "jne 21f",
        "call 31f",
    "21:",
        "dec {i:e}",
        "jnz 20b",
        "jmp 40f",
    "30:",
        "cmp byte ptr[{s} + {i}], {value:l}",
        "je 31f",
        "ret",
    "32:",
        "inc {total:e}",
        "ret",
    "31:",
        "cmp {value:l}, 57",
        "je 32b",
        "inc {value:e}",
        "sub {i:e}, {sizep1:e}",
        "js 33f",
        "call 30b",
    "33:",
        "add {i:e}, {size:e}",
        "js 34f",
        "call 30b",
    "34:",
        "add {i:e}, 2",
        "call 30b",
        "add {i:e}, {size:e}",
        "js 36f",
        "call 30b",
    "36:",
        "sub {i:e}, {sizep1:e}",
        "dec {value:e}",
        "ret",
    "40:",
        s = in(reg) s.as_ptr(),
        i = inout(reg) i => _,
        value = inout(reg) 48 => _,
        size = in(reg) size,
        sizep1 = in(reg) size + 1,
        total = inout(reg) total,
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
