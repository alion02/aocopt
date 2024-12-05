use std::arch::{asm, x86_64::_mm_testc_si128};

use super::*;

// 5-23 numbers in list
// all numbers 2 digit

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &[u8]) -> u32 {
    let mut matrix = [u8x16::splat(0); 90];
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    loop {
        let chunk = (ptr as *const u8x16).read_unaligned();
        if _mm_testc_si128(
            chunk.into(),
            u8x16::from_array([16, 0, 0, 16, 0, 0, 16, 0, 0, 16, 0, 0, 0, 0, 0, 0]).into(),
        ) == 0
        {
            break;
        }
        let normalized = chunk
            - Simd::from_array([
                b'1', b'0', 0, b'1', b'0', 0, b'1', b'0', 0, b'1', b'0', 0, 0, 0, 0, 0,
            ]);
        let shuffled = _mm_shuffle_epi8(
            normalized.into(),
            i8x16::from_array([0, 1, -1, -1, 3, 4, -1, -1, 6, 7, -1, -1, 9, 10, -1, -1]).into(),
        );
        let indices: u32x4 = _mm_maddubs_epi16(
            u8x16::from_array([160, 16, 0, 0, 10, 1, 0, 0, 160, 16, 0, 0, 10, 1, 0, 0]).into(),
            shuffled,
        )
        .into();
        asm!(
            "bts dword ptr[{base} + {i0:r}], {i1:e}",
            "bts dword ptr[{base} + {i2:r}], {i3:e}",
            base = in(reg) matrix.as_mut_ptr(),
            i0 = in(reg) indices[0],
            i1 = in(reg) indices[1],
            i2 = in(reg) indices[2],
            i3 = in(reg) indices[3],
            options(nostack),
        );
        // *matrix
        //     .as_mut_ptr()
        //     .cast::<u8>()
        //     .add(indices[0] as usize)
        //     .add((indices[1] / 32) as usize * 4)
        //     .cast::<u32>() |= 1u32.wrapping_shl(indices[1]);
        // *matrix
        //     .as_mut_ptr()
        //     .cast::<u8>()
        //     .add(indices[2] as usize)
        //     .add((indices[3] / 32) as usize * 4)
        //     .cast::<u32>() |= 1u32.wrapping_shl(indices[3]);
        ptr = ptr.add(12);
    }

    matrix.into_iter().sum::<u8x16>().reduce_sum() as u32
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
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
    fn test() {
        let s = read_to_string("./inputs/5.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/5p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/5p2.txt").unwrap(),
        );
    }
}
