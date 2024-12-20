use super::*;

#[inline]
unsafe fn inner1(s: &[u8]) -> u32 {
    static mut MAP: [i16; 142 * (141 + 40)] = [i16::MAX; 142 * (141 + 40)];
    let map = &mut MAP;
    map[142 * 20..142 * (141 + 20)].fill(i16::MAX);
    let map = map.as_ptr().add(142 * 20);
    let ptr = s.as_ptr();
    let mut i = 0;
    let mut chunk;
    loop {
        i += 32;
        chunk = ptr.add(i).cast::<u8x32>().read_unaligned();
        if _mm256_testz_si256(chunk.into(), u8x32::splat(0x40).into()) == 0 {
            break;
        }
    }
    i += _mm256_movemask_epi8((chunk << 1).into()).trailing_zeros() as usize;
    assert!(
        s[i] == b'S' || s[i] == b'E',
        "found \"{}\" at {i} in chunk {chunk:?}",
        s[i] as char,
    );
    let mut cuts = 0;
    asm!(
        "mov word ptr[{map} + {i} * 2], 0",
        "cmp byte ptr[{ptr} + {i} + 1], {wall}",
        "jne 200f", // right
        "cmp byte ptr[{ptr} + {i} + 142], {wall}",
        "jne 210f", // down
        "cmp byte ptr[{ptr} + {i} - 1], {wall}",
        "jne 220f", // left
        "cmp byte ptr[{ptr} + {i} - 142], {wall}",
        "jne 230f", // up
        "ud2",
    "300:",
        "add {dist:e}, 1",
        "lea {adj_dist:e}, [{dist:r} - 102]",
        "mov word ptr[{map} + {i} * 2], {dist:x}",
        "cmp word ptr[{map} + {i} * 2 + 4], {adj_dist:x}",
        "jg 30f",
        "inc {cuts:e}",
    "30:",
        "cmp word ptr[{map} + {i} * 2 + 568], {adj_dist:x}",
        "jg 30f",
        "inc {cuts:e}",
    "30:",
        "cmp word ptr[{map} + {i} * 2 - 4], {adj_dist:x}",
        "jg 30f",
        "inc {cuts:e}",
    "30:",
        "cmp word ptr[{map} + {i} * 2 - 568], {adj_dist:x}",
        "jg 30f",
        "inc {cuts:e}",
    "30:",
        "ret",
    "200:", // right
        "add {i:e}, 1",
        "call 300b",
        "add {i:e}, 1",
        "call 300b",
        "cmp byte ptr[{ptr} + {i} + 1], {wall}",
        "jne 200b", // right
        "cmp byte ptr[{ptr} + {i} - 142], {wall}",
        "jne 230f", // up
        "cmp byte ptr[{ptr} + {i} + 142], {wall}",
        "je 20f", // done
    "210:", // down
        "add {i:e}, 142",
        "call 300b",
        "add {i:e}, 142",
        "call 300b",
        "cmp byte ptr[{ptr} + {i} + 142], {wall}",
        "jne 210b", // down
        "cmp byte ptr[{ptr} + {i} + 1], {wall}",
        "jne 200b", // right
        "cmp byte ptr[{ptr} + {i} - 1], {wall}",
        "je 20f", // done
    "220:", // left
        "add {i:e}, -1",
        "call 300b",
        "add {i:e}, -1",
        "call 300b",
        "cmp byte ptr[{ptr} + {i} - 1], {wall}",
        "jne 220b", // left
        "cmp byte ptr[{ptr} + {i} + 142], {wall}",
        "jne 210b", // down
        "cmp byte ptr[{ptr} + {i} - 142], {wall}",
        "je 20f", // done
    "230:", // up
        "add {i:e}, -142",
        "call 300b",
        "add {i:e}, -142",
        "call 300b",
        "cmp byte ptr[{ptr} + {i} - 142], {wall}",
        "jne 230b", // up
        "cmp byte ptr[{ptr} + {i} - 1], {wall}",
        "jne 220b", // left
        "cmp byte ptr[{ptr} + {i} + 1], {wall}",
        "jne 200b", // right
        // done
    "20:",
        ptr = in(reg) ptr,
        map = in(reg) map,
        i = inout(reg) i => _,
        dist = inout(reg) 0 => _,
        adj_dist = out(reg) _,
        cuts = inout(reg) cuts,
        wall = const b'#',
    );

    cuts
}

#[inline]
unsafe fn inner2(s: &[u8]) -> u32 {
    0
}

#[inline]
pub fn part1(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> u32 {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/20.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/20p1.txt").unwrap(),);
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/20.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/20p2.txt").unwrap(),);
    }
}