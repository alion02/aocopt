use super::*;

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u64 {
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    #[repr(C, align(8))]
    #[derive(Clone, Copy)]
    struct Node {
        a: u16,
        ctrl: u8,
        _padding1: u8,
        b: u16,
        _padding2: u16,
    }
    static mut GRAPH: [Node; 36 * 36 * 36] = [Node {
        a: 0,
        ctrl: 0,
        _padding1: 0,
        b: 0,
        _padding2: 0,
    }; 36 * 36 * 36];
    let graph = &mut GRAPH;
    for c1 in b'x'..b'y' + 1 {
        for c2 in b'0'..b'4' + 1 {
            for c3 in b'0'..if c2 == b'4' { b'4' } else { b'9' } + 1 {
                let idx = (c1 - b'a' + 10) as u32 * 36 * 36 + (c2 - b'0') as u32 * 36 + (c3 - b'0') as u32;
                graph.get_unchecked_mut(idx as usize).ctrl = *ptr.add(5) & 1;
                ptr = ptr.add(7);
            }
        }
    }

    asm!(
    "20:",
        "cmp byte ptr[{ptr} + 5], {ascii_O}",
        "vmovdqu {chunk}, [{ptr}]",
        "jne 21f",
        // OR
        "vpblendw {chunk}, {chunk}, [{ptr} + 2], 0xC0",
        "vpsubb {tmp}, {chunk}, {vec_ascii_am10}",
        "vpsubb {chunk}, {chunk}, {vec_ascii_0}",
        "vpblendvb {chunk}, {tmp}, {chunk}, {tmp}",
        "vpmaddubsw {chunk}, {mults_short}, {chunk}",
        "vpmaddwd {chunk}, {chunk}, {mults_wide}",
        "vpextrd {idx:e}, {chunk}, 3",
        "vpackusdw {chunk}, {chunk}, {chunk}",
        "vmovq [{graph} + {idx} * 8], {chunk}",
        "mov byte ptr[{graph} + {idx} * 8 + 2], 3",
        "add {ptr}, 18",
        "cmp {ptr}, {end}",
        "jne 20b",
        "jmp 40f",
    "21:",
        "vpblendw {chunk}, {chunk}, [{ptr} + 3], 0xC0",
        "vpsubb {tmp}, {chunk}, {vec_ascii_am10}",
        "vpsubb {chunk}, {chunk}, {vec_ascii_0}",
        "vpblendvb {chunk}, {tmp}, {chunk}, {tmp}",
        "vpmaddubsw {chunk}, {mults_long}, {chunk}",
        "vpmaddwd {chunk}, {chunk}, {mults_wide}",
        "vpextrd {idx:e}, {chunk}, 3",
        "vpackusdw {chunk}, {chunk}, {chunk}",
        "vmovq [{graph} + {idx} * 8], {chunk}",
        "jl 22f",
        // XOR
        "mov byte ptr[{graph} + {idx} * 8 + 2], 4",
        "add {ptr}, 19",
        "cmp {ptr}, {end}",
        "jne 20b",
        "jmp 40f",
    "22:",
        // AND
        "mov byte ptr[{graph} + {idx} * 8 + 2], 2",
        "add {ptr}, 19",
        "cmp {ptr}, {end}",
        "jne 20b",
    "40:",

        chunk = out(xmm_reg) _,
        tmp = out(xmm_reg) _,
        vec_ascii_am10 = in(xmm_reg) u8x16::splat(b'a' - 10),
        vec_ascii_0 = in(xmm_reg) u8x16::splat(b'0'),
        mults_short = in(xmm_reg) u8x16::from_array([0, 36, 36, 1, 0, 0, 0, 0, 36, 1, 1, 0, 0, 36, 36, 1]),
        mults_long = in(xmm_reg) u8x16::from_array([0, 36, 36, 1, 0, 0, 0, 0, 0, 36, 36, 1, 0, 36, 36, 1]),
        mults_wide = in(xmm_reg) u16x8::from_array([36, 1, 36, 1, 36, 1, 36, 1]),
        ptr = inout(reg) ptr => _,
        idx = out(reg) _,
        ascii_O = const b'O',
        graph = in(reg) graph,
        end = in(reg) r.end.sub(1),

        options(nostack),
    );

    let addr: usize;
    asm!(
        "lea {addr}, [rip + 30f]",
        "jmp 40f",
    "20:",
        "cmp byte ptr[rdx + rcx * 8 + 2], 2",
        "jge 30f",
        "movzx esi, byte ptr[rdx + rcx * 8 + 2]",
        "ret",
    "30:",
        "push rcx",
        "movzx ecx, word ptr[rdx + rcx * 8]",
        "call 20b", // inline
        "mov rcx, [rsp]",
        "push rsi",
        "movzx ecx, word ptr[rdx + rcx * 8 + 4]",
        "call 20b", // inline
        "pop rdi",
        "pop rcx",
        "cmp byte ptr[rdx + rcx * 8 + 2], 3",
        "jne 31f",
        "or esi, edi",
        "mov byte ptr[rdx + rcx * 8 + 2], sil",
        "ret",
    "31:",
        "jl 32f",
        "xor esi, edi",
        "mov byte ptr[rdx + rcx * 8 + 2], sil",
        "ret",
    "32:",
        "and esi, edi",
        "mov byte ptr[rdx + rcx * 8 + 2], sil",
        "ret",
    "40:",
        addr = out(reg) addr,
        out("rdx") _,
        out("rcx") _,
        out("rsi") _,
        out("rdi") _,
        options(nostack, nomem, pure, preserves_flags),
    );
    let mut total = 0;
    let c1 = b'z';
    for c2 in (b'0'..b'4' + 1).rev() {
        for c3 in (b'0'..if c2 == b'4' { b'5' } else { b'9' } + 1).rev() {
            let idx = (c1 - b'a' + 10) as u32 * 36 * 36 + (c2 - b'0') as u32 * 36 + (c3 - b'0') as u32;
            let res: u64;
            asm!(
                "call {addr}",
                addr = in(reg) addr,
                in("rdx") graph,
                inout("rcx") idx as usize => _,
                out("rsi") res,
                out("rdi") _,
            );
            total = res + total * 2;
        }
    }
    total
}

#[inline]
#[repr(align(64))]
unsafe fn inner2(s: &[u8]) -> &str {
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    #[repr(C, align(8))]
    #[derive(Clone, Copy)]
    struct Node {
        a: u16,
        ctrl: u8,
        _padding1: u8,
        b: u16,
        _padding2: u16,
    }
    static mut GRAPH: [Node; 36 * 36 * 36] = [Node {
        a: 0,
        ctrl: 0,
        _padding1: 0,
        b: 0,
        _padding2: 0,
    }; 36 * 36 * 36];
    let graph = &mut GRAPH;
    for _c1 in b'x'..b'y' + 1 {
        for c2 in b'0'..b'4' + 1 {
            for _c3 in b'0'..if c2 == b'4' { b'4' } else { b'9' } + 1 {
                ptr = ptr.add(7);
            }
        }
    }

    static mut BUF: [u16; 8] = [0; 8];
    let buf = &mut BUF;
    let mut buf_idx = 0usize;

    asm!(
    "20:",
        "cmp byte ptr[{ptr} + 5], {ascii_O}",
        "jne 21f",
        // OR
        "vmovdqu {chunk}, [{ptr}]",
        "vpblendw {chunk}, {chunk}, [{ptr} + 2], 0xC0",
        "vpsubb {vec_tmp}, {chunk}, {vec_ascii_am10}",
        "vpsubb {chunk}, {chunk}, {vec_ascii_0}",
        "vpblendvb {chunk}, {vec_tmp}, {chunk}, {vec_tmp}",
        "vpmaddubsw {chunk}, {mults_short}, {chunk}",
        "vpmaddwd {chunk}, {chunk}, {mults_wide}",
        "vpextrd {idx:e}, {chunk}, 3",
        "vpackusdw {chunk}, {chunk}, {chunk}",
        "vmovq [{graph} + {idx} * 8], {chunk}",
        "mov byte ptr[{graph} + {idx} * 8 + 2], 3",
        "cmp byte ptr[{ptr} + 15], {ascii_z}",
        "jne 99f",
        "cmp word ptr[{ptr} + 16], {ascii_45}",
        "je 99f",
        "mov word ptr[{buf} + {buf_idx} * 2], {idx:x}",
        "inc {buf_idx:e}",
    "99:",
        "add {ptr}, 18",
        "cmp {ptr}, {end}",
        "jne 20b",
        "jmp 40f",
    "21:",
        "jl 22f",
        // XOR
        "vmovdqu {chunk}, [{ptr}]",
        "vpblendw {chunk}, {chunk}, [{ptr} + 3], 0xC0",
        "vpsubb {vec_tmp}, {chunk}, {vec_ascii_am10}",
        "vpsubb {chunk}, {chunk}, {vec_ascii_0}",
        "vpblendvb {chunk}, {vec_tmp}, {chunk}, {vec_tmp}",
        "vpmaddubsw {chunk}, {mults_long}, {chunk}",
        "vpmaddwd {chunk}, {chunk}, {mults_wide}",
        "vpextrd {idx:e}, {chunk}, 3",
        "vpackusdw {chunk}, {chunk}, {chunk}",
        "vmovq [{graph} + {idx} * 8], {chunk}",
        "mov byte ptr[{graph} + {idx} * 8 + 2], 4",
        "cmp byte ptr[{ptr} + 2], {ascii_a}",
        "jl 99f",
        "cmp byte ptr[{ptr} + 17], {ascii_a}",
        "jl 99f",
        "mov byte ptr[{graph} + {idx} * 8 + 2], 2",
        "mov word ptr[{buf} + {buf_idx} * 2], {idx:x}",
        "inc {buf_idx:e}",
    "99:",
        "add {ptr}, 19",
        "cmp {ptr}, {end}",
        "jne 20b",
        "jmp 40f",
    "22:",
        // AND
        "vmovdqu {chunk}, [{ptr}]",
        "vpblendw {chunk}, {chunk}, [{ptr} + 3], 0xC0",
        "vpsubb {vec_tmp}, {chunk}, {vec_ascii_am10}",
        "vpsubb {chunk}, {chunk}, {vec_ascii_0}",
        "vpblendvb {chunk}, {vec_tmp}, {chunk}, {vec_tmp}",
        "vpmaddubsw {chunk}, {mults_long}, {chunk}",
        "vpmaddwd {chunk}, {chunk}, {mults_wide}",
        "vpextrd {idx:e}, {chunk}, 3",
        "vpackusdw {chunk}, {chunk}, {chunk}",
        "vmovq [{graph} + {idx} * 8], {chunk}",
        "mov byte ptr[{graph} + {idx} * 8 + 2], 2",
        "cmp byte ptr[{ptr} + 16], {ascii_z}",
        "jne 99f",
        "mov word ptr[{buf} + {buf_idx} * 2], {idx:x}",
        "inc {buf_idx:e}",
    "99:",
        "add {ptr}, 19",
        "cmp {ptr}, {end}",
        "jne 20b",
    "40:",

        chunk = out(xmm_reg) _,
        vec_tmp = out(xmm_reg) _,
        vec_ascii_am10 = in(xmm_reg) u8x16::splat(b'a' - 10),
        vec_ascii_0 = in(xmm_reg) u8x16::splat(b'0'),
        mults_short = in(xmm_reg) u8x16::from_array([0, 36, 36, 1, 0, 0, 0, 0, 36, 1, 1, 0, 0, 36, 36, 1]),
        mults_long = in(xmm_reg) u8x16::from_array([0, 36, 36, 1, 0, 0, 0, 0, 0, 36, 36, 1, 0, 36, 36, 1]),
        mults_wide = in(xmm_reg) u16x8::from_array([36, 1, 36, 1, 36, 1, 36, 1]),
        ptr = inout(reg) ptr => _,
        idx = out(reg) _,
        graph = in(reg) graph,
        end = in(reg) r.end.sub(1),
        buf = in(reg) buf,
        buf_idx = inout(reg) buf_idx,
        ascii_O = const b'O',
        ascii_a = const b'a',
        ascii_z = const b'z',
        ascii_45 = const u16::from_ne_bytes([b'4', b'5']),

        options(nostack),
    );

    asm!(
    "20:",
        "cmp byte ptr[{ptr} + 5], {ascii_O}",
        "jne 21f",
        // OR
        "vmovdqu {chunk}, [{ptr}]",
        "vpblendw {chunk}, {chunk}, [{ptr} + 2], 0xC0",
        "vpsubb {vec_tmp}, {chunk}, {vec_ascii_am10}",
        "vpsubb {chunk}, {chunk}, {vec_ascii_0}",
        "vpblendvb {chunk}, {vec_tmp}, {chunk}, {vec_tmp}",
        "vpmaddubsw {chunk}, {mults_short}, {chunk}",
        "vpmaddwd {chunk}, {chunk}, {mults_wide}",
        "vpextrd {idx:e}, {chunk}, 3",
        "movzx {a:e}, word ptr[{graph} + {idx} * 8]",
        "movzx {b:e}, word ptr[{graph} + {idx} * 8 + 4]",
        "cmp byte ptr[{graph} + {a} * 8 + 2], 2",
        "jne 22f",
        "cmp byte ptr[{graph} + {b} * 8 + 2], 2",
        "jne 23f",
    "99:",
        "add {ptr}, 18",
        "cmp {ptr}, {end}",
        "jne 20b",
        "jmp 40f",
    "22:",
        "mov word ptr[{buf} + {buf_idx} * 2], {a:x}",
        "inc {buf_idx:e}",
        "movzx {a:e}, word ptr[{graph} + {b} * 8]",
        "cmp byte ptr[{graph} + {a} * 8 + 2], 2",
        "je 200f",
        "movzx {a:e}, word ptr[{graph} + {b} * 8 + 4]",
    "200:",
        "mov word ptr[{buf} + {buf_idx} * 2], {a:x}",
        "inc {buf_idx:e}",
        "jmp 99b",
    "23:",
        "mov word ptr[{buf} + {buf_idx} * 2], {b:x}",
        "inc {buf_idx:e}",
        "movzx {b:e}, word ptr[{graph} + {a} * 8]",
        "cmp byte ptr[{graph} + {b} * 8 + 2], 2",
        "je 200f",
        "movzx {b:e}, word ptr[{graph} + {a} * 8 + 4]",
    "200:",
        "mov word ptr[{buf} + {buf_idx} * 2], {b:x}",
        "inc {buf_idx:e}",
        "jmp 99b",
    "21:",
        "add {ptr}, 19",
        "cmp {ptr}, {end}",
        "jne 20b",
    "40:",

        chunk = out(xmm_reg) _,
        vec_tmp = out(xmm_reg) _,
        vec_ascii_am10 = in(xmm_reg) u8x16::splat(b'a' - 10),
        vec_ascii_0 = in(xmm_reg) u8x16::splat(b'0'),
        mults_short = in(xmm_reg) u8x16::from_array([0, 36, 36, 1, 0, 0, 0, 0, 36, 1, 1, 0, 0, 36, 36, 1]),
        mults_wide = in(xmm_reg) u16x8::from_array([36, 1, 36, 1, 36, 1, 36, 1]),
        ptr = inout(reg) ptr => _,
        idx = out(reg) _,
        graph = in(reg) graph,
        end = in(reg) r.end.sub(1),
        buf = in(reg) buf,
        buf_idx = inout(reg) buf_idx,
        a = out(reg) _,
        b = out(reg) _,
        ascii_O = const b'O',

        options(nostack),
    );

    assert_eq!(buf_idx, 8);

    static mut OUT: [u32; 32] = [0; 32];
    static REV_IDX: [u32; 36 * 36 * 36] = unsafe {
        let mut rev_idx = [[b','; 4]; 36 * 36 * 36];
        let mut i = 0;
        let mut c1 = b'0';
        while c1 <= b'z' {
            let mut c2 = b'0';
            while c2 <= b'z' {
                let mut c3 = b'0';
                while c3 <= b'z' {
                    let rev = &mut rev_idx[i];
                    rev[0] = c1;
                    rev[1] = c2;
                    rev[2] = c3;
                    i += 1;
                    c3 += 1;
                    if c3 == b'9' + 1 {
                        c3 = b'a';
                    }
                }
                c2 += 1;
                if c2 == b'9' + 1 {
                    c2 = b'a';
                }
            }
            c1 += 1;
            if c1 == b'9' + 1 {
                c1 = b'a';
            }
        }
        transmute(rev_idx)
    };
    let mut out = OUT.as_mut_ptr();
    let mut buf = buf.as_mut_ptr().cast::<u16x8>().read_unaligned();
    for _ in 0..8 {
        let min: u16x8 = _mm_minpos_epu16(buf.into()).into();
        *out = *REV_IDX.get_unchecked(min[0] as usize);
        buf[min[1] as usize] = !0;
        out = out.add(1);
    }

    std::str::from_utf8_unchecked(std::slice::from_raw_parts(OUT.as_ptr().cast::<u8>(), 31))
}

#[inline]
pub fn part1(s: &str) -> u64 {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> &str {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/24.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/24p1.txt").unwrap(),);
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/24.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/24p2.txt").unwrap(),);
    }
}
