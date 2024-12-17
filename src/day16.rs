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
    static mut CURR: [Node; 4096] = [unsafe { transmute(0) }; 4096];
    static mut NEXT: [Node; 4096] = [unsafe { transmute(0) }; 4096];
    static OFFSET: [i16; 4] = [1, row_len!(), -1, -row_len!()];

    let mut visited = [0u8; row_len!() * side_len!()];
    let mut origins = [Origin {
        dirpos1: !0,
        dirpos2: !0,
        dirpos3: !0,
        total_cost: !0,
    }; row_len!() * side_len!() * 4];
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

    #[derive(Clone, Copy)]
    #[repr(align(8))]
    struct Origin {
        dirpos1: u32,
        dirpos2: u32,
        dirpos3: u32,
        total_cost: u32,
    }

    curr[0] = Node {
        pos: far_edge!() * row_len!() + 1,
        dir: 0,
        cost: 0,
    };
    curr[1].cost = !0;

    let mut turn_cost = 0;
    let mut final_cost = 0;
    'outer: loop {
        let mut i = 0;
        let mut k = 0;
        loop {
            let mut j = i;
            let cost = curr.get_unchecked_mut(j).cost;
            let next_cost = cost + 1;
            loop {
                let node = curr.get_unchecked_mut(j);
                let pos = node.pos;
                assert!(*s.get_unchecked(pos as usize) != b'#');
                if pos == row_len!() + far_edge!() {
                    final_cost = turn_cost + cost as u32 * 2;
                }
                let mut dir = node.dir;
                let dirpos = pos as u32 * 4 + dir as u32;
                let visit_mask = 1 << (dir & 1);
                'delete: {
                    if *visited.get_unchecked(pos as usize) & visit_mask == 0 {
                        *visited.get_unchecked_mut(pos as usize) |= visit_mask;
                        dir ^= 1;
                        {
                            let mut npos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                            if *s.get_unchecked(npos as usize) != b'#' {
                                npos = npos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                                let origin = origins.get_unchecked_mut(npos as usize * 4 + dir as usize);
                                let total_cost = turn_cost + next_cost as u32 * 2 + 1000;
                                if origin.total_cost >= total_cost {
                                    origin.total_cost = total_cost;
                                    if origin.dirpos1 == !0 {
                                        origin.dirpos1 = dirpos;
                                    } else if origin.dirpos2 == !0 {
                                        origin.dirpos2 = dirpos;
                                    } else {
                                        origin.dirpos3 = dirpos;
                                    }
                                }
                                *next.get_unchecked_mut(k) = Node {
                                    pos: npos,
                                    dir,
                                    cost: next_cost,
                                };
                                k += 1;
                            }
                        }
                        dir ^= 2;
                        {
                            let mut npos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                            if *s.get_unchecked(npos as usize) != b'#' {
                                npos = npos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                                let origin = origins.get_unchecked_mut(npos as usize * 4 + dir as usize);
                                let total_cost = turn_cost + next_cost as u32 * 2 + 1000;
                                if origin.total_cost >= total_cost {
                                    origin.total_cost = total_cost;
                                    if origin.dirpos1 == !0 {
                                        origin.dirpos1 = dirpos;
                                    } else if origin.dirpos2 == !0 {
                                        origin.dirpos2 = dirpos;
                                    } else {
                                        origin.dirpos3 = dirpos;
                                    }
                                }
                                *next.get_unchecked_mut(k) = Node {
                                    pos: npos,
                                    dir,
                                    cost: next_cost,
                                };
                                k += 1;
                            }
                        }
                        dir ^= 3;
                        let mut npos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                        if *s.get_unchecked(npos as usize) != b'#' {
                            npos = npos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                            let origin = origins.get_unchecked_mut(npos as usize * 4 + dir as usize);
                            let total_cost = turn_cost + next_cost as u32 * 2;
                            if origin.total_cost >= total_cost {
                                origin.total_cost = total_cost;
                                if origin.dirpos1 == !0 {
                                    origin.dirpos1 = dirpos;
                                } else if origin.dirpos2 == !0 {
                                    origin.dirpos2 = dirpos;
                                } else {
                                    origin.dirpos3 = dirpos;
                                }
                            }
                            node.pos = npos;
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

            if final_cost > 0 {
                break 'outer;
            }

            if curr.get_unchecked(i).cost == !0 {
                break;
            }
        }

        turn_cost += 1000;
        (curr, next) = (next, curr);
        curr.get_unchecked_mut(k).cost = !0;
    }

    let mut res = 0;

    for dirpos in [(row_len!() + far_edge!()) * 4, (row_len!() + far_edge!()) * 4 + 3] {
        let origin = origins.get_unchecked_mut(dirpos);
        if origin.total_cost == final_cost {
            unsafe fn mark_path(origins: &mut [Origin; row_len!() * side_len!() * 4], dirpos: usize) -> i32 {
                let origin = *origins.get_unchecked(dirpos);
                let mut total = 0;
                if origin.dirpos1 < !0 {
                    total += mark_path(origins, origin.dirpos1 as _) + 2;
                }
                if origin.dirpos2 < !0 {
                    total += mark_path(origins, origin.dirpos2 as _) - 1;
                }
                if origin.dirpos3 < !0 {
                    total += mark_path(origins, origin.dirpos3 as _) - 1;
                }
                *origins.get_unchecked_mut(dirpos) = Origin {
                    dirpos1: !0,
                    dirpos2: !0,
                    dirpos3: !0,
                    total_cost: !0,
                };
                total
            }
            res += mark_path(&mut origins, dirpos);
        }
    }

    res as u32 + 1
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
