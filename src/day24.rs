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
        "call 20b",
        "mov byte ptr[rdx + rcx * 8 + 2], sil",
        "mov rcx, [rsp]",
        "push rsi",
        "movzx ecx, word ptr[rdx + rcx * 8 + 4]",
        "call 20b",
        "mov byte ptr[rdx + rcx * 8 + 2], sil",
        "pop rdi",
        "pop rcx",
        "cmp byte ptr[rdx + rcx * 8 + 2], 3",
        "jne 31f",
        "or esi, edi",
        "ret",
    "31:",
        "jl 32f",
        "xor esi, edi",
        "ret",
    "32:",
        "and esi, edi",
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
    ""
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
