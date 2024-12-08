use super::*;

// list len 3-12
// list number range 1-999
// len < 5 uncommon
// target digit count range 2?-15?

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn process<const P2: bool>(s: &[u8]) -> u64 {
    static LOG_TABLE: [u64; 1000] = {
        let mut table = [0; 1000];
        let mut i = 1;
        while i < 1000 {
            table[i] = 10u64.pow(i.ilog10() + 1);
            i += 1;
        }
        table
    };

    static LUTS: ([u8x16; 65536], [(u8, u8); 65536]) = unsafe {
        let mut shuffle = [[-1i8; 16]; 65536];
        let mut adj = [(0u8, 0u8); 65536];

        let mut i = 0u16;
        loop {
            let shuf = &mut shuffle[i as usize];
            let (step, count) = &mut adj[i as usize];

            let mut mask = i;

            let mut vec_idx = 16;
            let mut dest_idx = 16;
            let mut digits;
            loop {
                digits = mask.leading_zeros();
                if dest_idx == 0 || digits == 0 || digits > 3 {
                    break;
                }
                let mut to_insert = digits;
                while to_insert > 0 {
                    dest_idx -= 1;
                    vec_idx -= 1;
                    to_insert -= 1;
                    shuf[dest_idx] = vec_idx;
                }
                *count += 1;
                dest_idx &= !3;
                vec_idx -= 1;
                mask <<= digits + 1;
            }
            *step = (16 - vec_idx) as u8 + (digits == 0) as u8 * 16;

            if let Some(ni) = i.checked_add(1) {
                i = ni;
            } else {
                break;
            }
        }

        (transmute(shuffle), adj)
    };

    static SLIDE: [i8; 33] = {
        let mut slide = [0; 33];
        let mut i = 16;
        while i < 33 {
            slide[i] = -1;
            i += 1;
        }
        slide
    };

    const MAX_LEFT_STEP: usize = 80;
    const RIGHT_TAIL: usize = 16;

    static mut REMAINDER: [u8; MAX_LEFT_STEP * 2 + RIGHT_TAIL] =
        [0; MAX_LEFT_STEP * 2 + RIGHT_TAIL];

    let r = s.as_ptr_range();
    let mut safe_end = r.start.add(MAX_LEFT_STEP);
    let mut ptr = r.end.sub(17);
    let mut total = 0;
    let mut numbers = [MaybeUninit::<u64>::uninit(); 3];
    let (shuffle, adj) = &LUTS;
    let slide = SLIDE.as_ptr();
    let remainder = REMAINDER.as_mut_ptr().add(MAX_LEFT_STEP);
    let mut finishing = false;

    loop {
        let mut num_count = 0;
        let mut bundle = 2;
        loop {
            let chunk = ptr.cast::<u8x16>().read_unaligned();
            let chunk = chunk - Simd::splat(b'0');
            let spaces = chunk.simd_gt(Simd::splat(9));
            let mask = spaces.to_bitmask() as usize;
            let shuffle = *shuffle.get_unchecked(mask);
            let (step, count) = *adj.get_unchecked(mask);
            let chunk = _mm_shuffle_epi8(chunk.into(), shuffle.into());
            let chunk = _mm_maddubs_epi16(
                chunk,
                u8x16::from_array([0, 100, 10, 1, 0, 100, 10, 1, 0, 100, 10, 1, 0, 100, 10, 1])
                    .into(),
            );
            let chunk: u64x2 = _mm_hadd_epi16(chunk, chunk).into();
            numbers.get_unchecked_mut(bundle).write(chunk[0]);
            ptr = ptr.sub(step as usize);
            num_count += count as usize;
            if step > 16 {
                ptr = ptr.add(15);
                break;
            }
            bundle = bundle.wrapping_sub(1);
        }

        let chunk = ptr.cast::<u8x16>().read_unaligned();
        let chunk = chunk - Simd::splat(b'0');
        let mask = _mm_movemask_epi8(chunk.into()) as u16;
        let digits = mask.leading_zeros();
        let digit_mask = slide.add(digits as usize).cast::<u8x16>().read_unaligned();
        let chunk = chunk & digit_mask;
        let chunk = _mm_maddubs_epi16(
            chunk.into(),
            u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]).into(),
        );
        let chunk = _mm_madd_epi16(
            chunk,
            u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]).into(),
        );
        let chunk: u32x4 = _mm_madd_epi16(
            _mm_packus_epi32(chunk, chunk),
            u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]).into(),
        )
        .into();
        let target = chunk[0] as u64 * 100000000 + chunk[1] as u64;

        ptr = ptr.sub(digits as usize + 1);
        let rightmost = numbers.as_ptr().cast::<u16>().add(11);
        let leftmost = rightmost.sub(num_count - 1);

        if !P2 {
            asm!(
                "mov {rsp}, rsp",
                "call 23f",
                "jmp 25f",
            "23:",
                "movzx {curr:e}, word ptr[{list} + {i}]",
                "add {i}, -2",
                "jl 20f",
                "mov rax, {target}",
                "xor edx, edx",
                "div {curr}",
                "test rdx, rdx",
                "jnz 22f",
                "push {target}",
                "mov {target}, rax",
                "call 23b",
                "pop {target}",
                "movzx {curr:e}, word ptr[{list} + {i} + 2]",
            "22:",
                "sub {target}, {curr}",
                "jbe 24f",
                "call 23b",
            "24:",
                "add {i}, 2",
                "ret",
            "20:",
                "cmp {target}, {curr}",
                "jne 24b",
            "21:",
                "add {total}, {orig_target}",
                "mov rsp, {rsp}",
            "25:",
                out("rax") _,
                out("rdx") _,
                list = in(reg) leftmost,
                i = inout(reg) rightmost.byte_offset_from(leftmost) => _,
                curr = out(reg) _,
                target = inout(reg) target => _,
                total = inout(reg) total,
                orig_target = in(reg) target,
                rsp = out(reg) _,
            );
        } else {
            asm!(
                "mov {rsp}, rsp",
                "call 23f",
                "jmp 25f",
            "23:",
                "movzx {curr:e}, word ptr[{list} + {i}]",
                "add {i}, -2",
                "jl 20f",
                "mov rax, {target}",
                "xor edx, edx",
                "div {curr}",
                "test rdx, rdx",
                "jnz 22f",
                "push {target}",
                "mov {target}, rax",
                "call 23b",
                "pop {target}",
                "movzx {curr:e}, word ptr[{list} + {i} + 2]",
            "22:",
                "mov rax, {target}",
                "xor edx, edx",
                "div qword ptr[{table} + {curr} * 8]",
                "cmp rdx, {curr}",
                "jne 26f",
                "push {target}",
                "mov {target}, rax",
                "call 23b",
                "pop {target}",
                "movzx {curr:e}, word ptr[{list} + {i} + 2]",
            "26:",
                "sub {target}, {curr}",
                "jbe 24f",
                "call 23b",
            "24:",
                "add {i}, 2",
                "ret",
            "20:",
                "cmp {target}, {curr}",
                "jne 24b",
            "21:",
                "add {total}, {orig_target}",
                "mov rsp, {rsp}",
            "25:",
                out("rax") _,
                out("rdx") _,
                list = in(reg) leftmost,
                i = inout(reg) rightmost.byte_offset_from(leftmost) => _,
                curr = out(reg) _,
                target = inout(reg) target => _,
                table = in(reg) &LOG_TABLE,
                total = inout(reg) total,
                orig_target = in(reg) target,
                rsp = out(reg) _,
            );
        }

        if ptr < safe_end {
            if finishing {
                break;
            }
            finishing = true;
            remainder.copy_from_nonoverlapping(r.start, MAX_LEFT_STEP + RIGHT_TAIL);
            ptr = remainder.offset(ptr.offset_from(r.start));
            safe_end = remainder.sub(RIGHT_TAIL);
        }
    }

    total
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[allow(unreachable_code)]
unsafe fn inner1(s: &[u8]) -> u64 {
    process::<false>(s)
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &[u8]) -> u64 {
    process::<true>(s)
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
        let s = read_to_string("./inputs/7.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/7p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/7p2.txt").unwrap(),
        );
    }
}
