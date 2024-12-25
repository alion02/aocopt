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
    for i in 0..250 {
        for j in 0..32 {
            sums += (Simd::splat(*locks.add(i)) & *keys.add(j))
                .simd_eq(Simd::splat(0))
                .to_int();
        }
    }

    -sums.reduce_sum() as u32
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
