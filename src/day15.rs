use super::*;

static DIR_TABLE: [i16; 256] = {
    let mut dir_table = [0; 256];
    dir_table[b'>' as usize] = 1;
    dir_table[b'v' as usize] = 51;
    dir_table[b'<' as usize] = -1;
    dir_table[b'<' as usize + 1] = -1;
    dir_table[b'^' as usize] = -51;
    dir_table[b'^' as usize + 1] = -1;
    dir_table
};

#[allow(unreachable_code)]
unsafe fn inner1(s: &[u8]) -> u32 {
    static mut MAP: [u8; 2560] = [0; 2560];
    let map = &mut MAP;
    map.copy_from_slice(s.get_unchecked(..2560));
    let pos = 24usize * 51 + 24;
    *map.get_unchecked_mut(pos) = b'.';

    asm!(
        "jmp 24f",
    // #
    "21:",
        "sub {pos:e}, {tmp:e}",
    // .
    "20:",
        "inc {ip}",
        "je 99f",
    "24:",
        "movzx {inst:e}, byte ptr[{instrs} + {ip}]",
        "mov {tmp:e}, dword ptr[{dir_table} + {inst} * 2]",
        "add {pos:e}, {tmp:e}",
        "cmp byte ptr[{map} + {pos}], 46",
        "je 20b",
        "jb 21b",
    // O
        "mov {block_pos:e}, {pos:e}",
    "22:",
    // O repeats
        "add {block_pos:e}, {tmp:e}",
        "cmp byte ptr[{map} + {block_pos}], 46",
        "ja 22b",
        "jb 21b",
    // O then .
    "23:",
        "mov byte ptr[{map} + {pos}], 46",
        "mov byte ptr[{map} + {block_pos}], 79",
        "inc {ip}",
        "jne 24b",
    "99:",
        instrs = in(reg) s.as_ptr_range().end,
        ip = inout(reg) -20020isize => _,
        map = in(reg) map,
        pos = inout(reg) pos => _,
        tmp = out(reg) _,
        inst = out(reg) _,
        block_pos = out(reg) _,
        dir_table = inout(reg) &DIR_TABLE => _,
        options(nostack),
    );

    #[inline(never)]
    fn count(map: &mut [u8; 2560]) -> u32 {
        map.iter()
            .zip((0..50).flat_map(|y| (0..51).map(move |x| (x, y))))
            .map(|(c, (x, y))| (*c == b'O') as u32 * (x + y * 100))
            .sum()
    }

    count(map)
}

unsafe fn inner2(s: &[u8]) -> u32 {
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
    fn p1() {
        let s = read_to_string("./inputs/15.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/15p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/15.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/15p2.txt").unwrap(),);
    }
}
