use std::hint::assert_unchecked;

use super::*;

unsafe fn inner1(s: &[u8]) -> usize {
    macro_rules! len {
        ($i:expr) => {
            *s.get_unchecked($i) as usize - b'0' as usize
        };
    }

    let mut checksum = 0;
    let mut left = 0;
    let mut right = s.len() / 2;
    let mut disk_pos = 0;
    let mut rem_dst = 0;
    let mut rem_src = 0;
    'outer: loop {
        while rem_dst == 0 {
            if left == right {
                break 'outer;
            }
            let mut file = len!(left * 2);
            assert_unchecked(file > 0);
            loop {
                checksum += left * disk_pos;
                disk_pos += 1;
                file -= 1;
                if file == 0 {
                    break;
                }
            }
            rem_dst = len!(left * 2 + 1);
            left += 1;
        }

        if rem_src == 0 {
            if left == right {
                break 'outer;
            }
            right -= 1;
            rem_src = len!(right * 2);
        }

        assert_unchecked(rem_src > 0);
        while rem_dst > 0 && rem_src > 0 {
            checksum += right * disk_pos;
            disk_pos += 1;
            rem_dst -= 1;
            rem_src -= 1;
        }
    }

    while rem_src > 0 {
        checksum += right * disk_pos;
        disk_pos += 1;
        rem_src -= 1;
    }

    checksum
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

unsafe fn inner2(s: &[u8]) -> u64 {
    0
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
        let s = read_to_string("./inputs/9.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/9p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/9p2.txt").unwrap(),
        );
    }
}
