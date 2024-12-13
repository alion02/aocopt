use super::*;

const ROW: usize = 141;
const SIDE: usize = ROW - 1;
const BYTES: usize = ROW * SIDE;
const MARGIN: usize = ROW + 32;
const TABLE_WITH_MARGINS: usize = BYTES + MARGIN * 2;

unsafe fn inner1(s: &[u8]) -> u32 {
    static mut EDGES: [i8; TABLE_WITH_MARGINS] = [-1; TABLE_WITH_MARGINS];

    let edges = EDGES.as_mut_ptr().add(MARGIN).cast::<i8x32>();
    let ptr = s.as_ptr().cast::<i8x32>();

    for off in (0..SIDE).step_by(32) {
        let off = off.min(SIDE - 32);
        let mut up = Simd::splat(0);
        let mut mid = ptr.byte_add(off).read_unaligned();
        for row in 0..SIDE {
            let off = off + row * ROW;
            let left = if off == 0 {
                simd_swizzle!(ptr.read_unaligned(), Simd::splat(0), [
                    32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                    26, 27, 28, 29, 30,
                ])
            } else {
                ptr.byte_add(off - 1).read_unaligned()
            };
            let right = ptr.byte_add(off + 1).read_unaligned();
            let down = if row + 1 == SIDE {
                Simd::splat(0)
            } else {
                ptr.byte_add(off + ROW).read_unaligned()
            };
            let up_mask = mid.simd_eq(up).to_int() & Simd::splat(8);
            let left_mask = mid.simd_eq(left).to_int() & Simd::splat(4);
            let right_mask = mid.simd_eq(right).to_int() & Simd::splat(1);
            let down_mask = mid.simd_eq(down).to_int() & Simd::splat(2);
            edges
                .byte_add(off)
                .write_unaligned(up_mask | left_mask | right_mask | down_mask);
            up = mid;
            mid = down;
        }
    }

    // unsafe fn flood(ptr: *mut i8) -> (u32, u32) {
    //     let cell = ptr.read();
    //     if cell < 0 {
    //         return (0, 0);
    //     }
    //     ptr.write(-1);
    //     let mut area = 1;
    //     let mut len = (cell ^ 15).count_ones();
    //     if cell & 1 != 0 {
    //         let (a, l) = flood(ptr.add(1));
    //         area += a;
    //         len += l;
    //     }
    //     if cell & 2 != 0 {
    //         let (a, l) = flood(ptr.add(ROW));
    //         area += a;
    //         len += l;
    //     }
    //     if cell & 4 != 0 {
    //         let (a, l) = flood(ptr.offset(-1));
    //         area += a;
    //         len += l;
    //     }
    //     if cell & 8 != 0 {
    //         let (a, l) = flood(ptr.offset(-(ROW as isize)));
    //         area += a;
    //         len += l;
    //     }
    //     (area, len)
    // }

    let mut total = 0;
    for i in 0..BYTES - 1 {
        let (mut area, mut len) = (0, 0);
        asm!(
            // "lea {table}, [rip + 22222f]",
            "call 20f",
            "jmp 99f",
        "20:",
            "movzx {cell:e}, byte ptr[{ptr}]",
            "xor {cell:l}, 15",
            "jns 21f",
            "ret",
        "21:",
            "mov byte ptr[{ptr}], -1",
            "inc {area:e}",
            "popcnt {tmp:e}, {cell:e}",
            "add {len:e}, {tmp:e}",
            "push {cell}",
            "inc {ptr}",
            "test {cell:l}, 1",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, 140",
            "test byte ptr[rsp], 2",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, -142",
            "test byte ptr[rsp], 4",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, -140",
            "test byte ptr[rsp], 8",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, 141",
            "pop {tmp}",
            "ret",
        //     "mov {tmp:e}, dword ptr[{table} + {cell} * 4]",
        //     "add {tmp}, {table}",
        //     "jmp {tmp}",
        // "22222:",
        //     ".long 20000f-22222b",
        //     ".long 20001f-22222b",
        //     ".long 20010f-22222b",
        //     ".long 20011f-22222b",
        //     ".long 20100f-22222b",
        //     ".long 20101f-22222b",
        //     ".long 20110f-22222b",
        //     ".long 20111f-22222b",
        //     ".long 21000f-22222b",
        //     ".long 21001f-22222b",
        //     ".long 21010f-22222b",
        //     ".long 21011f-22222b",
        //     ".long 21100f-22222b",
        //     ".long 21101f-22222b",
        //     ".long 21110f-22222b",
        //     ".long 21111f-22222b",
        "99:",
            // table = out(reg) _,
            tmp = out(reg) _,
            cell = out(reg) _,
            area = inout(reg) area,
            len = inout(reg) len,
            ptr = inout(reg) edges.cast::<i8>().add(i) => _,
        );
        total += area * len;
    }

    total
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
        let s = read_to_string("./inputs/12.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/12p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/12.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/12p2.txt").unwrap(),);
    }
}
