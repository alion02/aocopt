use super::*;

#[inline]
unsafe fn inner1(s: &[u8]) -> u32 {
    static mut MAP: [i16; 142 * (141 + 40)] = [-1; 142 * (141 + 40)];
    let map = &mut MAP;
    map[142 * 20..142 * (141 + 20)].fill(black_box!(-1));
    let map = map.as_ptr().add(142 * 20);
    let ptr = s.as_ptr();
    let mut i = 0;
    let mut chunk;
    loop {
        i += 32;
        chunk = ptr.add(i).cast::<u8x32>().read_unaligned();
        if _mm256_testz_si256(chunk.into(), u8x32::splat(0x40).into()) == 0 {
            break;
        }
    }
    i += _mm256_movemask_epi8((chunk << 1).into()).trailing_zeros() as usize;
    let mut cuts = 0;
    asm!(
        "mov word ptr[{map} + {i} * 2], 100",
        "cmp byte ptr[{ptr} + {i} + 1], {wall}",
        "jne 200f", // right
        "cmp byte ptr[{ptr} + {i} + 142], {wall}",
        "jne 210f", // down
        "cmp byte ptr[{ptr} + {i} - 1], {wall}",
        "jne 220f", // left
        "cmp byte ptr[{ptr} + {i} - 142], {wall}",
        "jne 230f", // up
        "ud2",
    "300:",
        "add {dist:e}, 1",
        "lea {adj_dist:e}, [{dist:r} - 101]",
        "mov word ptr[{map} + {i} * 2], {dist:x}",
        "cmp word ptr[{map} + {i} * 2 + 4], {adj_dist:x}",
        "adc {cuts:e}, 0",
        "cmp word ptr[{map} + {i} * 2 + 568], {adj_dist:x}",
        "adc {cuts:e}, 0",
        "cmp word ptr[{map} + {i} * 2 - 4], {adj_dist:x}",
        "adc {cuts:e}, 0",
        "cmp word ptr[{map} + {i} * 2 - 568], {adj_dist:x}",
        "adc {cuts:e}, 0",
        "ret",
    "200:", // right
        "add {i:e}, 1",
        "call 300b",
        "add {i:e}, 1",
        "call 300b",
        "cmp byte ptr[{ptr} + {i} + 1], {wall}",
        "jne 200b", // right
        "cmp byte ptr[{ptr} + {i} - 142], {wall}",
        "jne 230f", // up
        "cmp byte ptr[{ptr} + {i} + 142], {wall}",
        "je 20f", // done
    "210:", // down
        "add {i:e}, 142",
        "call 300b",
        "add {i:e}, 142",
        "call 300b",
        "cmp byte ptr[{ptr} + {i} + 142], {wall}",
        "jne 210b", // down
        "cmp byte ptr[{ptr} + {i} + 1], {wall}",
        "jne 200b", // right
        "cmp byte ptr[{ptr} + {i} - 1], {wall}",
        "je 20f", // done
    "220:", // left
        "add {i:e}, -1",
        "call 300b",
        "add {i:e}, -1",
        "call 300b",
        "cmp byte ptr[{ptr} + {i} - 1], {wall}",
        "jne 220b", // left
        "cmp byte ptr[{ptr} + {i} + 142], {wall}",
        "jne 210b", // down
        "cmp byte ptr[{ptr} + {i} - 142], {wall}",
        "je 20f", // done
    "230:", // up
        "add {i:e}, -142",
        "call 300b",
        "add {i:e}, -142",
        "call 300b",
        "cmp byte ptr[{ptr} + {i} - 142], {wall}",
        "jne 230b", // up
        "cmp byte ptr[{ptr} + {i} - 1], {wall}",
        "jne 220b", // left
        "cmp byte ptr[{ptr} + {i} + 1], {wall}",
        "jne 200b", // right
        // done
    "20:",
        ptr = in(reg) ptr,
        map = in(reg) map,
        i = inout(reg) i => _,
        dist = inout(reg) 100 => _,
        adj_dist = out(reg) _,
        cuts = inout(reg) cuts,
        wall = const b'#',
    );

    cuts
}

#[inline]
unsafe fn inner2(s: &[u8]) -> u32 {
    static mut MAP: [i16; 160 * (141 + 40)] = [32767; 160 * (141 + 40)];
    let map = MAP.as_mut_ptr().add(160 * 20);
    let ptr = s.as_ptr();
    let mut start = ptr::null();
    for row in 1..140 {
        for i in 0..5 {
            let i = i * 32;
            let j = row * 160 + i;
            let i = row * 142 + i;
            let chunk = ptr.add(i).cast::<u8x32>().read_unaligned();
            if unlikely(_mm256_testz_si256(chunk.into(), u8x32::splat(0x40).into()) == 0) {
                start = map.add(j + _mm256_movemask_epi8((chunk << 1).into()).trailing_zeros() as usize);
            }
            let chunk = chunk.cast::<i16>();
            map.add(j).cast::<i16x32>().write_unaligned(
                chunk
                    .simd_eq(Simd::splat(b'#' as _))
                    .select(Simd::splat(32767), Simd::splat(32766)),
            );
        }
    }
    const unsafe fn luts() -> ([i16x16; 39], [usize; 75], [isize; 75]) {
        let mut penalties = [-32768i16; 16 * 39];
        let mut indices = [0; 75];
        let mut offsets = [0; 75];
        let mut width = 1;
        let mut i = 0;
        let mut y = 0;
        while i < 39 {
            let reserved = width / 16 + 1;
            let slot = i * 16;
            let mut j = 0;
            while j < width {
                penalties[slot + j] = -119 - j.abs_diff(y) as i16 + y as i16;
                j += 1;
            }
            let mut k = 0;
            while k < reserved {
                indices[i + k] = (i + k) * 32;
                indices[75 - reserved - i + k] = (i + k) * 32;
                let y = y as isize;
                offsets[i + k] = (20 - y) * 320 - y * 2 + k as isize * 32;
                offsets[75 - reserved - i + k] = (y - 20) * 320 - y * 2 + k as isize * 32;
                k += 1;
            }
            width += 2;
            i += reserved;
            y += 1;
        }

        (transmute(penalties), indices, offsets)
    }
    static PENALTIES: [i16x16; 39] = unsafe { luts().0 };
    const INDICES: [usize; 75] = unsafe { luts().1 };
    const OFFSETS: [isize; 75] = unsafe { luts().2 };
    let cuts: u32x8;
    asm!(
        "mov word ptr[{ptr}], 0",
        "cmp word ptr[{ptr} + 2*1], 32767",
        "jne 200f", // right
        "cmp word ptr[{ptr} + 2*160], 32767",
        "jne 210f", // down
        "cmp word ptr[{ptr} - 2*1], 32767",
        "jne 220f", // left
        "cmp word ptr[{ptr} - 2*160], 32767",
        "jne 230f", // up
        "ud2",
    "300:",
        "add {dist:e}, 1",
        "vpsubw {vecdist}, {vecdist}, {vecneg1}",
        "mov word ptr[{ptr}], {dist:x}",
        "vpxor {cutsw}, {cutsw}, {cutsw}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx23}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off23}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off25}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off28}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off31}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off34}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off37}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off40}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off43}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off46}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off49}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off52}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx1}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off1}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off75}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off10}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off67}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off27}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off51}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx2}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off2}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off74}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx3}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off3}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off73}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx4}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off4}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off72}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx5}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off5}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off71}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx6}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off6}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off70}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx7}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off7}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off69}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx8}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off8}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off68}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx9}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off9}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off66}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx11}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off11}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off64}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx12}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off12}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off65}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off30}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off48}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx13}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off13}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off62}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx14}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off14}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off63}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx15}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off15}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off60}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx16}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off16}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off61}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off36}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off42}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx17}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off17}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off58}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx18}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off18}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off59}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off39}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx19}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off19}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off56}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx20}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off20}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off57}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx21}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off21}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off54}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx22}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off22}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off55}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx24}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off24}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off53}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx26}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off26}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off50}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx29}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off29}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off47}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx32}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off32}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off44}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx33}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off33}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off45}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx35}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off35}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off41}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpaddw {adjdist}, {vecdist}, [rip + {penalties}+{idx38}]",
        "vpcmpgtw {tmp}, {adjdist}, [{ptr} + {off38}]",
        "vpaddw {cutsw}, {cutsw}, {tmp}",
        "vpmaddwd {tmp}, {cutsw}, {vecneg1}",
        "vpaddd {cutsd}, {tmp}, {cutsd}",
        "ret",
    "200:", // right
        "add {ptr}, 1*2",
        "call 300b",
        "add {ptr}, 1*2",
        "call 300b",
        "cmp word ptr[{ptr} + 2*1], 32767",
        "jne 200b", // right
        "cmp word ptr[{ptr} - 2*160], 32767",
        "jne 230f", // up
        "cmp word ptr[{ptr} + 2*160], 32767",
        "je 20f", // done
    "210:", // down
        "add {ptr}, 160*2",
        "call 300b",
        "add {ptr}, 160*2",
        "call 300b",
        "cmp word ptr[{ptr} + 2*160], 32767",
        "jne 210b", // down
        "cmp word ptr[{ptr} + 2*1], 32767",
        "jne 200b", // right
        "cmp word ptr[{ptr} - 2*1], 32767",
        "je 20f", // done
    "220:", // left
        "add {ptr}, -1*2",
        "call 300b",
        "add {ptr}, -1*2",
        "call 300b",
        "cmp word ptr[{ptr} - 2*1], 32767",
        "jne 220b", // left
        "cmp word ptr[{ptr} + 2*160], 32767",
        "jne 210b", // down
        "cmp word ptr[{ptr} - 2*160], 32767",
        "je 20f", // done
    "230:", // up
        "add {ptr}, -160*2",
        "call 300b",
        "add {ptr}, -160*2",
        "call 300b",
        "cmp word ptr[{ptr} - 2*160], 32767",
        "jne 230b", // up
        "cmp word ptr[{ptr} - 2*1], 32767",
        "jne 220b", // left
        "cmp word ptr[{ptr} + 2*1], 32767",
        "jne 200b", // right
        // done
    "20:",

        ptr = in(reg) start,
        dist = inout(reg) 0 => _,
        cutsw = out(ymm_reg) _,
        cutsd = inout(ymm_reg) u32x8::splat(0) => cuts,
        tmp = out(ymm_reg) _,
        vecdist = inout(ymm_reg) i16x16::splat(0) => _,
        vecneg1 = in(ymm_reg) i16x16::splat(-1),
        penalties = sym PENALTIES,
        adjdist = out(ymm_reg) _,
        off1 = const OFFSETS[0],
        off2 = const OFFSETS[1],
        off3 = const OFFSETS[2],
        off4 = const OFFSETS[3],
        off5 = const OFFSETS[4],
        off6 = const OFFSETS[5],
        off7 = const OFFSETS[6],
        off8 = const OFFSETS[7],
        off9 = const OFFSETS[8],
        off10 = const OFFSETS[9],
        off11 = const OFFSETS[10],
        off12 = const OFFSETS[11],
        off13 = const OFFSETS[12],
        off14 = const OFFSETS[13],
        off15 = const OFFSETS[14],
        off16 = const OFFSETS[15],
        off17 = const OFFSETS[16],
        off18 = const OFFSETS[17],
        off19 = const OFFSETS[18],
        off20 = const OFFSETS[19],
        off21 = const OFFSETS[20],
        off22 = const OFFSETS[21],
        off23 = const OFFSETS[22],
        off24 = const OFFSETS[23],
        off25 = const OFFSETS[24],
        off26 = const OFFSETS[25],
        off27 = const OFFSETS[26],
        off28 = const OFFSETS[27],
        off29 = const OFFSETS[28],
        off30 = const OFFSETS[29],
        off31 = const OFFSETS[30],
        off32 = const OFFSETS[31],
        off33 = const OFFSETS[32],
        off34 = const OFFSETS[33],
        off35 = const OFFSETS[34],
        off36 = const OFFSETS[35],
        off37 = const OFFSETS[36],
        off38 = const OFFSETS[37],
        off39 = const OFFSETS[38],
        off40 = const OFFSETS[39],
        off41 = const OFFSETS[40],
        off42 = const OFFSETS[41],
        off43 = const OFFSETS[42],
        off44 = const OFFSETS[43],
        off45 = const OFFSETS[44],
        off46 = const OFFSETS[45],
        off47 = const OFFSETS[46],
        off48 = const OFFSETS[47],
        off49 = const OFFSETS[48],
        off50 = const OFFSETS[49],
        off51 = const OFFSETS[50],
        off52 = const OFFSETS[51],
        off53 = const OFFSETS[52],
        off54 = const OFFSETS[53],
        off55 = const OFFSETS[54],
        off56 = const OFFSETS[55],
        off57 = const OFFSETS[56],
        off58 = const OFFSETS[57],
        off59 = const OFFSETS[58],
        off60 = const OFFSETS[59],
        off61 = const OFFSETS[60],
        off62 = const OFFSETS[61],
        off63 = const OFFSETS[62],
        off64 = const OFFSETS[63],
        off65 = const OFFSETS[64],
        off66 = const OFFSETS[65],
        off67 = const OFFSETS[66],
        off68 = const OFFSETS[67],
        off69 = const OFFSETS[68],
        off70 = const OFFSETS[69],
        off71 = const OFFSETS[70],
        off72 = const OFFSETS[71],
        off73 = const OFFSETS[72],
        off74 = const OFFSETS[73],
        off75 = const OFFSETS[74],
        idx1 = const INDICES[0],
        idx2 = const INDICES[1],
        idx3 = const INDICES[2],
        idx4 = const INDICES[3],
        idx5 = const INDICES[4],
        idx6 = const INDICES[5],
        idx7 = const INDICES[6],
        idx8 = const INDICES[7],
        idx9 = const INDICES[8],
        idx11 = const INDICES[10],
        idx12 = const INDICES[11],
        idx13 = const INDICES[12],
        idx14 = const INDICES[13],
        idx15 = const INDICES[14],
        idx16 = const INDICES[15],
        idx17 = const INDICES[16],
        idx18 = const INDICES[17],
        idx19 = const INDICES[18],
        idx20 = const INDICES[19],
        idx21 = const INDICES[20],
        idx22 = const INDICES[21],
        idx23 = const INDICES[22],
        idx24 = const INDICES[23],
        idx26 = const INDICES[25],
        idx29 = const INDICES[28],
        idx32 = const INDICES[31],
        idx33 = const INDICES[32],
        idx35 = const INDICES[34],
        idx38 = const INDICES[37],
    );

    cuts.reduce_sum()
}

#[inline]
pub fn part1(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> u32 {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/20.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/20p1.txt").unwrap(),);
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/20.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/20p2.txt").unwrap(),);
    }
}
