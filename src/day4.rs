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
                (ptr.add($x).add($y * 141) as *const u8x32).read_unaligned()
            };
        }
        macro_rules! is {
            ($v:expr, $c:expr) => {
                ($v).simd_eq(Simd::splat($c))
            };
        }
        let v00 = load!(0, 0);
        let v10 = load!(1, 0);
        let v20 = load!(2, 0);
        let v30 = load!(3, 0);
        let v00x = is!(v00, b'X');
        let v10m = is!(v10, b'M');
        let v20a = is!(v20, b'A');
        let v30s = is!(v30, b'S');
        sums -= (v00x & v10m & v20a & v30s).to_int();
        let v00s = is!(v00, b'S');
        let v10a = is!(v10, b'A');
        let v20m = is!(v20, b'M');
        let v30x = is!(v30, b'X');
        sums -= (v00s & v10a & v20m & v30x).to_int();
        let v21 = load!(2, 1);
        let v12 = load!(1, 2);
        let v03 = load!(0, 3);
        let v21m = is!(v21, b'M');
        let v12a = is!(v12, b'A');
        let v03s = is!(v03, b'S');
        sums -= (v30x & v21m & v12a & v03s).to_int();
        let v21a = is!(v21, b'A');
        let v12m = is!(v12, b'M');
        let v03x = is!(v03, b'X');
        sums -= (v30s & v21a & v12m & v03x).to_int();
        let v01 = load!(0, 1);
        let v02 = load!(0, 2);
        let v01m = is!(v01, b'M');
        let v02a = is!(v02, b'A');
        sums -= (v00x & v01m & v02a & v03s).to_int();
        let v01a = is!(v01, b'A');
        let v02m = is!(v02, b'M');
        sums -= (v00s & v01a & v02m & v03x).to_int();
        let v11 = load!(1, 1);
        let v22 = load!(2, 2);
        let v33 = load!(3, 3);
        let v11m = is!(v11, b'M');
        let v22a = is!(v22, b'A');
        let v33s = is!(v33, b'S');
        sums -= (v00x & v11m & v22a & v33s).to_int();
        let v11a = is!(v11, b'A');
        let v22m = is!(v22, b'M');
        let v33x = is!(v33, b'X');
        sums -= (v00s & v11a & v22m & v33x).to_int();
        ptr = ptr.add(32);
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
