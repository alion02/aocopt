use std::intrinsics::unlikely;

use super::*;

unsafe fn inner1(s: &[u8]) -> usize {
    let mut checksum = 0;

    asm!(
    "20:",
        "movzx {len:e}, byte ptr[{s} + {left} * 2]",
        "sub {len:e}, 48",
        "lea {scratch:e}, [{len} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {left}",
        "imul {scratch}, {len}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {len:e}",
        "movzx {rem_dst:e}, byte ptr[{s} + {left} * 2 + 1]",
        "inc {left:e}",
        "sub {rem_dst:e}, 48",
        "jz 20b",
        "cmp {left:e}, {right:e}",
        "je 50f",
    "22:",
        "dec {right:e}",
        "movzx {rem_src:e}, byte ptr[{s} + {right} * 2]",
        "sub {rem_src:e}, 48",
        "cmp {rem_dst}, {rem_src}",
        "ja 40f",
    "21:",
        "lea {scratch:e}, [{rem_dst} + {disk_pos} * 2 - 1]",
        "jb 30f",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_dst}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {rem_dst:e}",
        "cmp {left:e}, {right:e}",
        "jne 20b",
        "jmp 50f",
    "30:",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_dst}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {rem_dst:e}",
        "sub {rem_src:e}, {rem_dst:e}",
    "31:",
        "cmp {left:e}, {right:e}",
        "je 60f",
        "movzx {len:e}, byte ptr[{s} + {left} * 2]",
        "sub {len:e}, 48",
        "lea {scratch:e}, [{len} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {left}",
        "imul {scratch}, {len}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {len:e}",
        "movzx {rem_dst:e}, byte ptr[{s} + {left} * 2 + 1]",
        "inc {left:e}",
        "sub {rem_dst:e}, 48",
        "jz 31b",
        "cmp {rem_dst}, {rem_src}",
        "jbe 21b",
    "40:",
        "lea {scratch:e}, [{rem_src} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_src}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {rem_src:e}",
        "sub {rem_dst:e}, {rem_src:e}",
        "cmp {left:e}, {right:e}",
        "jne 22b",
        "jmp 50f",
    "60:",
        "lea {scratch:e}, [{rem_src} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_src}",
        "add {checksum}, {scratch}",
    "50:",
        "shr {checksum}",
        checksum = inout(reg) checksum,
        s = in(reg) s.as_ptr(),
        left = inout(reg) 0usize => _,
        right = inout(reg) s.len() / 2 => _,
        disk_pos = inout(reg) 0usize => _,
        rem_dst = out(reg) _,
        rem_src = out(reg) _,
        scratch = out(reg) _,
        len = out(reg) _,
        options(nostack, readonly),
    );

    checksum
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

unsafe fn inner2(s: &[u8]) -> usize {
    macro_rules! bt {
        ($btkind:literal, $buf:expr, $bit:expr) => {
            asm!(
                concat!("bt", $btkind, " [{buf}], {i:e}"),
                buf = in(reg) $buf,
                i = in(reg) $bit,
                options(nostack),
            );
        };
    }

    let mut buffers = [[0u64; 313]; 10];
    let mut disk_pos = 0;
    let mut disk_pos: [u32; 19999] = std::array::from_fn(|i| {
        let len = *s.get_unchecked(i) as usize - b'0' as usize;
        if i % 2 == 1 {
            bt!("s", buffers.get_unchecked_mut(len), i);
        }
        let r = disk_pos;
        disk_pos += len;
        r as u32
    });

    let mut pointers: [u16; 17] = std::array::from_fn(|i| {
        if (1..10).contains(&i) {
            buffers
                .get_unchecked(i)
                .iter()
                .zip((0u32..).step_by(64))
                .find_map(|(mask, mask_pos)| {
                    let pos = mask.trailing_zeros();
                    if pos < 64 {
                        Some((mask_pos + pos) as u16)
                    } else {
                        None
                    }
                })
                .unwrap_unchecked()
        } else {
            19998
        }
    });

    let mut checksum = 0;
    let mut id = s.len() / 2;
    loop {
        id -= 1;
        let i = id * 2;
        if (i as u16) < pointers[1] {
            break;
        }
        let used_len = *s.get_unchecked(i) as usize - b'0' as usize;
        let minpos: u16x8 = _mm_minpos_epu16(
            pointers
                .as_ptr()
                .add(used_len)
                .cast::<__m128i>()
                .read_unaligned(),
        )
        .into();
        let mut best_bucket = minpos[1] as usize + used_len;
        let mut span_idx = minpos[0] as usize;
        if unlikely(used_len == 1) {
            let span_idx_9 = pointers[9] as usize;
            if span_idx_9 < span_idx {
                best_bucket = 9;
                span_idx = span_idx_9;
            }
        }
        let best_ptr = pointers.get_unchecked_mut(best_bucket);
        let curr_disk_pos = *disk_pos.get_unchecked(i);
        let new_disk_pos = *disk_pos.get_unchecked(span_idx);
        let disk_pos = if new_disk_pos < curr_disk_pos {
            *disk_pos.get_unchecked_mut(span_idx) += used_len as u32;
            let buf = buffers.get_unchecked_mut(best_bucket).as_mut_ptr();
            bt!("r", buf, span_idx);
            let mut byte_idx = span_idx / 8;
            let mut zeros;
            loop {
                let mask = buf.byte_add(byte_idx).read_unaligned();
                zeros = mask.trailing_zeros();
                if zeros < 64 {
                    break;
                }
                byte_idx += 8;
            }
            let new_span_idx = byte_idx as u32 * 8 + zeros;
            *best_ptr = new_span_idx as u16;
            let new_bucket = best_bucket - used_len;
            let buf = buffers.get_unchecked_mut(new_bucket);
            bt!("s", buf, span_idx);
            let new_ptr = pointers.get_unchecked_mut(new_bucket);
            *new_ptr = (*new_ptr as u32).min(span_idx as u32) as u16;
            new_disk_pos
        } else {
            curr_disk_pos
        };
        checksum += (used_len + disk_pos as usize * 2 - 1) * id * used_len;
    }

    loop {
        let i = id * 2;
        let used_len = *s.get_unchecked(i) as usize - b'0' as usize;
        let disk_pos = *disk_pos.get_unchecked(i);
        checksum += (used_len + disk_pos as usize * 2 - 1) * id * used_len;
        if let Some(nid) = id.checked_sub(1) {
            id = nid;
        } else {
            break;
        }
    }

    checksum / 2
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
