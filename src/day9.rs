use super::*;

unsafe fn inner1(s: &[u8]) -> usize {
    // struct State {
    //     input: *const u8,
    //     checksum: usize,
    //     disk_pos: usize,
    //     rem_empty: usize,
    //     rem_file: usize,
    //     left: usize,
    //     right: usize,
    // }

    // impl State {
    //     unsafe fn len(&self, i: usize) -> usize {
    //         self.input.add(i).read() as usize - b'0' as usize
    //     }

    //     unsafe fn nnef(&mut self) {
    //         self.left += 1;
    //         self.rem_empty += self.len(self.left);
    //         if self.rem_empty > 0 {
    //             self.ynff();
    //         } else {
    //             self.nnff();
    //         }
    //     }

    //     unsafe fn ynff(&mut self) {
    //         self.right -= 1;
    //         self.rem_file += self.len(self.right);
    //         if self.rem_empty > 0 {
    //             self.yyfe();
    //         } else {
    //             self.ynfe();
    //         }
    //     }

    //     unsafe fn nnff(&mut self) {
    //         self.left += 1;
    //         let mut file = self.len(self.left);
    //         while file > 0 {
    //             self.checksum += self.left / 2 * self.disk_pos;
    //             self.disk_pos += 1;
    //             file -= 1;
    //         }
    //         self.nnef();
    //     }

    //     unsafe fn yyfe(&mut self) {
    //         if self.rem_empty > self.rem_file {
    //             loop {
    //                 self.checksum += self.right / 2 * self.disk_pos;
    //                 self.disk_pos += 1;
    //                 self.rem_file -= 1;
    //                 if self.rem_file == 0 {
    //                     break;
    //                 }
    //             }
    //             self.right -= 1;
    //             self.ynff();
    //         } else {
    //             loop {
    //                 self.checksum += self.right / 2 * self.disk_pos;
    //                 self.disk_pos += 1;
    //                 self.rem_file -= 1;
    //                 if self.rem_empty == 0 {
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }

    macro_rules! len {
        ($i:expr) => {
            *s.get_unchecked($i) as usize - b'0' as usize
        };
    }

    let mut checksum = 0;
    let mut left = 0;
    let mut right = s.len();
    let mut disk_pos = 0;
    let mut rem_dst = 0;
    let mut rem_src = 0;
    'outer: loop {
        while rem_dst == 0 {
            if left == right {
                break 'outer;
            }
            let mut file = len!(left);
            while file > 0 {
                checksum += left / 2 * disk_pos;
                disk_pos += 1;
                file -= 1;
            }
            rem_dst = len!(left + 1);
            left += 2;
        }

        while rem_src == 0 {
            if left == right {
                break 'outer;
            }
            right -= 2;
            rem_src = len!(right);
        }

        while rem_dst > 0 && rem_src > 0 {
            checksum += right / 2 * disk_pos;
            disk_pos += 1;
            rem_dst -= 1;
            rem_src -= 1;
        }
    }

    while rem_src > 0 {
        checksum += right / 2 * disk_pos;
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
