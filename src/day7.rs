use std::num::NonZeroU64;

use super::*;

// list len 3-12
// list number range 1-999
// len < 5 uncommon
// target digit count range 2?-15?

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn process<const P2: bool>(s: &[u8]) -> u64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn f<const P2: bool>(target: u64, list: *mut u16, list_end: *mut u16) -> bool {
        let curr = *list as u64;
        if list == list_end {
            return target == curr;
        }
        let next = list.sub(1);
        let nzcurr = NonZeroU64::new_unchecked(curr);
        target % nzcurr == 0 && f::<P2>(target / nzcurr, next, list_end)
            || P2 && {
                let divisor = NonZeroU64::new_unchecked(10u32.pow(nzcurr.ilog10() + 1) as u64);
                target % divisor == curr && f::<P2>(target / divisor, next, list_end)
            }
            || target > curr && f::<P2>(target - curr, next, list_end)
    }

    let r = s.as_ptr_range();
    let mut ptr = r.start;
    let mut total = 0;
    let mut numbers = [MaybeUninit::<u16>::uninit(); 12];
    loop {
        let mut target = *ptr as u64 - b'0' as u64;
        ptr = ptr.add(1);
        loop {
            let b = *ptr;
            if b == b':' {
                ptr = ptr.add(2);
                break;
            }
            ptr = ptr.add(1);
            target *= 10;
            target += b as u64 - b'0' as u64;
        }

        let list_end = numbers.as_mut_ptr().cast::<u16>();
        let mut list = list_end;
        'outer: loop {
            let mut num = *ptr as i32 - b'0' as i32;
            ptr = ptr.add(1);
            loop {
                let b = *ptr as i32 - b'0' as i32;
                ptr = ptr.add(1);
                if b < 0 {
                    list.write(num as u16);
                    if b == b'\n' as i32 - b'0' as i32 {
                        break 'outer;
                    }
                    list = list.add(1);
                    break;
                }
                num *= 10;
                num += b;
            }
        }

        if f::<P2>(target, list, list_end) {
            total += target;
        }

        if ptr == r.end {
            break;
        }

        // asm!(
        // "20:",
        //     "movzx cx, [{list}]",
        //     "mov rbx, rax",
        //     "div rcx",
        //     "test rdx, rdx",
        //     "jnz 21f",
        //     "",
        //     "/*{list} {end}*/",
        //     inout("rax") target => _,
        //     out("rdx") _,
        //     out("rcx") _,
        //     out("rbx") _,
        //     list = inout(reg) list => _,
        //     end = in(reg) list_end,
        // );
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
