use std::collections::VecDeque;

use super::*;

macro_rules! row_len {
    () => {
        142
    };
}

macro_rules! side_len {
    () => {
        row_len!() - 1
    };
}

macro_rules! far_edge {
    () => {
        row_len!() - 3
    };
}

#[inline]
unsafe fn inner1(s: &[u8]) -> u32 {
    #[derive(Clone, Copy)]
    struct Node {
        pos: u16,
        dir: u8,
        cost: u8,
    }

    let mut curr = VecDeque::<Node>::from_iter([Node {
        pos: far_edge!() * row_len!() + 1,
        dir: 0,
        cost: 0,
    }]);
    let mut next = VecDeque::<Node>::new();

    let mut front = VecDeque::<Node>::new();

    let mut turn_cost = 0;

    let mut visited = [0u8; row_len!() * side_len!()];

    loop {
        // let mut map = s.to_vec();
        // for node in &front {
        //     map[node.pos as usize] = b">v<^"[node.dir as usize];
        // }
        // for node in &curr {
        //     map[node.pos as usize] = b'*';
        // }
        // for node in &next {
        //     map[node.pos as usize] = b'+';
        // }
        // for y in 0..side_len!() {
        //     println!(
        //         "{}",
        //         std::str::from_utf8(&map[y * row_len!()..y * row_len!() + side_len!()]).unwrap()
        //     );
        // }

        if front.is_empty() {
            if curr.is_empty() {
                (curr, next) = (next, curr);
                turn_cost += 1000;
            }

            front.push_back(curr.pop_front().unwrap_unchecked());
        }

        while let Some(node) = curr.front() {
            if node.cost != front.front().unwrap_unchecked().cost {
                break;
            }
            front.push_back(curr.pop_front().unwrap_unchecked());
        }

        let mut found_end = false;
        front.retain_mut(
            |&mut Node {
                 pos: ref mut pos_ref,
                 dir,
                 cost: ref mut cost_ref,
             }| {
                if found_end {
                    return false;
                }

                let pos = *pos_ref;
                let cost = *cost_ref + 1;

                if pos == row_len!() + far_edge!() {
                    found_end = true;
                    return true;
                }

                macro_rules! offset {
                    ($dir:expr) => {
                        *[1i16, row_len!(), -1, -row_len!()].get_unchecked($dir as usize)
                    };
                }

                let off = offset!(dir);
                let mut npos = pos.wrapping_add_signed(off);
                let retain = if *s.get_unchecked(npos as usize) != b'#' && {
                    npos = npos.wrapping_add_signed(off);
                    *visited.get_unchecked(npos as usize) & 5 << (dir & 1) == 0
                } {
                    *visited.get_unchecked_mut(npos as usize) |= 1 << dir;
                    *cost_ref = cost;
                    *pos_ref = npos;
                    true
                } else {
                    false
                };

                let dir = dir ^ 1;
                let off = offset!(dir);
                let mut npos = pos.wrapping_add_signed(off);
                if *s.get_unchecked(npos as usize) != b'#' && {
                    npos = npos.wrapping_add_signed(off);
                    *visited.get_unchecked(npos as usize) & 5 << (dir & 1) == 0
                } {
                    *visited.get_unchecked_mut(npos as usize) |= 1 << dir;
                    next.push_back(Node { pos: npos, dir, cost });
                }

                let dir = dir ^ 2;
                let off = offset!(dir);
                let mut npos = pos.wrapping_add_signed(off);
                if *s.get_unchecked(npos as usize) != b'#' && {
                    npos = npos.wrapping_add_signed(off);
                    *visited.get_unchecked(npos as usize) & 5 << (dir & 1) == 0
                } {
                    *visited.get_unchecked_mut(npos as usize) |= 1 << dir;
                    next.push_back(Node { pos: npos, dir, cost });
                }

                retain
            },
        );

        if found_end {
            return turn_cost + front.back().unwrap_unchecked().cost as u32 * 2;
        }
    }
}

#[inline]
unsafe fn inner2(s: &[u8]) -> u32 {
    0
}

#[inline]
pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/16.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/16p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/16.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/16p2.txt").unwrap(),);
    }
}
