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

        if !P2 {
            #[inline(never)]
            unsafe fn f(list: *mut u16, list_end: *mut u16, target: u64, mut total: u64) -> u64 {
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
                    list = in(reg) list_end,
                    i = inout(reg) list.byte_offset_from(list_end) => _,
                    curr = out(reg) _,
                    target = inout(reg) target => _,
                    total = inout(reg) total,
                    orig_target = in(reg) target,
                    rsp = out(reg) _,
                );
                total
            }
            total = f(list, list_end, target, total);
        } else {
            #[inline(never)]
            unsafe fn f(list: *mut u16, list_end: *mut u16, target: u64, mut total: u64) -> u64 {
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
                    list = in(reg) list_end,
                    i = inout(reg) list.byte_offset_from(list_end) => _,
                    curr = out(reg) _,
                    target = inout(reg) target => _,
                    table = in(reg) &LOG_TABLE,
                    total = inout(reg) total,
                    orig_target = in(reg) target,
                    rsp = out(reg) _,
                );
                total
            }
            total = f(list, list_end, target, total);
        }

        if ptr == r.end {
            break;
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
