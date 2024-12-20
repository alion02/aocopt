use super::*;

#[inline]
unsafe fn inner1(s: &[u8]) -> u32 {
    static mut TRIE: [[u16; 6]; 4096] = [[0; 6]; 4096];

    let mut ptr = s.as_ptr();
    let trie = TRIE.as_mut_ptr();
    *trie = [0; 6];
    let mut len = 0;

    macro_rules! hash {
        ($byte:expr) => {
            _pext_u32($byte as u32, 83).wrapping_sub(10)
        };
    }

    macro_rules! is_lf {
        ($hash:expr) => {
            $hash == 2u32.wrapping_sub(10)
        };
    }

    loop {
        let mut hash = hash!(*ptr);
        let mut curr = 0;
        loop {
            ptr = ptr.add(1);
            let next = trie.byte_add(curr).cast::<u16>().add(hash as usize);
            if *next == 0 {
                len += 3;
                *next = len;
                *trie.byte_add(len as usize * 4) = [0; 6];
            }

            hash = hash!(*ptr);
            curr = *next as usize * 4;
            if (hash as i32) < 0 {
                break;
            }
        }

        ptr = ptr.add(2);
        assert!(*ptr > 64);
        *trie.byte_add(curr).cast::<u16>().add(2) = 1;
        if is_lf!(hash) {
            break;
        }
    }

    let mut total = 0;

    asm!(
        "mov {saved_rsp}, rsp",
        "jmp 20f",

    "203:",
        "inc {i:e}",
        "lea {node}, [{trie} + {tmp} * 4]",
    "200:",
        "cmp byte ptr[{node} + 4], 0",
        "je 201f", // try continuing
        "bts {seen}, {i}",
        "jc 201f", // memoized: can't finish pattern here
        "cmp byte ptr[{ptr} + {i}], {lf}",
        "je 202f", // success
        // try finishing this pattern
        "push {i}",
        "push {node}",
        "mov {node}, {trie}",
        "call 201f",
        "pop {node}",
        "pop {i}",
    "201:",
        "movzx {hash:e}, byte ptr[{ptr} + {i}]",
        "pext {hash:e}, {hash:e}, {hash_mask:e}",
        "sub {hash:e}, {hash_offset}",
        "js 204f", // in the middle of a pattern but towel is done
        "movzx {tmp:e}, word ptr[{node} + {hash} * 2]",
        "test {tmp:e}, {tmp:e}",
        "jne 203b", // continue
    "204:",
        "ret", // dead end

    "202:",
        "mov rsp, {saved_rsp}",
        "inc {total:e}",
        "lea {ptr}, [{ptr} + {i} + 1]",
        "cmp {ptr}, {end}",
        "je 22f",
    "20:",
        "mov {node}, {trie}",
        "xor {seen:e}, {seen:e}",
        "xor {i:e}, {i:e}",
        "call 201b",
    "21:",
        "inc {ptr}",
        "cmp byte ptr[{ptr}], {lf}",
        "jne 21b",
        "inc {ptr}",
        "cmp {ptr}, {end}",
        "jne 20b",
    "22:",

        saved_rsp = out(reg) _,
        seen = out(reg) _,
        i = out(reg) _,
        ptr = inout(reg) ptr => _,
        end = in(reg) s.as_ptr_range().end,
        hash = out(reg) _,
        node = out(reg) _,
        tmp = out(reg) _,
        trie = in(reg) trie,
        hash_mask = in(reg) 83,
        hash_offset = const 10,
        total = inout(reg) total,
        lf = const b'\n',
    );

    total
}

#[inline]
unsafe fn inner2(s: &[u8]) -> u64 {
    static mut TRIE: [[u16; 6]; 4096] = [[0; 6]; 4096];

    let mut ptr = s.as_ptr();
    let trie = TRIE.as_mut_ptr();
    *trie = [0; 6];
    let mut len = 0;

    macro_rules! hash {
        ($byte:expr) => {
            _pext_u32($byte as u32, 83).wrapping_sub(10)
        };
    }

    macro_rules! is_lf {
        ($hash:expr) => {
            $hash == 2u32.wrapping_sub(10)
        };
    }

    loop {
        let mut hash = hash!(*ptr);
        let mut curr = 0;
        loop {
            ptr = ptr.add(1);
            let next = trie.byte_add(curr).cast::<u16>().add(hash as usize);
            if *next == 0 {
                len += 3;
                *next = len;
                *trie.byte_add(len as usize * 4) = [0; 6];
            }

            hash = hash!(*ptr);
            curr = *next as usize * 4;
            if (hash as i32) < 0 {
                break;
            }
        }

        ptr = ptr.add(2);
        assert!(*ptr > 64);
        *trie.byte_add(curr).cast::<u16>().add(2) = 1;
        if is_lf!(hash) {
            break;
        }
    }

    static mut SEEN: [u64x4; 16] = [Simd::from_array([0; 4]); 16];

    let mut total = 0;

    asm!(
        "jmp 20f",

    "203:",
        "inc {i:e}",
        "lea {node}, [{trie} + {tmp} * 4]",
    "200:",
        "cmp byte ptr[{node} + 4], 0",
        "je 201f", // try continuing
        "cmp qword ptr[{seen} + {i} * 8], -1",
        "jne 205f", // memoized: add result and return
        "cmp byte ptr[{ptr} + {i}], {lf}",
        "je 202f", // success: add 1
        // try finishing this pattern
        "push {i}",
        "push {node}",
        "mov {node}, {trie}",
        "call 201f",
        "pop {node}",
        "pop {i}",
    "201:",
        "push {i}",
        "movzx {hash:e}, byte ptr[{ptr} + {i}]",
        "pext {hash:e}, {hash:e}, {hash_mask:e}",
        "sub {hash:e}, {hash_offset}",
        "js 204f", // in the middle of a pattern but towel is done
        "movzx {tmp:e}, word ptr[{node} + {hash} * 2]",
        "test {tmp:e}, {tmp:e}",
        "jne 203b", // continue
        // dead end
    "204:",
        "pop {i}",
        "mov qword ptr[{seen} + {i} * 8], {curr_total}",
        "ret",
    "205:",
        "add {curr_total}, qword ptr[{seen} + {i} * 8 - 8]",
        "pop {i}",
        "ret",
    "202:",
        "inc {curr_total}",
        "pop {i}",
        "ret",

    "20:",
        "vmovdqa ymmword ptr[{seen}], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 32], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 64], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 96], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 128], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 160], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 192], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 224], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 256], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 288], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 320], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 352], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 384], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 416], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 448], {all_ones}",
        "vmovdqa ymmword ptr[{seen} + 480], {all_ones}",
        "mov {node}, {trie}",
        "xor {i:e}, {i:e}",
        "xor {curr_total:e}, {curr_total:e}",
        "call 201b",
        "add {total}, {curr_total}",
    "21:",
        "inc {ptr}",
        "cmp byte ptr[{ptr}], {lf}",
        "jne 21b",
        "inc {ptr}",
        "cmp {ptr}, {end}",
        "jne 20b",
    "22:",

        seen = in(reg) &mut SEEN,
        i = out(reg) _,
        ptr = inout(reg) ptr => _,
        end = in(reg) s.as_ptr_range().end,
        hash = out(reg) _,
        node = out(reg) _,
        tmp = out(reg) _,
        trie = in(reg) trie,
        hash_mask = in(reg) 83,
        hash_offset = const 10,
        curr_total = out(reg) _,
        total = inout(reg) total,
        all_ones = in(ymm_reg) u64x4::splat(!0),
        lf = const b'\n',
    );

    total
}

#[inline]
pub fn part1(s: &str) -> u32 {
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
        let s = read_to_string("./inputs/19.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/19p1.txt").unwrap(),);
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/19.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/19p2.txt").unwrap(),);
    }
}
