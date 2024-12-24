use super::*;

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut ptr = r.start.cast::<u8x16>();
    let mut graph_arr = [[u8x32::splat(0); 3]; 26 * 26];
    let graph = graph_arr.as_mut_ptr();
    let mut total = 0;
    for i in 0..3380 / 2 {
        let chunk = if i == 0 {
            let chunk = ptr.read_unaligned() - Simd::splat(b'a');
            simd_swizzle!(chunk, [0, 1, 3, 4, 6, 7, 9, 10, 0, 1, 3, 4, 6, 7, 9, 10])
        } else {
            let chunk = ptr.byte_sub(5).read_unaligned() - Simd::splat(b'a');
            simd_swizzle!(chunk, [5, 6, 8, 9, 11, 12, 14, 15, 5, 6, 8, 9, 11, 12, 14, 15])
        };
        let chunk: u16x8 = _mm_maddubs_epi16(
            chunk.into(),
            u8x16::from_array([26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1]).into(),
        )
        .into();
        let mask = chunk - Simd::splat((b't' - b'a') as u16 * 26);
        let mask = mask.simd_lt(Simd::splat(26)).to_int();
        for i in (0..4).step_by(2) {
            let l = chunk[i] as u32;
            let r = chunk[i + 1] as u32;
            macro_rules! link {
                ($a:expr, $b:expr) => {{
                    let o: usize;
                    asm!(
                        "imul {t:e}, {a:e}, 96",
                        "bts dword ptr[{graph} + {t}], {b:e}",
                        t = out(reg) o,
                        graph = in(reg) graph,
                        a = in(reg) $a,
                        b = in(reg) $b,
                        options(nostack),
                    );
                    o
                }};
            }
            let il = link!(l, r);
            let ir = link!(r, l);
            let s = if mask[i] != 0 || mask[i + 1] != 0 {
                let intersection: [u8x32; 3] = array::from_fn(|i| {
                    graph.byte_add(il).cast::<u8x32>().add(i).read() & graph.byte_add(ir).cast::<u8x32>().add(i).read()
                });
                let s: u32 = intersection
                    .iter()
                    .flat_map(|s| s.as_array())
                    .map(|m| m.count_ones())
                    .sum();
                s
            } else {
                let intersection = (graph.byte_add(il).cast::<u32>().byte_add(61).read()
                    & graph.byte_add(ir).cast::<u32>().byte_add(61).read())
                    >> 6;
                intersection.count_ones()
            };
            total += s;
        }
        ptr = ptr.byte_add(12);
    }
    total
}

#[inline]
#[repr(align(64))]
unsafe fn inner2(s: &str) -> String {
    use std::collections::{HashMap, HashSet};
    let connections = s.lines().map(|l| l.split_once('-').unwrap()).collect::<Vec<_>>();

    let mut graph = HashMap::<&str, HashSet<&str>>::new();
    for &(a, b) in &connections {
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let mut pass = HashSet::new();
    for &(a, b) in &connections {
        let intersection = graph[a].intersection(&graph[b]).collect::<Vec<_>>();
        for i in 0..intersection.len() {
            let mut clique = HashSet::from_iter([a, b]);
            for &new in &intersection[i..] {
                if clique.contains(new) {
                    continue;
                }
                if graph[new].intersection(&clique).count() == clique.len() {
                    clique.insert(new);
                }
            }
            if clique.len() > pass.len() {
                pass = clique;
            }
        }
    }
    let mut pass = pass.iter().collect::<Vec<_>>();
    pass.sort_unstable();
    let mut out = String::with_capacity(30);
    for comp in pass {
        use std::fmt::Write;
        write!(out, "{comp}").unwrap_unchecked();
        out.push(',');
    }
    out.pop();
    out
}

#[inline]
pub fn part1(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> String {
    unsafe { inner2(s) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/23.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/23p1.txt").unwrap(),);
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/23.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/23p2.txt").unwrap(),);
    }
}
