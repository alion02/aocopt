use super::*;

unsafe fn inner1(s: &[u8]) -> usize {
    let mut checksum = 0;
    let mut left = 0;
    let mut right = s.len() / 2;
    let mut disk_pos = 0;
    let mut rem_dst = 0;
    let mut rem_src = 0;

    macro_rules! len {
        ($i:expr) => {
            *s.get_unchecked($i) as usize - b'0' as usize
        };
    }

    macro_rules! insert_file {
        ($len:expr, $id:expr) => {
            let len = $len;
            checksum += (disk_pos * 2 + len - 1) * $id * len;
            disk_pos += len;
        };
    }

    'outer: loop {
        while rem_dst == 0 {
            if left == right {
                break 'outer;
            }

            insert_file!(len!(left * 2), left);
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

        let len = rem_dst.min(rem_src);
        insert_file!(len, right);
        rem_dst -= len;
        rem_src -= len;
    }

    insert_file!(rem_src, left);

    checksum / 2
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
