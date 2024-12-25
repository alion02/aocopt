use super::*;

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u32 {
    static mut LOCKS: [u32; 250] = [0; 250];
    static mut KEYS: [u32x8; 32] = [Simd::from_array([!0; 8]); 32];

    let locks = LOCKS.as_mut_ptr();
    let keys = KEYS.as_mut_ptr();

    {
        let ptr = s.as_ptr();
        let mut locks = locks;
        let mut keys = keys.cast::<u32>();
        for i in 0..500 {
            let chunk = ptr.add(i * 43 + 3).cast::<u8x32>().read_unaligned();
            let chunk = chunk.simd_eq(Simd::splat(b'#'));
            let mask = chunk.to_bitmask() as u32;
            if mask & 1 == 1 {
                *keys = mask;
                keys = keys.add(1);
            } else {
                *locks = mask;
                locks = locks.add(1);
            }
        }
    }

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
