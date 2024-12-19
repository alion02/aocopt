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
            let next = trie.add(curr).cast::<u16>().add(hash as usize);
            if *next == 0 {
                len += 1;
                *next = len;
                *trie.add(len as usize) = [0; 6];
            }

            hash = hash!(*ptr);
            curr = *next as usize;
            if (hash as i32) < 0 {
                break;
            }
        }

        ptr = ptr.add(2);
        assert!(*ptr > 64);
        *trie.add(curr).cast::<u16>().add(2) = 1;
        if is_lf!(hash) {
            break;
        }
    }

    let mut total = 0;

    while ptr != s.as_ptr_range().end {
        unsafe fn check_towel(trie: *mut [u16; 6], mut ptr: *const u8) -> u32 {
            let mut hash = hash!(*ptr);
            if is_lf!(hash) {
                return 1;
            }

            let mut node = trie.cast::<u16>();

            loop {
                let next = *node.add(hash as usize);
                if next == 0 {
                    return 0;
                }

                ptr = ptr.add(1);
                hash = hash!(*ptr);
                node = trie.add(next as usize).cast();

                if *node.add(2) > 0 && check_towel(trie, ptr) > 0 {
                    return 1;
                }
                if is_lf!(hash) {
                    return 0;
                }
            }
        }

        total += check_towel(trie, ptr);
        while *ptr != b'\n' {
            ptr = ptr.add(1);
        }
        ptr = ptr.add(1);
    }

    total
}

#[inline]
unsafe fn inner2(s: &[u8]) -> u64 {
    0
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

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/19p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/19.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/19p2.txt").unwrap(),);
    }
}
