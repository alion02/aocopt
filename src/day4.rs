use super::*;

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    let end = r.end;
    let mut sums = i8x32::splat(0);
    loop {
        macro_rules! load {
            ($x:expr, $y:expr) => {
                (ptr.add($x).add($y * 141) as *const i8x32).read_unaligned()
            };
        }
        macro_rules! test_four {
            ($a:expr, $b:expr, $c:expr, $d:expr) => {
                let diff0 = $d - $a;
                let diff1 = $b - $c;
                let abs0 = diff0.abs();
                let abs1 = diff1.abs();
                let eq0 = abs0.simd_eq(Simd::splat(b'X' - b'S').cast());
                let eq1 = abs1.simd_eq(Simd::splat(b'M' - b'A').cast());
                let sign = diff0 ^ diff1;
                let eq = eq0 & eq1;
                let signs_match = sign.simd_lt(Simd::splat(0));
                sums -= (signs_match & eq).to_int();
            };
        }
        let v00 = load!(0, 0);
        let v10 = load!(1, 0);
        let v20 = load!(2, 0);
        let v30 = load!(3, 0);
        test_four!(v00, v10, v20, v30);
        let v21 = load!(2, 1);
        let v12 = load!(1, 2);
        let v03 = load!(0, 3);
        test_four!(v30, v21, v12, v03);
        let v01 = load!(0, 1);
        let v02 = load!(0, 2);
        test_four!(v00, v01, v02, v03);
        let v11 = load!(1, 1);
        let v22 = load!(2, 2);
        let v33 = load!(3, 3);
        test_four!(v00, v11, v22, v33);
        ptr = ptr.add(32);
        // yes we're reading hundreds of bytes past the end of the buffer. sue me
        if ptr >= end {
            break;
        }
    }
    let sums = _mm256_maddubs_epi16(sums.into(), u8x32::splat(1).into());
    let sums: u32x8 = _mm256_madd_epi16(sums, u16x16::splat(1).into()).into();
    sums.reduce_sum()
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
        let s = read_to_string("./inputs/4.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/4p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/4p2.txt").unwrap(),
        );
    }
}
