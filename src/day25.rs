use super::*;

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u32 {
    static mut LOCKS: [u32; 250] = [0; 250];
    static mut KEYS: [u32x8; 32] = [u32x8::from_array([!0; 8]); 32];
    static mut KEY_BUCKETS: [[u32; 250]; 6] = [[0; 250]; 6];

    let locks = LOCKS.as_mut_ptr();
    let keys = KEYS.as_mut_ptr();

    let mut end_indices = [0; 6];

    const FIRST_COL_MASK: u32 = 0b1000001000001000001000001000;

    {
        let ptr = s.as_ptr();
        let mut locks = locks;
        let buckets = &mut KEY_BUCKETS;
        let mut indices = [0; 6];
        for i in 0..500 {
            let chunk = ptr.add(i * 43 + 3).cast::<u8x32>().read_unaligned();
            let chunk = chunk.simd_eq(Simd::splat(b'#'));
            let mask = chunk.to_bitmask() as u32;
            if mask & 1 == 1 {
                let bucket = (mask & FIRST_COL_MASK).count_ones() as usize;
                let idx = indices.get_unchecked_mut(bucket);
                *buckets.get_unchecked_mut(bucket).get_unchecked_mut(*idx) = mask;
                *idx += 1;
            } else {
                *locks = mask;
                locks = locks.add(1);
            }
        }
        let mut keys = keys.cast::<u32>();
        let mut end_idx = 0;
        for i in 0..6 {
            end_idx += indices[i];
            end_indices[5 - i] = end_idx;
            for j in 0..indices[i] {
                *keys = *buckets[i].get_unchecked(j);
                keys = keys.add(1);
            }
        }
    }

    let mut sums = i32x8::splat(0);

    for i in (0..250).rev() {
        let mask = *locks.add(i);
        let height = (mask & FIRST_COL_MASK).count_ones() as usize;
        let end = *end_indices.get_unchecked(height);
        asm!(
        "20:",
            "vpand {chunk}, {vmask}, [{keys} + {j} * 4]",
            "vpcmpeqd {chunk}, {chunk}, {vzero}",
            "vpaddd {sums}, {sums}, {chunk}",
            "add {j}, 8",
            "cmp {j}, {end}",
            "jl 20b",
            keys = in(reg) keys,
            j = inout(reg) 0usize => _,
            sums = inout(ymm_reg) sums,
            vmask = in(ymm_reg) u32x8::splat(mask),
            vzero = in(ymm_reg) u32x8::splat(0),
            chunk = out(ymm_reg) _,
            end = in(reg) end,
            options(readonly, nostack),
        );
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
