use std::hint::{black_box, unreachable_unchecked};

use super::*;

static mut SCRATCH: [u8x32; 67] = [u8x32::from_array([0; 32]); 67];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut loc = r.start;
    asm!(
        "jmp 21f",
    "20:",
        "add {loc}, 64",
    "21:",
        "vmovdqu {c1}, ymmword ptr[{loc}]",
        "vptest {c1}, {mask}",
        "jnz 22f",
        "vmovdqu {c1}, ymmword ptr[{loc} + 32]",
        "vptest {c1}, {mask}",
        "jz 20b",
        "add {loc}, 32",
    "22:",
        "vpsllw {c1}, {c1}, 1",
        "vpmovmskb {r1:e}, {c1}",
        "tzcnt {r1:e}, {r1:e}",
        "add {loc}, {r1}",
        loc = inout(reg) loc,
        mask = in(ymm_reg) u8x32::splat(0x40),
        c1 = out(ymm_reg) _,
        r1 = out(reg) _,
        options(nostack, readonly),
    );
    let visited = &mut SCRATCH;
    visited.fill(Simd::splat(0));
    let mut loc = loc.offset_from(r.start) as usize;
    let mut total = 0;
    macro_rules! visit {
        () => {
            let bit = 1u32.wrapping_shl(loc as u32);
            let cell = (visited as *mut _ as *mut u32).add(loc / 32);
            let value = cell.read();
            total += (value & bit == 0) as u32;
            cell.write(value | bit);
        };
    }
    'outer: loop {
        loop {
            visit!();
            let next = loc.wrapping_sub(131);
            if next >= s.len() {
                break 'outer;
            }
            if *s.get_unchecked(next) == b'#' {
                loc = loc.wrapping_add(1);
                break;
            }
            loc = next;
        }
        loop {
            visit!();
            let next = loc.wrapping_add(1);
            if *s.get_unchecked(next) == b'\n' {
                break 'outer;
            }
            if *s.get_unchecked(next) == b'#' {
                loc = loc.wrapping_add(131);
                break;
            }
            loc = next;
        }
        loop {
            visit!();
            let next = loc.wrapping_add(131);
            if next >= s.len() {
                break 'outer;
            }
            if *s.get_unchecked(next) == b'#' {
                loc = loc.wrapping_sub(1);
                break;
            }
            loc = next;
        }
        loop {
            visit!();
            let next = loc.wrapping_sub(1);
            if *s.get_unchecked(next) == b'\n' {
                break 'outer;
            }
            if *s.get_unchecked(next) == b'#' {
                loc = loc.wrapping_sub(131);
                break;
            }
            loc = next;
        }
    }
    total
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &[u8]) -> u32 {
    static MASKS: [u64; 193] = {
        let mut masks = [0; 193];
        let mut i = 0;
        while i < 128 {
            masks[i] = ((!0u128 << i) >> 64) as u64;
            i += 1;
        }
        masks
    };

    let r = s.as_ptr_range();
    let mut loc = r.start;
    asm!(
        "jmp 21f",
    "20:",
        "add {loc}, 64",
    "21:",
        "vmovdqu {c1}, ymmword ptr[{loc}]",
        "vptest {c1}, {mask}",
        "jnz 22f",
        "vmovdqu {c1}, ymmword ptr[{loc} + 32]",
        "vptest {c1}, {mask}",
        "jz 20b",
        "add {loc}, 32",
    "22:",
        "vpsllw {c1}, {c1}, 1",
        "vpmovmskb {r1:e}, {c1}",
        "tzcnt {r1:e}, {r1:e}",
        "add {loc}, {r1}",
        loc = inout(reg) loc,
        mask = in(ymm_reg) u8x32::splat(0x40),
        c1 = out(ymm_reg) _,
        r1 = out(reg) _,
        options(nostack, readonly),
    );

    let loc = loc.offset_from(r.start) as usize;

    let mut x = loc % 131;
    let mut y = loc / 131;

    struct Tables {
        obstacles: [[u128; 130]; 4],
        visited: [[u8; 130]; 130],
    }

    let mut tables = Tables {
        obstacles: [[0; 130]; 4],
        visited: [[0; 130]; 130],
    };

    macro_rules! toggle_wall {
        ($x:expr, $y:expr) => {
            if $y <= 127 {
                *tables.obstacles[0].get_unchecked_mut($x) ^= 1 << 127 - $y;
            }
            if $x >= 2 {
                *tables.obstacles[1].get_unchecked_mut($y) ^= 1 << $x - 2;
            }
            if $y >= 2 {
                *tables.obstacles[2].get_unchecked_mut($x) ^= 1 << $y - 2;
            }
            if $x <= 127 {
                *tables.obstacles[3].get_unchecked_mut($y) ^= 1 << 127 - $x;
            }
        };
    }

    for y in 0..130 {
        for x in 0..130 {
            if *s.get_unchecked(y * 131 + x) == b'#' {
                toggle_wall!(x, y);
            }
        }
    }

    let masks = &MASKS;

    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn go_up(tables: &mut Tables, masks: &[u64; 193], x: usize, mut y: usize) -> bool {
        let obstacle_mask = tables.obstacles[0].get_unchecked(x)
            & (*masks.get_unchecked(192 - y) as u128
                | (*masks.get_unchecked(128 - y) as u128) << 64);
        if obstacle_mask == 0 {
            return false;
        }
        let c = 128 - obstacle_mask.trailing_zeros() as usize;
        y = c;
        let cell = tables.visited.get_unchecked_mut(y).get_unchecked_mut(x);
        if *cell & 1 != 0 {
            return true;
        }
        *cell |= 1;
        let r = go_right(tables, masks, x, y);
        *tables.visited.get_unchecked_mut(y).get_unchecked_mut(x) &= !1; // aaa
        r
    }

    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn go_right(tables: &mut Tables, masks: &[u64; 193], mut x: usize, y: usize) -> bool {
        let obstacle_mask = tables.obstacles[1].get_unchecked(y)
            & (*masks.get_unchecked(x + 63) as u128 | (*masks.get_unchecked(x - 1) as u128) << 64);
        if obstacle_mask == 0 {
            return false;
        }
        let c = obstacle_mask.trailing_zeros() as usize + 1;
        x = c;
        let cell = tables.visited.get_unchecked_mut(y).get_unchecked_mut(x);
        if *cell & 2 != 0 {
            return true;
        }
        *cell |= 2;
        let r = go_down(tables, masks, x, y);
        *tables.visited.get_unchecked_mut(y).get_unchecked_mut(x) &= !2; // aaa
        r
    }

    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn go_down(tables: &mut Tables, masks: &[u64; 193], x: usize, mut y: usize) -> bool {
        let obstacle_mask = tables.obstacles[2].get_unchecked(x)
            & (*masks.get_unchecked(y + 63) as u128 | (*masks.get_unchecked(y - 1) as u128) << 64);
        if obstacle_mask == 0 {
            return false;
        }
        let c = obstacle_mask.trailing_zeros() as usize + 1;
        y = c;
        let cell = tables.visited.get_unchecked_mut(y).get_unchecked_mut(x);
        if *cell & 4 != 0 {
            return true;
        }
        *cell |= 4;
        let r = go_left(tables, masks, x, y);
        *tables.visited.get_unchecked_mut(y).get_unchecked_mut(x) &= !4; // aaa
        r
    }

    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn go_left(tables: &mut Tables, masks: &[u64; 193], mut x: usize, y: usize) -> bool {
        let obstacle_mask = tables.obstacles[3].get_unchecked(y)
            & (*masks.get_unchecked(192 - x) as u128
                | (*masks.get_unchecked(128 - x) as u128) << 64);
        if obstacle_mask == 0 {
            return false;
        }
        let c = 128 - obstacle_mask.trailing_zeros() as usize;
        x = c;
        let cell = tables.visited.get_unchecked_mut(y).get_unchecked_mut(x);
        if *cell & 8 != 0 {
            return true;
        }
        *cell |= 8;
        let r = go_up(tables, masks, x, y);
        *tables.visited.get_unchecked_mut(y).get_unchecked_mut(x) &= !8; // aaa
        r
    }

    let mut total = 0;

    loop {
        let obstacle_mask = tables.obstacles[0].get_unchecked(x)
            & (*masks.get_unchecked(192 - y) as u128
                | (*masks.get_unchecked(128 - y) as u128) << 64);
        let c = 128usize.wrapping_sub(obstacle_mask.trailing_zeros() as usize);
        while y != c {
            *tables.visited.get_unchecked_mut(y).get_unchecked_mut(x) |= 1;
            y -= 1;
            if *tables.visited.get_unchecked(y).get_unchecked(x) == 0 {
                toggle_wall!(x, y);
                total += go_right(&mut tables, masks, x, y + 1) as u32;
                toggle_wall!(x, y);
            }
        }
        if obstacle_mask == 0 {
            break;
        }

        let obstacle_mask = tables.obstacles[1].get_unchecked(y)
            & (*masks.get_unchecked(x + 63) as u128 | (*masks.get_unchecked(x - 1) as u128) << 64);
        let c = obstacle_mask.trailing_zeros() as usize + 1;
        while x != c {
            *tables.visited.get_unchecked_mut(y).get_unchecked_mut(x) |= 2;
            x += 1;
            if *tables.visited.get_unchecked(y).get_unchecked(x) == 0 {
                toggle_wall!(x, y);
                total += go_down(&mut tables, masks, x - 1, y) as u32;
                toggle_wall!(x, y);
            }
        }
        if obstacle_mask == 0 {
            break;
        }

        let obstacle_mask = tables.obstacles[2].get_unchecked(x)
            & (*masks.get_unchecked(y + 63) as u128 | (*masks.get_unchecked(y - 1) as u128) << 64);
        let c = obstacle_mask.trailing_zeros() as usize + 1;
        while y != c {
            *tables.visited.get_unchecked_mut(y).get_unchecked_mut(x) |= 4;
            y += 1;
            if *tables.visited.get_unchecked(y).get_unchecked(x) == 0 {
                toggle_wall!(x, y);
                total += go_left(&mut tables, masks, x, y - 1) as u32;
                toggle_wall!(x, y);
            }
        }
        if obstacle_mask == 0 {
            break;
        }

        let obstacle_mask = tables.obstacles[3].get_unchecked(y)
            & (*masks.get_unchecked(192 - x) as u128
                | (*masks.get_unchecked(128 - x) as u128) << 64);
        let c = 128usize.wrapping_sub(obstacle_mask.trailing_zeros() as usize);
        while x != c {
            *tables.visited.get_unchecked_mut(y).get_unchecked_mut(x) |= 8;
            x -= 1;
            if *tables.visited.get_unchecked(y).get_unchecked(x) == 0 {
                toggle_wall!(x, y);
                total += go_up(&mut tables, masks, x + 1, y) as u32;
                toggle_wall!(x, y);
            }
        }
        if obstacle_mask == 0 {
            break;
        }
    }

    total
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
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
        let s = read_to_string("./inputs/6.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/6p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/6p2.txt").unwrap(),
        );
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
// unsafe fn is_loop(
//     tables: &mut Tables,
//     mut x: usize,
//     mut y: usize,
//     mut initial_d: u32,
//     masks: &[u64; 193],
// ) -> bool {
//     loop {
//         'left: {
//             'down: {
//                 'right: {
//                     'up: {
//                         match initial_d {
//                             0 => break 'up,
//                             1 => break 'right,
//                             2 => break 'down,
//                             3 => break 'left,
//                             _ => unreachable_unchecked(),
//                         }
//                     }
//                     let near =
//                         tables.obstacles[0].get_unchecked(x) & masks.get_unchecked(192 - y);
//                     let far =
//                         tables.obstacles[1].get_unchecked(x) & masks.get_unchecked(128 - y);
//                     if near | far == 0 {
//                         return false;
//                     }
//                     let c_lo = near.trailing_zeros();
//                     let c_hi = far.trailing_zeros() + 64;
//                     let c = if c_lo == 64 { c_hi } else { c_lo };
//                     y = 128 - c as usize;
//                     let cell = tables.visited.get_unchecked_mut(y).get_unchecked_mut(x);
//                     if *cell {
//                         return true;
//                     }
//                     *cell = true;
//                 }
//                 let near = tables.obstacles[2].get_unchecked(y) & masks.get_unchecked(x + 63);
//                 let far = tables.obstacles[3].get_unchecked(y) & masks.get_unchecked(x - 1);
//                 if near | far == 0 {
//                     return false;
//                 }
//                 let c_lo = near.trailing_zeros();
//                 let c_hi = far.trailing_zeros() + 64;
//                 let c = if c_lo == 64 { c_hi } else { c_lo };
//                 x = c as usize + 1;
//                 let cell = tables.visited.get_unchecked_mut(y).get_unchecked_mut(x);
//                 if *cell {
//                     return true;
//                 }
//                 *cell = true;
//             }
//             let near = tables.obstacles[4].get_unchecked(x) & masks.get_unchecked(y + 63);
//             let far = tables.obstacles[5].get_unchecked(x) & masks.get_unchecked(y - 1);
//             if near | far == 0 {
//                 return false;
//             }
//             let c_lo = near.trailing_zeros();
//             let c_hi = far.trailing_zeros() + 64;
//             let c = if c_lo == 64 { c_hi } else { c_lo };
//             y = c as usize + 1;
//             let cell = tables.visited.get_unchecked_mut(y).get_unchecked_mut(x);
//             if *cell {
//                 return true;
//             }
//             *cell = true;
//         }
//         let near = tables.obstacles[6].get_unchecked(y) & masks.get_unchecked(192 - x);
//         let far = tables.obstacles[7].get_unchecked(y) & masks.get_unchecked(128 - x);
//         if near | far == 0 {
//             return false;
//         }
//         let c_lo = near.trailing_zeros();
//         let c_hi = far.trailing_zeros() + 64;
//         let c = if c_lo == 64 { c_hi } else { c_lo };
//         x = 128 - c as usize;
//         let cell = tables.visited.get_unchecked_mut(y).get_unchecked_mut(x);
//         if *cell {
//             return true;
//         }
//         *cell = true;
//     }
// }
