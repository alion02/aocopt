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

    static mut SEEN: [u64; 64] = [!0; 64];
    let seen = &mut SEEN;

    let mut total = 0;

    loop {
        let mut end = 0;
        while *ptr.add(end) != b'\n' {
            end += 1;
        }
        *seen.get_unchecked_mut(end) = 1;
        let mut i = end - 1;
        let mut zero_in_a_row = 0;
        loop {
            let mut curr_total = 0;
            let mut j = i;
            let mut node = trie.cast::<u16>();
            loop {
                let hash = hash!(*ptr.add(j));
                if (hash as i32) < 0 {
                    break;
                }
                let next = *node.add(hash as _);
                if next == 0 {
                    break;
                }
                node = trie.byte_add(next as usize * 4).cast();
                j += 1;
                if *node.add(2) > 0 {
                    curr_total += *seen.get_unchecked_mut(j);
                }
            }

            *seen.get_unchecked_mut(i) = curr_total;
            if curr_total == 0 {
                zero_in_a_row += 1;
                if zero_in_a_row >= 8 {
                    break;
                }
            } else {
                zero_in_a_row = 0;
            }
            i = i.wrapping_sub(1);
            if (i as isize) < 0 {
                total += curr_total;
                break;
            }
        }
        ptr = ptr.add(end + 1);
        if ptr == s.as_ptr_range().end {
            break;
        }
    }

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
