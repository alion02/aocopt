use std::cmp::Ordering;

use super::*;

unsafe fn inner1(s: &[u8]) -> usize {
    let mut checksum = 0;

    asm!(
    "20:",
        "movzx {len:e}, byte ptr[{s} + {left} * 2]",
        "sub {len:e}, 48",
        "lea {scratch:e}, [{len} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {left}",
        "imul {scratch}, {len}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {len:e}",
        "movzx {rem_dst:e}, byte ptr[{s} + {left} * 2 + 1]",
        "inc {left:e}",
        "sub {rem_dst:e}, 48",
        "jz 20b",
        "cmp {left:e}, {right:e}",
        "je 50f",
    "22:",
        "dec {right:e}",
        "movzx {rem_src:e}, byte ptr[{s} + {right} * 2]",
        "sub {rem_src:e}, 48",
        "cmp {rem_dst}, {rem_src}",
        "ja 40f",
    "21:",
        "lea {scratch:e}, [{rem_dst} + {disk_pos} * 2 - 1]",
        "jb 30f",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_dst}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {rem_dst:e}",
        "cmp {left:e}, {right:e}",
        "jne 20b",
        "jmp 50f",
    "30:",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_dst}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {rem_dst:e}",
        "sub {rem_src:e}, {rem_dst:e}",
    "31:",
        "cmp {left:e}, {right:e}",
        "je 60f",
        "movzx {len:e}, byte ptr[{s} + {left} * 2]",
        "sub {len:e}, 48",
        "lea {scratch:e}, [{len} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {left}",
        "imul {scratch}, {len}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {len:e}",
        "movzx {rem_dst:e}, byte ptr[{s} + {left} * 2 + 1]",
        "inc {left:e}",
        "sub {rem_dst:e}, 48",
        "jz 31b",
        "cmp {rem_dst}, {rem_src}",
        "jbe 21b",
    "40:",
        "lea {scratch:e}, [{rem_src} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_src}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {rem_src:e}",
        "sub {rem_dst:e}, {rem_src:e}",
        "cmp {left:e}, {right:e}",
        "jne 22b",
        "jmp 50f",
    "60:",
        "lea {scratch:e}, [{rem_src} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_src}",
        "add {checksum}, {scratch}",
    "50:",
        "shr {checksum}",
        checksum = inout(reg) checksum,
        s = in(reg) s.as_ptr(),
        left = inout(reg) 0usize => _,
        right = inout(reg) s.len() / 2 => _,
        disk_pos = inout(reg) 0usize => _,
        rem_dst = out(reg) _,
        rem_src = out(reg) _,
        scratch = out(reg) _,
        len = out(reg) _,
        options(nostack, readonly),
    );

    checksum
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

unsafe fn inner2(s: &[u8]) -> u64 {
    0
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
        let s = read_to_string("./inputs/9.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/9p1.txt").unwrap(),
        );
        // assert_eq!(
        //     part2(s).to_string(),
        //     read_to_string("./outputs/9p2.txt").unwrap(),
        // );
    }
}
