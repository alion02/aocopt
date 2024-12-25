use super::*;

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u32 {
    0
}

#[inline]
pub fn part1(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/25.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/25p1.txt").unwrap(),);
    }
}
