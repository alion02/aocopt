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
    static mut CURR: [Node; 4096] = [unsafe { transmute(0) }; 4096];
    static mut NEXT: [Node; 4096] = [unsafe { transmute(0) }; 4096];
    static OFFSET: [i16; 4] = [1, row_len!(), -1, -row_len!()];

    let mut visited = [0u8; row_len!() * side_len!()];
    let mut curr = &mut CURR;
    let mut next = &mut NEXT;
    let offset = &OFFSET;

    #[derive(Clone, Copy)]
    #[repr(align(4))]
    struct Node {
        pos: u16,
        dir: u8,
        cost: u8,
    }

    curr[0] = Node {
        pos: far_edge!() * row_len!() + 1,
        dir: 0,
        cost: 0,
    };
    curr[1].cost = !0;

    let mut turn_cost = 0;
    loop {
        let mut i = 0;
        let mut k = 0;
        loop {
            let mut j = i;
            let cost = curr.get_unchecked_mut(j).cost;
            let next_cost = cost + 1;
            loop {
                let node = curr.get_unchecked_mut(j);
                let mut pos = node.pos;
                assert!(*s.get_unchecked(pos as usize) != b'#');
                if pos == row_len!() + far_edge!() {
                    return turn_cost + cost as u32 * 2;
                }
                let mut dir = node.dir;
                let visit_mask = 1 << (dir & 1);
                'delete: {
                    if *visited.get_unchecked(pos as usize) & visit_mask == 0 {
                        *visited.get_unchecked_mut(pos as usize) |= visit_mask;
                        dir ^= 1;
                        {
                            let pos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                            if *s.get_unchecked(pos as usize) != b'#' {
                                *next.get_unchecked_mut(k) = Node {
                                    pos: pos.wrapping_add_signed(*offset.get_unchecked(dir as usize)),
                                    dir,
                                    cost: next_cost,
                                };
                                k += 1;
                            }
                        }
                        dir ^= 2;
                        {
                            let pos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                            if *s.get_unchecked(pos as usize) != b'#' {
                                *next.get_unchecked_mut(k) = Node {
                                    pos: pos.wrapping_add_signed(*offset.get_unchecked(dir as usize)),
                                    dir,
                                    cost: next_cost,
                                };
                                k += 1;
                            }
                        }
                        dir ^= 3;
                        pos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                        if *s.get_unchecked(pos as usize) != b'#' {
                            node.pos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                            node.cost = next_cost;
                            break 'delete;
                        }
                    }
                    *curr.get_unchecked_mut(j) = *curr.get_unchecked(i);
                    i += 1;
                }

                j += 1;
                if curr.get_unchecked(j).cost > cost {
                    break;
                }
            }

            if curr.get_unchecked(i).cost == !0 {
                break;
            }
        }

        turn_cost += 1000;
        (curr, next) = (next, curr);
        curr.get_unchecked_mut(k).cost = !0;
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
