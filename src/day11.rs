use super::*;

unsafe fn inner1(s: &[u8]) -> u32 {
    #[repr(C)]
    struct Tables {
        mults: [u8x8; 8],
        lookup: [u32; 10_000_000],
    }

    static TABLES: Tables = Tables {
        mults: unsafe {
            let mut mask = [[0u8; 8]; 8];
            let mut i = 0;
            while i < 8 {
                let mut j = 0;
                while j < 8 {
                    mask[i][j] = if j + i >= 8 {
                        if j % 2 == 0 {
                            10
                        } else {
                            1
                        }
                    } else {
                        0
                    };
                    j += 1;
                }
                i += 1;
            }
            transmute(mask)
        },
        lookup: unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day11_depth25.bin"))) },
    };

    let mut res = 0;
    let tables = (&raw const TABLES).byte_add(64);

    asm!(
        "vpaddb {y}, {off}, ymmword ptr[{ptr}]",
        "vpaddb {x}, {off:x}, xmmword ptr[{ptr} + 20]",
        "vmovdqu ymmword ptr[rsp - 36], {y}",
        "vmovdqu xmmword ptr[rsp - 16], {x}",
        "mov byte ptr[rsp - 37], -1",
    "20:",
        "vmovdqu {x}, xmmword ptr[rsp + {len} - 53]",
        "vpmovmskb {r1:e}, {x}",
        "lzcnt {r1:x}, {r1:x}",
        "vpmaddubsw {x}, {x}, xmmword ptr[{tables} + {r1} * 8 - 72]",
        "vpmaddwd {x}, {x}, {mults100}",
        "vpackusdw {x}, {x}, {x}",
        "vpmaddwd {x}, {x}, {mults10000}",
        "vpextrd {r2:e}, {x}, 3",
        "add {res:e}, [{tables} + {r2} * 4]",
        "sub {len:e}, {r1:e}",
        "dec {len:e}",
        "jne 20b",
        x = out(xmm_reg) _,
        y = out(ymm_reg) _,
        off = in(ymm_reg) i8x32::splat(-48),
        mults100 = in(xmm_reg) u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]),
        mults10000 = in(xmm_reg) u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]),
        res = inout(reg) res,
        ptr = in(reg) s.as_ptr(),
        len = inout(reg) s.len() => _,
        r1 = out(reg) _,
        r2 = out(reg) _,
        tables = in(reg) tables,
    );

    res
}

unsafe fn inner2(s: &[u8]) -> u64 {
    #[repr(C)]
    struct Tables {
        mults: [u8x8; 8],
        lookup: [u64; 10_000_000],
    }

    static TABLES: Tables = Tables {
        mults: unsafe {
            let mut mask = [[0u8; 8]; 8];
            let mut i = 0;
            while i < 8 {
                let mut j = 0;
                while j < 8 {
                    mask[i][j] = if j + i >= 8 {
                        if j % 2 == 0 {
                            10
                        } else {
                            1
                        }
                    } else {
                        0
                    };
                    j += 1;
                }
                i += 1;
            }
            transmute(mask)
        },
        lookup: unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day11_depth75.bin"))) },
    };

    let mut res = 0;
    let tables = (&raw const TABLES).byte_add(64);

    asm!(
        "vpaddb {y}, {off}, ymmword ptr[{ptr}]",
        "vpaddb {x}, {off:x}, xmmword ptr[{ptr} + 20]",
        "vmovdqu ymmword ptr[rsp - 36], {y}",
        "vmovdqu xmmword ptr[rsp - 16], {x}",
        "mov byte ptr[rsp - 37], -1",
    "20:",
        "vmovdqu {x}, xmmword ptr[rsp + {len} - 53]",
        "vpmovmskb {r1:e}, {x}",
        "lzcnt {r1:x}, {r1:x}",
        "vpmaddubsw {x}, {x}, xmmword ptr[{tables} + {r1} * 8 - 72]",
        "vpmaddwd {x}, {x}, {mults100}",
        "vpackusdw {x}, {x}, {x}",
        "vpmaddwd {x}, {x}, {mults10000}",
        "vpextrd {r2:e}, {x}, 3",
        "add {res}, [{tables} + {r2} * 8]",
        "sub {len:e}, {r1:e}",
        "dec {len:e}",
        "jne 20b",
        x = out(xmm_reg) _,
        y = out(ymm_reg) _,
        off = in(ymm_reg) i8x32::splat(-48),
        mults100 = in(xmm_reg) u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]),
        mults10000 = in(xmm_reg) u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]),
        res = inout(reg) res,
        ptr = in(reg) s.as_ptr(),
        len = inout(reg) s.len() => _,
        r1 = out(reg) _,
        r2 = out(reg) _,
        tables = in(reg) tables,
    );

    res
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
        let s = read_to_string("./inputs/11.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/11p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/11.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/11p2.txt").unwrap(),);
    }
}
