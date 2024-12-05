use std::arch::{asm, x86_64::_mm_testc_si128};

use super::*;

// 5-23 numbers in list
// all numbers 2 digit

static mut MATRIX: [u8x32; 45] = [u8x32::from_array([0; 32]); 45];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &[u8]) -> u32 {
    let matrix = &mut MATRIX;
    {
        MATRIX[0] = Simd::splat(0);
        MATRIX[1] = Simd::splat(0);
        MATRIX[2] = Simd::splat(0);
        MATRIX[3] = Simd::splat(0);
        MATRIX[4] = Simd::splat(0);
        MATRIX[5] = Simd::splat(0);
        MATRIX[6] = Simd::splat(0);
        MATRIX[7] = Simd::splat(0);
        MATRIX[8] = Simd::splat(0);
        MATRIX[9] = Simd::splat(0);
        MATRIX[10] = Simd::splat(0);
        MATRIX[11] = Simd::splat(0);
        MATRIX[12] = Simd::splat(0);
        MATRIX[13] = Simd::splat(0);
        MATRIX[14] = Simd::splat(0);
        MATRIX[15] = Simd::splat(0);
        MATRIX[16] = Simd::splat(0);
        MATRIX[17] = Simd::splat(0);
        MATRIX[18] = Simd::splat(0);
        MATRIX[19] = Simd::splat(0);
        MATRIX[20] = Simd::splat(0);
        MATRIX[21] = Simd::splat(0);
        MATRIX[22] = Simd::splat(0);
        MATRIX[23] = Simd::splat(0);
        MATRIX[24] = Simd::splat(0);
        MATRIX[25] = Simd::splat(0);
        MATRIX[26] = Simd::splat(0);
        MATRIX[27] = Simd::splat(0);
        MATRIX[28] = Simd::splat(0);
        MATRIX[29] = Simd::splat(0);
        MATRIX[30] = Simd::splat(0);
        MATRIX[31] = Simd::splat(0);
        MATRIX[32] = Simd::splat(0);
        MATRIX[33] = Simd::splat(0);
        MATRIX[34] = Simd::splat(0);
        MATRIX[35] = Simd::splat(0);
        MATRIX[36] = Simd::splat(0);
        MATRIX[37] = Simd::splat(0);
        MATRIX[38] = Simd::splat(0);
        MATRIX[39] = Simd::splat(0);
        MATRIX[40] = Simd::splat(0);
        MATRIX[41] = Simd::splat(0);
        MATRIX[42] = Simd::splat(0);
        MATRIX[43] = Simd::splat(0);
        MATRIX[44] = Simd::splat(0);
    }

    let matrix: &mut [u8x16; 90] = transmute(matrix);
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    macro_rules! fast_bt {
        ($bt:literal, $i0:expr, $i1:expr) => {
            asm!(
                concat!($bt, " dword ptr[{base} + {i0:r}], {i1:e}"),
                base = in(reg) matrix,
                i0 = in(reg) $i0,
                i1 = in(reg) $i1,
                options(nostack),
            );
        };
    }
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
        fast_bt!("bts", indices[0], indices[1]);
        fast_bt!("bts", indices[2], indices[3]);
        ptr = ptr.add(12);
    }
    matrix.iter().copied().sum::<u8x16>().reduce_sum() as u32
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
