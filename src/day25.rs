use super::*;

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u32 {
    static mut LOCKS: [u32; 250] = [0; 250];
    static mut KEYS: [u32x8; 32] = [Simd::from_array([!0; 8]); 32];

    let locks = LOCKS.as_mut_ptr();
    let keys = KEYS.as_mut_ptr();

    asm!(
        "jmp 20f",
    "21:",
        "mov [{locks}], {mask:e}",
        "add {locks}, 4",
        "add {i:e}, -43",
        "jl 30f",
    "20:",
        "vpcmpeqb {chunk}, {vec_ascii_hash}, [{ptr} + {i}]",
        "vpmovmskb {mask:e}, {chunk}",
        "test {mask:l}, 1",
        "jnz 21b",
        "mov [{keys}], {mask:e}",
        "add {keys}, 4",
        "add {i:e}, -43",
        "jge 20b",
    "30:",
        locks = inout(reg) locks => _,
        keys = inout(reg) keys => _,
        mask = out(reg) _,
        i = inout(reg) 43usize * 499 + 3 => _,
        ptr = in(reg) s.as_ptr(),
        chunk = out(ymm_reg) _,
        vec_ascii_hash = in(ymm_reg) u8x32::splat(b'#'),
        options(nostack),
    );

    let mut sums = i32x8::splat(0);

    asm!(
        "vmovdqa {cache1}, [rip + {keys}+0]",
        "vmovdqa {cache2}, [rip + {keys}+32]",
        "vmovdqa {cache3}, [rip + {keys}+64]",
        "vmovdqa {cache4}, [rip + {keys}+96]",
        "vmovdqa {cache5}, [rip + {keys}+128]",
        "vmovdqa {cache6}, [rip + {keys}+160]",
        "vmovdqa {cache7}, [rip + {keys}+192]",
        "vmovdqa {cache8}, [rip + {keys}+224]",
        "vmovdqa {cache9}, [rip + {keys}+256]",
        "vmovdqa {cache10}, [rip + {keys}+288]",
        "vmovdqa {cache11}, [rip + {keys}+320]",
        "vmovdqa {cache12}, [rip + {keys}+352]",
    "20:",
        "vpbroadcastd {lock}, [{locks} + {i}]",
        "vpand {tmp}, {lock}, {cache1}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache2}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache3}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache4}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache5}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache6}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache7}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache8}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache9}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache10}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache11}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, {cache12}",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+384]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+416]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+448]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+480]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+512]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+544]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+576]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+608]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+640]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+672]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+704]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+736]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+768]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+800]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+832]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+864]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+896]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+928]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+960]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "vpand {tmp}, {lock}, [rip + {keys}+992]",
        "vpcmpeqd {tmp}, {tmp}, {vzero}",
        "vpsubd {sums}, {sums}, {tmp}",
        "add {i:e}, -4",
        "jge 20b",
        keys = sym KEYS,
        locks = in(reg) locks,
        i = inout(reg) 996usize => _,
        vzero = in(ymm_reg) u32x8::splat(0),
        tmp = out(ymm_reg) _,
        lock = out(ymm_reg) _,
        sums = inout(ymm_reg) sums,
        cache1 = out(ymm_reg) _,
        cache2 = out(ymm_reg) _,
        cache3 = out(ymm_reg) _,
        cache4 = out(ymm_reg) _,
        cache5 = out(ymm_reg) _,
        cache6 = out(ymm_reg) _,
        cache7 = out(ymm_reg) _,
        cache8 = out(ymm_reg) _,
        cache9 = out(ymm_reg) _,
        cache10 = out(ymm_reg) _,
        cache11 = out(ymm_reg) _,
        cache12 = out(ymm_reg) _,
        options(nostack, readonly),
    );

    sums.reduce_sum() as u32
}

#[inline]
pub fn part1(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/25.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/25p1.txt").unwrap(),);
    }
}
