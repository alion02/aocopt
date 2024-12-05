use super::*;

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    let end = r.end;
    let mut long_sums = u16x16::splat(0);
    let mut sums = Simd::splat(0);

    let mut c000;
    let mut c000_x;
    let mut c000_m;
    let mut c000_a;
    let mut c000_s;
    let mut c001;
    let mut c001_x;
    let mut c001_m;
    let mut c001_a;
    let mut c001_s;
    let mut c002;
    let mut c002_x;
    let mut c002_m;
    let mut c002_a;
    let mut c002_s;
    let mut c003;
    let mut c003_x;
    let mut c003_m;
    let mut c003_a;
    let mut c003_s;
    let mut c010;
    let mut c010_x;
    let mut c010_m;
    let mut c010_a;
    let mut c010_s;
    let mut c011;
    let mut c011_x;
    let mut c011_m;
    let mut c011_a;
    let mut c011_s;
    let mut c012;
    let mut c012_x;
    let mut c012_m;
    let mut c012_a;
    let mut c012_s;
    let mut c013;
    let mut c013_x;
    let mut c013_m;
    let mut c013_a;
    let mut c013_s;
    let mut c020;
    let mut c020_x;
    let mut c020_m;
    let mut c020_a;
    let mut c020_s;
    let mut c021;
    let mut c021_x;
    let mut c021_m;
    let mut c021_a;
    let mut c021_s;
    let mut c022;
    let mut c022_x;
    let mut c022_m;
    let mut c022_a;
    let mut c022_s;
    let mut c023;
    let mut c023_x;
    let mut c023_m;
    let mut c023_a;
    let mut c023_s;
    let mut c030;
    let mut c030_x;
    let mut c030_m;
    let mut c030_a;
    let mut c030_s;
    let mut c031;
    let mut c031_x;
    let mut c031_m;
    let mut c031_a;
    let mut c031_s;
    let mut c032;
    let mut c032_x;
    let mut c032_m;
    let mut c032_a;
    let mut c032_s;
    let mut c033;
    let mut c033_x;
    let mut c033_m;
    let mut c033_a;
    let mut c033_s;
    let mut c040;
    let mut c040_x;
    let mut c040_m;
    let mut c040_a;
    let mut c040_s;
    let mut c041;
    let mut c041_x;
    let mut c041_m;
    let mut c041_a;
    let mut c041_s;
    let mut c042;
    let mut c042_x;
    let mut c042_m;
    let mut c042_a;
    let mut c042_s;
    let mut c043;
    let mut c043_x;
    let mut c043_m;
    let mut c043_a;
    let mut c043_s;
    let mut c100;
    let mut c100_x;
    let mut c100_m;
    let mut c100_a;
    let mut c100_s;
    let mut c101;
    let mut c101_x;
    let mut c101_m;
    let mut c101_a;
    let mut c101_s;
    let mut c102;
    let mut c102_x;
    let mut c102_m;
    let mut c102_a;
    let mut c102_s;
    let mut c103;
    let mut c103_x;
    let mut c103_m;
    let mut c103_a;
    let mut c103_s;
    let mut c110;
    let mut c110_x;
    let mut c110_m;
    let mut c110_a;
    let mut c110_s;
    let mut c111;
    let mut c111_x;
    let mut c111_m;
    let mut c111_a;
    let mut c111_s;
    let mut c112;
    let mut c112_x;
    let mut c112_m;
    let mut c112_a;
    let mut c112_s;
    let mut c113;
    let mut c113_x;
    let mut c113_m;
    let mut c113_a;
    let mut c113_s;
    let mut c120;
    let mut c120_x;
    let mut c120_m;
    let mut c120_a;
    let mut c120_s;
    let mut c121;
    let mut c121_x;
    let mut c121_m;
    let mut c121_a;
    let mut c121_s;
    let mut c122;
    let mut c122_x;
    let mut c122_m;
    let mut c122_a;
    let mut c122_s;
    let mut c123;
    let mut c123_x;
    let mut c123_m;
    let mut c123_a;
    let mut c123_s;
    let mut c130;
    let mut c130_x;
    let mut c130_m;
    let mut c130_a;
    let mut c130_s;
    let mut c131;
    let mut c131_x;
    let mut c131_m;
    let mut c131_a;
    let mut c131_s;
    let mut c132;
    let mut c132_x;
    let mut c132_m;
    let mut c132_a;
    let mut c132_s;
    let mut c133;
    let mut c133_x;
    let mut c133_m;
    let mut c133_a;
    let mut c133_s;
    let mut c140;
    let mut c140_x;
    let mut c140_m;
    let mut c140_a;
    let mut c140_s;
    let mut c141;
    let mut c141_x;
    let mut c141_m;
    let mut c141_a;
    let mut c141_s;
    let mut c142;
    let mut c142_x;
    let mut c142_m;
    let mut c142_a;
    let mut c142_s;
    let mut c143;
    let mut c143_x;
    let mut c143_m;
    let mut c143_a;
    let mut c143_s;
    let mut c200;
    let mut c200_x;
    let mut c200_m;
    let mut c200_a;
    let mut c200_s;
    let mut c201;
    let mut c201_x;
    let mut c201_m;
    let mut c201_a;
    let mut c201_s;
    let mut c202;
    let mut c202_x;
    let mut c202_m;
    let mut c202_a;
    let mut c202_s;
    let mut c203;
    let mut c203_x;
    let mut c203_m;
    let mut c203_a;
    let mut c203_s;
    let mut c210;
    let mut c210_x;
    let mut c210_m;
    let mut c210_a;
    let mut c210_s;
    let mut c211;
    let mut c211_x;
    let mut c211_m;
    let mut c211_a;
    let mut c211_s;
    let mut c212;
    let mut c212_x;
    let mut c212_m;
    let mut c212_a;
    let mut c212_s;
    let mut c213;
    let mut c213_x;
    let mut c213_m;
    let mut c213_a;
    let mut c213_s;
    let mut c220;
    let mut c220_x;
    let mut c220_m;
    let mut c220_a;
    let mut c220_s;
    let mut c221;
    let mut c221_x;
    let mut c221_m;
    let mut c221_a;
    let mut c221_s;
    let mut c222;
    let mut c222_x;
    let mut c222_m;
    let mut c222_a;
    let mut c222_s;
    let mut c223;
    let mut c223_x;
    let mut c223_m;
    let mut c223_a;
    let mut c223_s;
    let mut c230;
    let mut c230_x;
    let mut c230_m;
    let mut c230_a;
    let mut c230_s;
    let mut c231;
    let mut c231_x;
    let mut c231_m;
    let mut c231_a;
    let mut c231_s;
    let mut c232;
    let mut c232_x;
    let mut c232_m;
    let mut c232_a;
    let mut c232_s;
    let mut c233;
    let mut c233_x;
    let mut c233_m;
    let mut c233_a;
    let mut c233_s;
    let mut c240;
    let mut c240_x;
    let mut c240_m;
    let mut c240_a;
    let mut c240_s;
    let mut c241;
    let mut c241_x;
    let mut c241_m;
    let mut c241_a;
    let mut c241_s;
    let mut c242;
    let mut c242_x;
    let mut c242_m;
    let mut c242_a;
    let mut c242_s;
    let mut c243;
    let mut c243_x;
    let mut c243_m;
    let mut c243_a;
    let mut c243_s;
    let mut c300;
    let mut c300_x;
    let mut c300_m;
    let mut c300_a;
    let mut c300_s;
    let mut c301;
    let mut c301_x;
    let mut c301_m;
    let mut c301_a;
    let mut c301_s;
    let mut c302;
    let mut c302_x;
    let mut c302_m;
    let mut c302_a;
    let mut c302_s;
    let mut c303;
    let mut c303_x;
    let mut c303_m;
    let mut c303_a;
    let mut c303_s;
    let mut c310;
    let mut c310_x;
    let mut c310_m;
    let mut c310_a;
    let mut c310_s;
    let mut c311;
    let mut c311_x;
    let mut c311_m;
    let mut c311_a;
    let mut c311_s;
    let mut c312;
    let mut c312_x;
    let mut c312_m;
    let mut c312_a;
    let mut c312_s;
    let mut c313;
    let mut c313_x;
    let mut c313_m;
    let mut c313_a;
    let mut c313_s;
    let mut c320;
    let mut c320_x;
    let mut c320_m;
    let mut c320_a;
    let mut c320_s;
    let mut c321;
    let mut c321_x;
    let mut c321_m;
    let mut c321_a;
    let mut c321_s;
    let mut c322;
    let mut c322_x;
    let mut c322_m;
    let mut c322_a;
    let mut c322_s;
    let mut c323;
    let mut c323_x;
    let mut c323_m;
    let mut c323_a;
    let mut c323_s;
    let mut c330;
    let mut c330_x;
    let mut c330_m;
    let mut c330_a;
    let mut c330_s;
    let mut c331;
    let mut c331_x;
    let mut c331_m;
    let mut c331_a;
    let mut c331_s;
    let mut c332;
    let mut c332_x;
    let mut c332_m;
    let mut c332_a;
    let mut c332_s;
    let mut c333;
    let mut c333_x;
    let mut c333_m;
    let mut c333_a;
    let mut c333_s;
    let mut c340;
    let mut c340_x;
    let mut c340_m;
    let mut c340_a;
    let mut c340_s;
    let mut c341;
    let mut c341_x;
    let mut c341_m;
    let mut c341_a;
    let mut c341_s;
    let mut c342;
    let mut c342_x;
    let mut c342_m;
    let mut c342_a;
    let mut c342_s;
    let mut c343;
    let mut c343_x;
    let mut c343_m;
    let mut c343_a;
    let mut c343_s;
    c000 = (ptr.add(0) as *const u8x32).read_unaligned();
    c000_x = c000.simd_eq(Simd::splat(b'X'));
    c000_m = c000.simd_eq(Simd::splat(b'M'));
    c000_a = c000.simd_eq(Simd::splat(b'A'));
    c000_s = c000.simd_eq(Simd::splat(b'S'));
    c001 = (ptr.add(1) as *const u8x32).read_unaligned();
    c001_x = c001.simd_eq(Simd::splat(b'X'));
    c001_m = c001.simd_eq(Simd::splat(b'M'));
    c001_a = c001.simd_eq(Simd::splat(b'A'));
    c001_s = c001.simd_eq(Simd::splat(b'S'));
    c002 = (ptr.add(2) as *const u8x32).read_unaligned();
    c002_x = c002.simd_eq(Simd::splat(b'X'));
    c002_m = c002.simd_eq(Simd::splat(b'M'));
    c002_a = c002.simd_eq(Simd::splat(b'A'));
    c002_s = c002.simd_eq(Simd::splat(b'S'));
    c003 = (ptr.add(3) as *const u8x32).read_unaligned();
    c003_x = c003.simd_eq(Simd::splat(b'X'));
    c003_m = c003.simd_eq(Simd::splat(b'M'));
    c003_a = c003.simd_eq(Simd::splat(b'A'));
    c003_s = c003.simd_eq(Simd::splat(b'S'));
    sums -=
        (c000_x & c001_m & c002_a & c003_s).to_int() + (c000_s & c001_a & c002_m & c003_x).to_int();
    c010 = (ptr.add(32) as *const u8x32).read_unaligned();
    c010_x = c010.simd_eq(Simd::splat(b'X'));
    c010_m = c010.simd_eq(Simd::splat(b'M'));
    c010_a = c010.simd_eq(Simd::splat(b'A'));
    c010_s = c010.simd_eq(Simd::splat(b'S'));
    c011 = (ptr.add(33) as *const u8x32).read_unaligned();
    c011_x = c011.simd_eq(Simd::splat(b'X'));
    c011_m = c011.simd_eq(Simd::splat(b'M'));
    c011_a = c011.simd_eq(Simd::splat(b'A'));
    c011_s = c011.simd_eq(Simd::splat(b'S'));
    c012 = (ptr.add(34) as *const u8x32).read_unaligned();
    c012_x = c012.simd_eq(Simd::splat(b'X'));
    c012_m = c012.simd_eq(Simd::splat(b'M'));
    c012_a = c012.simd_eq(Simd::splat(b'A'));
    c012_s = c012.simd_eq(Simd::splat(b'S'));
    c013 = (ptr.add(35) as *const u8x32).read_unaligned();
    c013_x = c013.simd_eq(Simd::splat(b'X'));
    c013_m = c013.simd_eq(Simd::splat(b'M'));
    c013_a = c013.simd_eq(Simd::splat(b'A'));
    c013_s = c013.simd_eq(Simd::splat(b'S'));
    sums -=
        (c010_x & c011_m & c012_a & c013_s).to_int() + (c010_s & c011_a & c012_m & c013_x).to_int();
    c020 = (ptr.add(64) as *const u8x32).read_unaligned();
    c020_x = c020.simd_eq(Simd::splat(b'X'));
    c020_m = c020.simd_eq(Simd::splat(b'M'));
    c020_a = c020.simd_eq(Simd::splat(b'A'));
    c020_s = c020.simd_eq(Simd::splat(b'S'));
    c021 = (ptr.add(65) as *const u8x32).read_unaligned();
    c021_x = c021.simd_eq(Simd::splat(b'X'));
    c021_m = c021.simd_eq(Simd::splat(b'M'));
    c021_a = c021.simd_eq(Simd::splat(b'A'));
    c021_s = c021.simd_eq(Simd::splat(b'S'));
    c022 = (ptr.add(66) as *const u8x32).read_unaligned();
    c022_x = c022.simd_eq(Simd::splat(b'X'));
    c022_m = c022.simd_eq(Simd::splat(b'M'));
    c022_a = c022.simd_eq(Simd::splat(b'A'));
    c022_s = c022.simd_eq(Simd::splat(b'S'));
    c023 = (ptr.add(67) as *const u8x32).read_unaligned();
    c023_x = c023.simd_eq(Simd::splat(b'X'));
    c023_m = c023.simd_eq(Simd::splat(b'M'));
    c023_a = c023.simd_eq(Simd::splat(b'A'));
    c023_s = c023.simd_eq(Simd::splat(b'S'));
    sums -=
        (c020_x & c021_m & c022_a & c023_s).to_int() + (c020_s & c021_a & c022_m & c023_x).to_int();
    c030 = (ptr.add(96) as *const u8x32).read_unaligned();
    c030_x = c030.simd_eq(Simd::splat(b'X'));
    c030_m = c030.simd_eq(Simd::splat(b'M'));
    c030_a = c030.simd_eq(Simd::splat(b'A'));
    c030_s = c030.simd_eq(Simd::splat(b'S'));
    c031 = (ptr.add(97) as *const u8x32).read_unaligned();
    c031_x = c031.simd_eq(Simd::splat(b'X'));
    c031_m = c031.simd_eq(Simd::splat(b'M'));
    c031_a = c031.simd_eq(Simd::splat(b'A'));
    c031_s = c031.simd_eq(Simd::splat(b'S'));
    c032 = (ptr.add(98) as *const u8x32).read_unaligned();
    c032_x = c032.simd_eq(Simd::splat(b'X'));
    c032_m = c032.simd_eq(Simd::splat(b'M'));
    c032_a = c032.simd_eq(Simd::splat(b'A'));
    c032_s = c032.simd_eq(Simd::splat(b'S'));
    c033 = (ptr.add(99) as *const u8x32).read_unaligned();
    c033_x = c033.simd_eq(Simd::splat(b'X'));
    c033_m = c033.simd_eq(Simd::splat(b'M'));
    c033_a = c033.simd_eq(Simd::splat(b'A'));
    c033_s = c033.simd_eq(Simd::splat(b'S'));
    sums -=
        (c030_x & c031_m & c032_a & c033_s).to_int() + (c030_s & c031_a & c032_m & c033_x).to_int();
    c040 = simd_swizzle!(
        (ptr.add(124) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c040_x = c040.simd_eq(Simd::splat(b'X'));
    c040_m = c040.simd_eq(Simd::splat(b'M'));
    c040_a = c040.simd_eq(Simd::splat(b'A'));
    c040_s = c040.simd_eq(Simd::splat(b'S'));
    c041 = simd_swizzle!(
        (ptr.add(124) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c041_x = c041.simd_eq(Simd::splat(b'X'));
    c041_m = c041.simd_eq(Simd::splat(b'M'));
    c041_a = c041.simd_eq(Simd::splat(b'A'));
    c041_s = c041.simd_eq(Simd::splat(b'S'));
    c042 = simd_swizzle!(
        (ptr.add(124) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c042_x = c042.simd_eq(Simd::splat(b'X'));
    c042_m = c042.simd_eq(Simd::splat(b'M'));
    c042_a = c042.simd_eq(Simd::splat(b'A'));
    c042_s = c042.simd_eq(Simd::splat(b'S'));
    c043 = simd_swizzle!(
        (ptr.add(124) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c043_x = c043.simd_eq(Simd::splat(b'X'));
    c043_m = c043.simd_eq(Simd::splat(b'M'));
    c043_a = c043.simd_eq(Simd::splat(b'A'));
    c043_s = c043.simd_eq(Simd::splat(b'S'));
    sums -=
        (c040_x & c041_m & c042_a & c043_s).to_int() + (c040_s & c041_a & c042_m & c043_x).to_int();
    c100 = (ptr.add(141) as *const u8x32).read_unaligned();
    c100_x = c100.simd_eq(Simd::splat(b'X'));
    c100_m = c100.simd_eq(Simd::splat(b'M'));
    c100_a = c100.simd_eq(Simd::splat(b'A'));
    c100_s = c100.simd_eq(Simd::splat(b'S'));
    c101 = (ptr.add(142) as *const u8x32).read_unaligned();
    c101_x = c101.simd_eq(Simd::splat(b'X'));
    c101_m = c101.simd_eq(Simd::splat(b'M'));
    c101_a = c101.simd_eq(Simd::splat(b'A'));
    c101_s = c101.simd_eq(Simd::splat(b'S'));
    c102 = (ptr.add(143) as *const u8x32).read_unaligned();
    c102_x = c102.simd_eq(Simd::splat(b'X'));
    c102_m = c102.simd_eq(Simd::splat(b'M'));
    c102_a = c102.simd_eq(Simd::splat(b'A'));
    c102_s = c102.simd_eq(Simd::splat(b'S'));
    c103 = (ptr.add(144) as *const u8x32).read_unaligned();
    c103_x = c103.simd_eq(Simd::splat(b'X'));
    c103_m = c103.simd_eq(Simd::splat(b'M'));
    c103_a = c103.simd_eq(Simd::splat(b'A'));
    c103_s = c103.simd_eq(Simd::splat(b'S'));
    sums -=
        (c100_x & c101_m & c102_a & c103_s).to_int() + (c100_s & c101_a & c102_m & c103_x).to_int();
    c110 = (ptr.add(173) as *const u8x32).read_unaligned();
    c110_x = c110.simd_eq(Simd::splat(b'X'));
    c110_m = c110.simd_eq(Simd::splat(b'M'));
    c110_a = c110.simd_eq(Simd::splat(b'A'));
    c110_s = c110.simd_eq(Simd::splat(b'S'));
    c111 = (ptr.add(174) as *const u8x32).read_unaligned();
    c111_x = c111.simd_eq(Simd::splat(b'X'));
    c111_m = c111.simd_eq(Simd::splat(b'M'));
    c111_a = c111.simd_eq(Simd::splat(b'A'));
    c111_s = c111.simd_eq(Simd::splat(b'S'));
    c112 = (ptr.add(175) as *const u8x32).read_unaligned();
    c112_x = c112.simd_eq(Simd::splat(b'X'));
    c112_m = c112.simd_eq(Simd::splat(b'M'));
    c112_a = c112.simd_eq(Simd::splat(b'A'));
    c112_s = c112.simd_eq(Simd::splat(b'S'));
    c113 = (ptr.add(176) as *const u8x32).read_unaligned();
    c113_x = c113.simd_eq(Simd::splat(b'X'));
    c113_m = c113.simd_eq(Simd::splat(b'M'));
    c113_a = c113.simd_eq(Simd::splat(b'A'));
    c113_s = c113.simd_eq(Simd::splat(b'S'));
    sums -=
        (c110_x & c111_m & c112_a & c113_s).to_int() + (c110_s & c111_a & c112_m & c113_x).to_int();
    c120 = (ptr.add(205) as *const u8x32).read_unaligned();
    c120_x = c120.simd_eq(Simd::splat(b'X'));
    c120_m = c120.simd_eq(Simd::splat(b'M'));
    c120_a = c120.simd_eq(Simd::splat(b'A'));
    c120_s = c120.simd_eq(Simd::splat(b'S'));
    c121 = (ptr.add(206) as *const u8x32).read_unaligned();
    c121_x = c121.simd_eq(Simd::splat(b'X'));
    c121_m = c121.simd_eq(Simd::splat(b'M'));
    c121_a = c121.simd_eq(Simd::splat(b'A'));
    c121_s = c121.simd_eq(Simd::splat(b'S'));
    c122 = (ptr.add(207) as *const u8x32).read_unaligned();
    c122_x = c122.simd_eq(Simd::splat(b'X'));
    c122_m = c122.simd_eq(Simd::splat(b'M'));
    c122_a = c122.simd_eq(Simd::splat(b'A'));
    c122_s = c122.simd_eq(Simd::splat(b'S'));
    c123 = (ptr.add(208) as *const u8x32).read_unaligned();
    c123_x = c123.simd_eq(Simd::splat(b'X'));
    c123_m = c123.simd_eq(Simd::splat(b'M'));
    c123_a = c123.simd_eq(Simd::splat(b'A'));
    c123_s = c123.simd_eq(Simd::splat(b'S'));
    sums -=
        (c120_x & c121_m & c122_a & c123_s).to_int() + (c120_s & c121_a & c122_m & c123_x).to_int();
    c130 = (ptr.add(237) as *const u8x32).read_unaligned();
    c130_x = c130.simd_eq(Simd::splat(b'X'));
    c130_m = c130.simd_eq(Simd::splat(b'M'));
    c130_a = c130.simd_eq(Simd::splat(b'A'));
    c130_s = c130.simd_eq(Simd::splat(b'S'));
    c131 = (ptr.add(238) as *const u8x32).read_unaligned();
    c131_x = c131.simd_eq(Simd::splat(b'X'));
    c131_m = c131.simd_eq(Simd::splat(b'M'));
    c131_a = c131.simd_eq(Simd::splat(b'A'));
    c131_s = c131.simd_eq(Simd::splat(b'S'));
    c132 = (ptr.add(239) as *const u8x32).read_unaligned();
    c132_x = c132.simd_eq(Simd::splat(b'X'));
    c132_m = c132.simd_eq(Simd::splat(b'M'));
    c132_a = c132.simd_eq(Simd::splat(b'A'));
    c132_s = c132.simd_eq(Simd::splat(b'S'));
    c133 = (ptr.add(240) as *const u8x32).read_unaligned();
    c133_x = c133.simd_eq(Simd::splat(b'X'));
    c133_m = c133.simd_eq(Simd::splat(b'M'));
    c133_a = c133.simd_eq(Simd::splat(b'A'));
    c133_s = c133.simd_eq(Simd::splat(b'S'));
    sums -=
        (c130_x & c131_m & c132_a & c133_s).to_int() + (c130_s & c131_a & c132_m & c133_x).to_int();
    c140 = simd_swizzle!(
        (ptr.add(265) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c140_x = c140.simd_eq(Simd::splat(b'X'));
    c140_m = c140.simd_eq(Simd::splat(b'M'));
    c140_a = c140.simd_eq(Simd::splat(b'A'));
    c140_s = c140.simd_eq(Simd::splat(b'S'));
    c141 = simd_swizzle!(
        (ptr.add(265) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c141_x = c141.simd_eq(Simd::splat(b'X'));
    c141_m = c141.simd_eq(Simd::splat(b'M'));
    c141_a = c141.simd_eq(Simd::splat(b'A'));
    c141_s = c141.simd_eq(Simd::splat(b'S'));
    c142 = simd_swizzle!(
        (ptr.add(265) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c142_x = c142.simd_eq(Simd::splat(b'X'));
    c142_m = c142.simd_eq(Simd::splat(b'M'));
    c142_a = c142.simd_eq(Simd::splat(b'A'));
    c142_s = c142.simd_eq(Simd::splat(b'S'));
    c143 = simd_swizzle!(
        (ptr.add(265) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c143_x = c143.simd_eq(Simd::splat(b'X'));
    c143_m = c143.simd_eq(Simd::splat(b'M'));
    c143_a = c143.simd_eq(Simd::splat(b'A'));
    c143_s = c143.simd_eq(Simd::splat(b'S'));
    sums -=
        (c140_x & c141_m & c142_a & c143_s).to_int() + (c140_s & c141_a & c142_m & c143_x).to_int();
    c200 = (ptr.add(282) as *const u8x32).read_unaligned();
    c200_x = c200.simd_eq(Simd::splat(b'X'));
    c200_m = c200.simd_eq(Simd::splat(b'M'));
    c200_a = c200.simd_eq(Simd::splat(b'A'));
    c200_s = c200.simd_eq(Simd::splat(b'S'));
    c201 = (ptr.add(283) as *const u8x32).read_unaligned();
    c201_x = c201.simd_eq(Simd::splat(b'X'));
    c201_m = c201.simd_eq(Simd::splat(b'M'));
    c201_a = c201.simd_eq(Simd::splat(b'A'));
    c201_s = c201.simd_eq(Simd::splat(b'S'));
    c202 = (ptr.add(284) as *const u8x32).read_unaligned();
    c202_x = c202.simd_eq(Simd::splat(b'X'));
    c202_m = c202.simd_eq(Simd::splat(b'M'));
    c202_a = c202.simd_eq(Simd::splat(b'A'));
    c202_s = c202.simd_eq(Simd::splat(b'S'));
    c203 = (ptr.add(285) as *const u8x32).read_unaligned();
    c203_x = c203.simd_eq(Simd::splat(b'X'));
    c203_m = c203.simd_eq(Simd::splat(b'M'));
    c203_a = c203.simd_eq(Simd::splat(b'A'));
    c203_s = c203.simd_eq(Simd::splat(b'S'));
    sums -=
        (c200_x & c201_m & c202_a & c203_s).to_int() + (c200_s & c201_a & c202_m & c203_x).to_int();
    c210 = (ptr.add(314) as *const u8x32).read_unaligned();
    c210_x = c210.simd_eq(Simd::splat(b'X'));
    c210_m = c210.simd_eq(Simd::splat(b'M'));
    c210_a = c210.simd_eq(Simd::splat(b'A'));
    c210_s = c210.simd_eq(Simd::splat(b'S'));
    c211 = (ptr.add(315) as *const u8x32).read_unaligned();
    c211_x = c211.simd_eq(Simd::splat(b'X'));
    c211_m = c211.simd_eq(Simd::splat(b'M'));
    c211_a = c211.simd_eq(Simd::splat(b'A'));
    c211_s = c211.simd_eq(Simd::splat(b'S'));
    c212 = (ptr.add(316) as *const u8x32).read_unaligned();
    c212_x = c212.simd_eq(Simd::splat(b'X'));
    c212_m = c212.simd_eq(Simd::splat(b'M'));
    c212_a = c212.simd_eq(Simd::splat(b'A'));
    c212_s = c212.simd_eq(Simd::splat(b'S'));
    c213 = (ptr.add(317) as *const u8x32).read_unaligned();
    c213_x = c213.simd_eq(Simd::splat(b'X'));
    c213_m = c213.simd_eq(Simd::splat(b'M'));
    c213_a = c213.simd_eq(Simd::splat(b'A'));
    c213_s = c213.simd_eq(Simd::splat(b'S'));
    sums -=
        (c210_x & c211_m & c212_a & c213_s).to_int() + (c210_s & c211_a & c212_m & c213_x).to_int();
    c220 = (ptr.add(346) as *const u8x32).read_unaligned();
    c220_x = c220.simd_eq(Simd::splat(b'X'));
    c220_m = c220.simd_eq(Simd::splat(b'M'));
    c220_a = c220.simd_eq(Simd::splat(b'A'));
    c220_s = c220.simd_eq(Simd::splat(b'S'));
    c221 = (ptr.add(347) as *const u8x32).read_unaligned();
    c221_x = c221.simd_eq(Simd::splat(b'X'));
    c221_m = c221.simd_eq(Simd::splat(b'M'));
    c221_a = c221.simd_eq(Simd::splat(b'A'));
    c221_s = c221.simd_eq(Simd::splat(b'S'));
    c222 = (ptr.add(348) as *const u8x32).read_unaligned();
    c222_x = c222.simd_eq(Simd::splat(b'X'));
    c222_m = c222.simd_eq(Simd::splat(b'M'));
    c222_a = c222.simd_eq(Simd::splat(b'A'));
    c222_s = c222.simd_eq(Simd::splat(b'S'));
    c223 = (ptr.add(349) as *const u8x32).read_unaligned();
    c223_x = c223.simd_eq(Simd::splat(b'X'));
    c223_m = c223.simd_eq(Simd::splat(b'M'));
    c223_a = c223.simd_eq(Simd::splat(b'A'));
    c223_s = c223.simd_eq(Simd::splat(b'S'));
    sums -=
        (c220_x & c221_m & c222_a & c223_s).to_int() + (c220_s & c221_a & c222_m & c223_x).to_int();
    c230 = (ptr.add(378) as *const u8x32).read_unaligned();
    c230_x = c230.simd_eq(Simd::splat(b'X'));
    c230_m = c230.simd_eq(Simd::splat(b'M'));
    c230_a = c230.simd_eq(Simd::splat(b'A'));
    c230_s = c230.simd_eq(Simd::splat(b'S'));
    c231 = (ptr.add(379) as *const u8x32).read_unaligned();
    c231_x = c231.simd_eq(Simd::splat(b'X'));
    c231_m = c231.simd_eq(Simd::splat(b'M'));
    c231_a = c231.simd_eq(Simd::splat(b'A'));
    c231_s = c231.simd_eq(Simd::splat(b'S'));
    c232 = (ptr.add(380) as *const u8x32).read_unaligned();
    c232_x = c232.simd_eq(Simd::splat(b'X'));
    c232_m = c232.simd_eq(Simd::splat(b'M'));
    c232_a = c232.simd_eq(Simd::splat(b'A'));
    c232_s = c232.simd_eq(Simd::splat(b'S'));
    c233 = (ptr.add(381) as *const u8x32).read_unaligned();
    c233_x = c233.simd_eq(Simd::splat(b'X'));
    c233_m = c233.simd_eq(Simd::splat(b'M'));
    c233_a = c233.simd_eq(Simd::splat(b'A'));
    c233_s = c233.simd_eq(Simd::splat(b'S'));
    sums -=
        (c230_x & c231_m & c232_a & c233_s).to_int() + (c230_s & c231_a & c232_m & c233_x).to_int();
    c240 = simd_swizzle!(
        (ptr.add(406) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c240_x = c240.simd_eq(Simd::splat(b'X'));
    c240_m = c240.simd_eq(Simd::splat(b'M'));
    c240_a = c240.simd_eq(Simd::splat(b'A'));
    c240_s = c240.simd_eq(Simd::splat(b'S'));
    c241 = simd_swizzle!(
        (ptr.add(406) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c241_x = c241.simd_eq(Simd::splat(b'X'));
    c241_m = c241.simd_eq(Simd::splat(b'M'));
    c241_a = c241.simd_eq(Simd::splat(b'A'));
    c241_s = c241.simd_eq(Simd::splat(b'S'));
    c242 = simd_swizzle!(
        (ptr.add(406) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c242_x = c242.simd_eq(Simd::splat(b'X'));
    c242_m = c242.simd_eq(Simd::splat(b'M'));
    c242_a = c242.simd_eq(Simd::splat(b'A'));
    c242_s = c242.simd_eq(Simd::splat(b'S'));
    c243 = simd_swizzle!(
        (ptr.add(406) as *const u8x16).read_unaligned(),
        Simd::splat(0),
        [
            7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16, 16, 16
        ]
    );
    c243_x = c243.simd_eq(Simd::splat(b'X'));
    c243_m = c243.simd_eq(Simd::splat(b'M'));
    c243_a = c243.simd_eq(Simd::splat(b'A'));
    c243_s = c243.simd_eq(Simd::splat(b'S'));
    sums -=
        (c240_x & c241_m & c242_a & c243_s).to_int() + (c240_s & c241_a & c242_m & c243_x).to_int();
    ptr = ptr.add(423);
    loop {
        c300 = (ptr.add(0) as *const u8x32).read_unaligned();
        c300_x = c300.simd_eq(Simd::splat(b'X'));
        c300_m = c300.simd_eq(Simd::splat(b'M'));
        c300_a = c300.simd_eq(Simd::splat(b'A'));
        c300_s = c300.simd_eq(Simd::splat(b'S'));
        c301 = (ptr.add(1) as *const u8x32).read_unaligned();
        c301_x = c301.simd_eq(Simd::splat(b'X'));
        c301_m = c301.simd_eq(Simd::splat(b'M'));
        c301_a = c301.simd_eq(Simd::splat(b'A'));
        c301_s = c301.simd_eq(Simd::splat(b'S'));
        c302 = (ptr.add(2) as *const u8x32).read_unaligned();
        c302_x = c302.simd_eq(Simd::splat(b'X'));
        c302_m = c302.simd_eq(Simd::splat(b'M'));
        c302_a = c302.simd_eq(Simd::splat(b'A'));
        c302_s = c302.simd_eq(Simd::splat(b'S'));
        c303 = (ptr.add(3) as *const u8x32).read_unaligned();
        c303_x = c303.simd_eq(Simd::splat(b'X'));
        c303_m = c303.simd_eq(Simd::splat(b'M'));
        c303_a = c303.simd_eq(Simd::splat(b'A'));
        c303_s = c303.simd_eq(Simd::splat(b'S'));
        sums -= (c300_x & c301_m & c302_a & c303_s).to_int()
            + (c300_s & c301_a & c302_m & c303_x).to_int()
            + (c000_x & c101_m & c202_a & c303_s).to_int()
            + (c000_s & c101_a & c202_m & c303_x).to_int()
            + (c000_x & c100_m & c200_a & c300_s).to_int()
            + (c000_s & c100_a & c200_m & c300_x).to_int()
            + (c003_x & c102_m & c201_a & c300_s).to_int()
            + (c003_s & c102_a & c201_m & c300_x).to_int();
        c000 = c100;
        c000_x = c100_x;
        c000_m = c100_m;
        c000_a = c100_a;
        c000_s = c100_s;
        c001 = c101;
        c001_x = c101_x;
        c001_m = c101_m;
        c001_a = c101_a;
        c001_s = c101_s;
        c002 = c102;
        c002_x = c102_x;
        c002_m = c102_m;
        c002_a = c102_a;
        c002_s = c102_s;
        c003 = c103;
        c003_x = c103_x;
        c003_m = c103_m;
        c003_a = c103_a;
        c003_s = c103_s;
        c100 = c200;
        c100_x = c200_x;
        c100_m = c200_m;
        c100_a = c200_a;
        c100_s = c200_s;
        c101 = c201;
        c101_x = c201_x;
        c101_m = c201_m;
        c101_a = c201_a;
        c101_s = c201_s;
        c102 = c202;
        c102_x = c202_x;
        c102_m = c202_m;
        c102_a = c202_a;
        c102_s = c202_s;
        c103 = c203;
        c103_x = c203_x;
        c103_m = c203_m;
        c103_a = c203_a;
        c103_s = c203_s;
        c200 = c300;
        c200_x = c300_x;
        c200_m = c300_m;
        c200_a = c300_a;
        c200_s = c300_s;
        c201 = c301;
        c201_x = c301_x;
        c201_m = c301_m;
        c201_a = c301_a;
        c201_s = c301_s;
        c202 = c302;
        c202_x = c302_x;
        c202_m = c302_m;
        c202_a = c302_a;
        c202_s = c302_s;
        c203 = c303;
        c203_x = c303_x;
        c203_m = c303_m;
        c203_a = c303_a;
        c203_s = c303_s;
        c310 = (ptr.add(32) as *const u8x32).read_unaligned();
        c310_x = c310.simd_eq(Simd::splat(b'X'));
        c310_m = c310.simd_eq(Simd::splat(b'M'));
        c310_a = c310.simd_eq(Simd::splat(b'A'));
        c310_s = c310.simd_eq(Simd::splat(b'S'));
        c311 = (ptr.add(33) as *const u8x32).read_unaligned();
        c311_x = c311.simd_eq(Simd::splat(b'X'));
        c311_m = c311.simd_eq(Simd::splat(b'M'));
        c311_a = c311.simd_eq(Simd::splat(b'A'));
        c311_s = c311.simd_eq(Simd::splat(b'S'));
        c312 = (ptr.add(34) as *const u8x32).read_unaligned();
        c312_x = c312.simd_eq(Simd::splat(b'X'));
        c312_m = c312.simd_eq(Simd::splat(b'M'));
        c312_a = c312.simd_eq(Simd::splat(b'A'));
        c312_s = c312.simd_eq(Simd::splat(b'S'));
        c313 = (ptr.add(35) as *const u8x32).read_unaligned();
        c313_x = c313.simd_eq(Simd::splat(b'X'));
        c313_m = c313.simd_eq(Simd::splat(b'M'));
        c313_a = c313.simd_eq(Simd::splat(b'A'));
        c313_s = c313.simd_eq(Simd::splat(b'S'));
        sums -= (c310_x & c311_m & c312_a & c313_s).to_int()
            + (c310_s & c311_a & c312_m & c313_x).to_int()
            + (c010_x & c111_m & c212_a & c313_s).to_int()
            + (c010_s & c111_a & c212_m & c313_x).to_int()
            + (c010_x & c110_m & c210_a & c310_s).to_int()
            + (c010_s & c110_a & c210_m & c310_x).to_int()
            + (c013_x & c112_m & c211_a & c310_s).to_int()
            + (c013_s & c112_a & c211_m & c310_x).to_int();
        c010 = c110;
        c010_x = c110_x;
        c010_m = c110_m;
        c010_a = c110_a;
        c010_s = c110_s;
        c011 = c111;
        c011_x = c111_x;
        c011_m = c111_m;
        c011_a = c111_a;
        c011_s = c111_s;
        c012 = c112;
        c012_x = c112_x;
        c012_m = c112_m;
        c012_a = c112_a;
        c012_s = c112_s;
        c013 = c113;
        c013_x = c113_x;
        c013_m = c113_m;
        c013_a = c113_a;
        c013_s = c113_s;
        c110 = c210;
        c110_x = c210_x;
        c110_m = c210_m;
        c110_a = c210_a;
        c110_s = c210_s;
        c111 = c211;
        c111_x = c211_x;
        c111_m = c211_m;
        c111_a = c211_a;
        c111_s = c211_s;
        c112 = c212;
        c112_x = c212_x;
        c112_m = c212_m;
        c112_a = c212_a;
        c112_s = c212_s;
        c113 = c213;
        c113_x = c213_x;
        c113_m = c213_m;
        c113_a = c213_a;
        c113_s = c213_s;
        c210 = c310;
        c210_x = c310_x;
        c210_m = c310_m;
        c210_a = c310_a;
        c210_s = c310_s;
        c211 = c311;
        c211_x = c311_x;
        c211_m = c311_m;
        c211_a = c311_a;
        c211_s = c311_s;
        c212 = c312;
        c212_x = c312_x;
        c212_m = c312_m;
        c212_a = c312_a;
        c212_s = c312_s;
        c213 = c313;
        c213_x = c313_x;
        c213_m = c313_m;
        c213_a = c313_a;
        c213_s = c313_s;
        c320 = (ptr.add(64) as *const u8x32).read_unaligned();
        c320_x = c320.simd_eq(Simd::splat(b'X'));
        c320_m = c320.simd_eq(Simd::splat(b'M'));
        c320_a = c320.simd_eq(Simd::splat(b'A'));
        c320_s = c320.simd_eq(Simd::splat(b'S'));
        c321 = (ptr.add(65) as *const u8x32).read_unaligned();
        c321_x = c321.simd_eq(Simd::splat(b'X'));
        c321_m = c321.simd_eq(Simd::splat(b'M'));
        c321_a = c321.simd_eq(Simd::splat(b'A'));
        c321_s = c321.simd_eq(Simd::splat(b'S'));
        c322 = (ptr.add(66) as *const u8x32).read_unaligned();
        c322_x = c322.simd_eq(Simd::splat(b'X'));
        c322_m = c322.simd_eq(Simd::splat(b'M'));
        c322_a = c322.simd_eq(Simd::splat(b'A'));
        c322_s = c322.simd_eq(Simd::splat(b'S'));
        c323 = (ptr.add(67) as *const u8x32).read_unaligned();
        c323_x = c323.simd_eq(Simd::splat(b'X'));
        c323_m = c323.simd_eq(Simd::splat(b'M'));
        c323_a = c323.simd_eq(Simd::splat(b'A'));
        c323_s = c323.simd_eq(Simd::splat(b'S'));
        sums -= (c320_x & c321_m & c322_a & c323_s).to_int()
            + (c320_s & c321_a & c322_m & c323_x).to_int()
            + (c020_x & c121_m & c222_a & c323_s).to_int()
            + (c020_s & c121_a & c222_m & c323_x).to_int()
            + (c020_x & c120_m & c220_a & c320_s).to_int()
            + (c020_s & c120_a & c220_m & c320_x).to_int()
            + (c023_x & c122_m & c221_a & c320_s).to_int()
            + (c023_s & c122_a & c221_m & c320_x).to_int();
        c020 = c120;
        c020_x = c120_x;
        c020_m = c120_m;
        c020_a = c120_a;
        c020_s = c120_s;
        c021 = c121;
        c021_x = c121_x;
        c021_m = c121_m;
        c021_a = c121_a;
        c021_s = c121_s;
        c022 = c122;
        c022_x = c122_x;
        c022_m = c122_m;
        c022_a = c122_a;
        c022_s = c122_s;
        c023 = c123;
        c023_x = c123_x;
        c023_m = c123_m;
        c023_a = c123_a;
        c023_s = c123_s;
        c120 = c220;
        c120_x = c220_x;
        c120_m = c220_m;
        c120_a = c220_a;
        c120_s = c220_s;
        c121 = c221;
        c121_x = c221_x;
        c121_m = c221_m;
        c121_a = c221_a;
        c121_s = c221_s;
        c122 = c222;
        c122_x = c222_x;
        c122_m = c222_m;
        c122_a = c222_a;
        c122_s = c222_s;
        c123 = c223;
        c123_x = c223_x;
        c123_m = c223_m;
        c123_a = c223_a;
        c123_s = c223_s;
        c220 = c320;
        c220_x = c320_x;
        c220_m = c320_m;
        c220_a = c320_a;
        c220_s = c320_s;
        c221 = c321;
        c221_x = c321_x;
        c221_m = c321_m;
        c221_a = c321_a;
        c221_s = c321_s;
        c222 = c322;
        c222_x = c322_x;
        c222_m = c322_m;
        c222_a = c322_a;
        c222_s = c322_s;
        c223 = c323;
        c223_x = c323_x;
        c223_m = c323_m;
        c223_a = c323_a;
        c223_s = c323_s;
        c330 = (ptr.add(96) as *const u8x32).read_unaligned();
        c330_x = c330.simd_eq(Simd::splat(b'X'));
        c330_m = c330.simd_eq(Simd::splat(b'M'));
        c330_a = c330.simd_eq(Simd::splat(b'A'));
        c330_s = c330.simd_eq(Simd::splat(b'S'));
        c331 = (ptr.add(97) as *const u8x32).read_unaligned();
        c331_x = c331.simd_eq(Simd::splat(b'X'));
        c331_m = c331.simd_eq(Simd::splat(b'M'));
        c331_a = c331.simd_eq(Simd::splat(b'A'));
        c331_s = c331.simd_eq(Simd::splat(b'S'));
        c332 = (ptr.add(98) as *const u8x32).read_unaligned();
        c332_x = c332.simd_eq(Simd::splat(b'X'));
        c332_m = c332.simd_eq(Simd::splat(b'M'));
        c332_a = c332.simd_eq(Simd::splat(b'A'));
        c332_s = c332.simd_eq(Simd::splat(b'S'));
        c333 = (ptr.add(99) as *const u8x32).read_unaligned();
        c333_x = c333.simd_eq(Simd::splat(b'X'));
        c333_m = c333.simd_eq(Simd::splat(b'M'));
        c333_a = c333.simd_eq(Simd::splat(b'A'));
        c333_s = c333.simd_eq(Simd::splat(b'S'));
        sums -= (c330_x & c331_m & c332_a & c333_s).to_int()
            + (c330_s & c331_a & c332_m & c333_x).to_int()
            + (c030_x & c131_m & c232_a & c333_s).to_int()
            + (c030_s & c131_a & c232_m & c333_x).to_int()
            + (c030_x & c130_m & c230_a & c330_s).to_int()
            + (c030_s & c130_a & c230_m & c330_x).to_int()
            + (c033_x & c132_m & c231_a & c330_s).to_int()
            + (c033_s & c132_a & c231_m & c330_x).to_int();
        c030 = c130;
        c030_x = c130_x;
        c030_m = c130_m;
        c030_a = c130_a;
        c030_s = c130_s;
        c031 = c131;
        c031_x = c131_x;
        c031_m = c131_m;
        c031_a = c131_a;
        c031_s = c131_s;
        c032 = c132;
        c032_x = c132_x;
        c032_m = c132_m;
        c032_a = c132_a;
        c032_s = c132_s;
        c033 = c133;
        c033_x = c133_x;
        c033_m = c133_m;
        c033_a = c133_a;
        c033_s = c133_s;
        c130 = c230;
        c130_x = c230_x;
        c130_m = c230_m;
        c130_a = c230_a;
        c130_s = c230_s;
        c131 = c231;
        c131_x = c231_x;
        c131_m = c231_m;
        c131_a = c231_a;
        c131_s = c231_s;
        c132 = c232;
        c132_x = c232_x;
        c132_m = c232_m;
        c132_a = c232_a;
        c132_s = c232_s;
        c133 = c233;
        c133_x = c233_x;
        c133_m = c233_m;
        c133_a = c233_a;
        c133_s = c233_s;
        c230 = c330;
        c230_x = c330_x;
        c230_m = c330_m;
        c230_a = c330_a;
        c230_s = c330_s;
        c231 = c331;
        c231_x = c331_x;
        c231_m = c331_m;
        c231_a = c331_a;
        c231_s = c331_s;
        c232 = c332;
        c232_x = c332_x;
        c232_m = c332_m;
        c232_a = c332_a;
        c232_s = c332_s;
        c233 = c333;
        c233_x = c333_x;
        c233_m = c333_m;
        c233_a = c333_a;
        c233_s = c333_s;
        c340 = simd_swizzle!(
            (ptr.add(124) as *const u8x16).read_unaligned(),
            Simd::splat(0),
            [
                4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
                16, 16, 16, 16, 16, 16, 16, 16, 16, 16
            ]
        );
        c340_x = c340.simd_eq(Simd::splat(b'X'));
        c340_m = c340.simd_eq(Simd::splat(b'M'));
        c340_a = c340.simd_eq(Simd::splat(b'A'));
        c340_s = c340.simd_eq(Simd::splat(b'S'));
        c341 = simd_swizzle!(
            (ptr.add(124) as *const u8x16).read_unaligned(),
            Simd::splat(0),
            [
                5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
                16, 16, 16, 16, 16, 16, 16, 16, 16, 16
            ]
        );
        c341_x = c341.simd_eq(Simd::splat(b'X'));
        c341_m = c341.simd_eq(Simd::splat(b'M'));
        c341_a = c341.simd_eq(Simd::splat(b'A'));
        c341_s = c341.simd_eq(Simd::splat(b'S'));
        c342 = simd_swizzle!(
            (ptr.add(124) as *const u8x16).read_unaligned(),
            Simd::splat(0),
            [
                6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
                16, 16, 16, 16, 16, 16, 16, 16, 16, 16
            ]
        );
        c342_x = c342.simd_eq(Simd::splat(b'X'));
        c342_m = c342.simd_eq(Simd::splat(b'M'));
        c342_a = c342.simd_eq(Simd::splat(b'A'));
        c342_s = c342.simd_eq(Simd::splat(b'S'));
        c343 = simd_swizzle!(
            (ptr.add(124) as *const u8x16).read_unaligned(),
            Simd::splat(0),
            [
                7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
                16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16
            ]
        );
        c343_x = c343.simd_eq(Simd::splat(b'X'));
        c343_m = c343.simd_eq(Simd::splat(b'M'));
        c343_a = c343.simd_eq(Simd::splat(b'A'));
        c343_s = c343.simd_eq(Simd::splat(b'S'));
        sums -= (c340_x & c341_m & c342_a & c343_s).to_int()
            + (c340_s & c341_a & c342_m & c343_x).to_int()
            + (c040_x & c141_m & c242_a & c343_s).to_int()
            + (c040_s & c141_a & c242_m & c343_x).to_int()
            + (c040_x & c140_m & c240_a & c340_s).to_int()
            + (c040_s & c140_a & c240_m & c340_x).to_int()
            + (c043_x & c142_m & c241_a & c340_s).to_int()
            + (c043_s & c142_a & c241_m & c340_x).to_int();
        c040 = c140;
        c040_x = c140_x;
        c040_m = c140_m;
        c040_a = c140_a;
        c040_s = c140_s;
        c041 = c141;
        c041_x = c141_x;
        c041_m = c141_m;
        c041_a = c141_a;
        c041_s = c141_s;
        c042 = c142;
        c042_x = c142_x;
        c042_m = c142_m;
        c042_a = c142_a;
        c042_s = c142_s;
        c043 = c143;
        c043_x = c143_x;
        c043_m = c143_m;
        c043_a = c143_a;
        c043_s = c143_s;
        c140 = c240;
        c140_x = c240_x;
        c140_m = c240_m;
        c140_a = c240_a;
        c140_s = c240_s;
        c141 = c241;
        c141_x = c241_x;
        c141_m = c241_m;
        c141_a = c241_a;
        c141_s = c241_s;
        c142 = c242;
        c142_x = c242_x;
        c142_m = c242_m;
        c142_a = c242_a;
        c142_s = c242_s;
        c143 = c243;
        c143_x = c243_x;
        c143_m = c243_m;
        c143_a = c243_a;
        c143_s = c243_s;
        c240 = c340;
        c240_x = c340_x;
        c240_m = c340_m;
        c240_a = c340_a;
        c240_s = c340_s;
        c241 = c341;
        c241_x = c341_x;
        c241_m = c341_m;
        c241_a = c341_a;
        c241_s = c341_s;
        c242 = c342;
        c242_x = c342_x;
        c242_m = c342_m;
        c242_a = c342_a;
        c242_s = c342_s;
        c243 = c343;
        c243_x = c343_x;
        c243_m = c343_m;
        c243_a = c343_a;
        c243_s = c343_s;
        let t: u16x16 = _mm256_maddubs_epi16(sums.into(), u8x32::splat(1).into()).into();
        long_sums += t;
        sums = Simd::splat(0);
        ptr = ptr.add(141);
        if ptr == end {
            break;
        }
    }

    long_sums.reduce_sum() as u32
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &[u8]) -> u32 {
    0
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
        let s = read_to_string("./inputs/4.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/4p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/4p2.txt").unwrap(),
        );
    }
}
