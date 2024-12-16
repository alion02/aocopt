use std::hint::unreachable_unchecked;

use super::*;

#[allow(unreachable_code)]
unsafe fn inner1(s: &[u8]) -> u32 {
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
    static mut MAP: [u8; 2560] = [0; 2560];
    let map = &mut MAP;
    map.copy_from_slice(s.get_unchecked(..2560));
    let pos = 24usize * 51 + 24;
    *map.get_unchecked_mut(pos) = b'.';

    asm!(
        "jmp 24f",
    // #
    "21:",
        "sub {pos:e}, dword ptr[{dir_table} + {inst} * 2]",
    // .
    "20:",
        "inc {ip}",
        "je 99f",
    "24:",
        "movzx {inst:e}, byte ptr[{instrs} + {ip}]",
        "add {pos:e}, dword ptr[{dir_table} + {inst} * 2]",
        "cmp byte ptr[{map} + {pos}], 46",
        "je 20b",
        "jb 21b",
    // O
        "mov {block_pos:e}, {pos:e}",
    "22:",
    // O repeats
        "add {block_pos:e}, dword ptr[{dir_table} + {inst} * 2]",
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
    static DIR_TABLE: [i16; 256] = {
        let mut dir_table = [0; 256];
        dir_table[b'>' as usize] = 1;
        dir_table[b'v' as usize] = 128;
        dir_table[b'<' as usize] = -1;
        dir_table[b'<' as usize + 1] = -1;
        dir_table[b'^' as usize] = -128;
        dir_table[b'^' as usize + 1] = -1;
        dir_table
    };
    static mut MAP: [i8; 6400] = [0; 6400];
    let map = &mut MAP;

    #[inline(never)]
    unsafe fn init(s: &[u8], map: &mut [i8; 6400]) {
        for y in 0..50 {
            for x in 0..50 {
                let (a, b) = match *s.get_unchecked(y * 51 + x) {
                    b'O' => (0, 1),
                    b'#' => (-2, -2),
                    _ => (-1, -1),
                };
                *map.get_unchecked_mut(y * 128 + x * 2) = a;
                *map.get_unchecked_mut(y * 128 + x * 2 + 1) = b;
            }
        }
    }

    init(s, map);

    let pos = 24usize * 128 + 48;

    asm!(
        "jmp 24f",
    "21:",
        "sub {pos:e}, dword ptr[{dir_table} + {inst} * 2]",
    "20:",
        "inc {ip}",
        "je 99f",
    "24:",
        "movzx {inst:e}, byte ptr[{instrs} + {ip}]",
        "add {pos:e}, dword ptr[{dir_table} + {inst} * 2]",
        "cmp byte ptr[{map} + {pos}], -1",
        "je 20b", // .
        "jl 21b", // #
        // []
        "mov {bpos:e}, {pos:e}",
        "mov {step:e}, dword ptr[{dir_table} + {inst} * 2]",
        "cmp {step:l}, -128",
        "je 25f", // vertical
        // horizontal
        "add {step:e}, {step:e}",
    "26:",
        "add {bpos:e}, {step:e}",
        "cmp byte ptr[{map} + {bpos}], -1",
        "jg 26b", // [] repeats
        "jl 21b", // [] then # // TODO optimize
        // [] then .
        "cmp byte ptr[{map} + {pos}], 0",
        "je 27f", // right
        // left
    "28:",
        "mov word ptr[{map} + {bpos}], 256",
        "sub {bpos:e}, {step:e}",
        "cmp {bpos:e}, {pos:e}",
        "jne 28b",
        "mov byte ptr[{map} + {bpos}], -1",
        "inc {ip}",
        "jne 24b",
        "jmp 99f",
    "27:",
        "mov word ptr[{map} + {bpos} - 1], 256",
        "sub {bpos:e}, {step:e}",
        "cmp {bpos:e}, {pos:e}",
        "jne 27b",
        "mov byte ptr[{map} + {bpos}], -1",
        "inc {ip}",
        "jne 24b",
        "jmp 99f",
    "31:",
        "mov rsp, {saved_rsp}",
        "inc {ip}",
        "jne 24b",
        "jmp 99f",
    "33:",
        "inc {bpos:e}",
    "30:",
        "sub {bpos:l}, byte ptr[{map} + {bpos}]", // align block position to left
        "add {bpos:e}, {step:e}",
        "cmp byte ptr[{map} + {bpos}], -1",
        "jl 31b", // #
        "je 32f", // .
        "cmp byte ptr[{map} + {bpos}], 0",
        "je 30b",
        "push {bpos}",
        "call 30b",
        "pop {bpos}",
    "32:",
        "cmp byte ptr[{map} + {bpos} + 1], -1",
        "jl 31b", // #
        "jg 33b", // []
        // .
        "ret",
    "35:",
        "sub {bpos2:l}, byte ptr[{map} + {bpos2}]", // align block position to left
        "mov word ptr[{map} + {bpos2}], -1",
        "add {bpos2:e}, {step:e}",
        "cmp byte ptr[{map} + {bpos2}], 0",
        "push {bpos2}",
        "jl 36f", // done
        "call 35b",
        "mov {bpos2}, qword ptr[rsp]",
    "36:",
        "inc {bpos2:e}",
        "cmp byte ptr[{map} + {bpos2}], 0",
        "jl 37f", // done
        "call 35b",
    "37:",
        "pop {bpos2}",
        "mov word ptr[{map} + {bpos2}], 256",
        "ret",
    "25:",
        "mov {saved_rsp}, rsp",
        "mov {bpos2:e}, {bpos:e}",
        "call 30b", // check pushability
        "call 35b", // returned normally, so we can push
        "inc {ip}",
        "jne 24b",
    "99:",
        instrs = in(reg) s.as_ptr_range().end,
        ip = inout(reg) -20020isize => _,
        map = in(reg) map,
        pos = inout(reg) pos => _,
        inst = out(reg) _,
        bpos = out(reg) _,
        bpos2 = out(reg) _,
        step = out(reg) _,
        saved_rsp = out(reg) _,
        dir_table = inout(reg) &DIR_TABLE => _,
    );

    #[inline(never)]
    fn count(map: &mut [i8; 6400]) -> u32 {
        map.iter()
            .zip((0..50).flat_map(|y| (0..102).map(move |x| (x, y))))
            .map(|(c, (x, y))| (*c == 0) as u32 * (x + y * 100))
            .sum()
    }

    count(map)
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
