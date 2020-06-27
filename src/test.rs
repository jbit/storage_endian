use super::*;

use std::format;

use std::mem::transmute;

const DATA_128: [u8; 16] = [
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
];
const BE_U128: u128 = 0xa0_a1_a2_a3_a4_a5_a6_a7_a8_a9_aa_ab_ac_ad_ae_af;
const BE_U128_LHEX: &str = "a0a1a2a3a4a5a6a7a8a9aaabacadaeaf";
const BE_U128_UHEX: &str = "A0A1A2A3A4A5A6A7A8A9AAABACADAEAF";
const BE_U128_DEC: &str = "213515737835312729685031101235272199855";
const BE_U128_OCT: &str = "2405032124351122646517242515252565453327257";
const BE_U128_BIN: &str = "10100000101000011010001010100011101001001010010110100110101001111010100010101001101010101010101110101100101011011010111010101111";
const LE_U128: u128 = 0xaf_ae_ad_ac_ab_aa_a9_a8_a7_a6_a5_a4_a3_a2_a1_a0;
const LE_U128_LHEX: &str = "afaeadacabaaa9a8a7a6a5a4a3a2a1a0";
const LE_U128_UHEX: &str = "AFAEADACABAAA9A8A7A6A5A4A3A2A1A0";
const LE_U128_DEC: &str = "233521881452978977217833579116266430880";
const LE_U128_OCT: &str = "2575352665452725251521236465132224350520640";
const LE_U128_BIN: &str = "10101111101011101010110110101100101010111010101010101001101010001010011110100110101001011010010010100011101000101010000110100000";

const DATA_64: [u8; 8] = [0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7];
const BE_U64: u64 = 0xa0_a1_a2_a3_a4_a5_a6_a7;
const BE_U64_LHEX: &str = "a0a1a2a3a4a5a6a7";
const BE_U64_UHEX: &str = "A0A1A2A3A4A5A6A7";
const BE_U64_DEC: &str = "11574711341044573863";
const BE_U64_OCT: &str = "1202415052164451323247";
const BE_U64_BIN: &str = "1010000010100001101000101010001110100100101001011010011010100111";
const LE_U64: u64 = 0xa7_a6_a5_a4_a3_a2_a1_a0;
const LE_U64_LHEX: &str = "a7a6a5a4a3a2a1a0";
const LE_U64_UHEX: &str = "A7A6A5A4A3A2A1A0";
const LE_U64_DEC: &str = "12080525177006498208";
const LE_U64_OCT: &str = "1236465132224350520640";
const LE_U64_BIN: &str = "1010011110100110101001011010010010100011101000101010000110100000";

const DATA_32: [u8; 4] = [0xa0, 0xa1, 0xa2, 0xa3];
const BE_U32: u32 = 0xa0_a1_a2_a3;
const BE_U32_LHEX: &str = "a0a1a2a3";
const BE_U32_UHEX: &str = "A0A1A2A3";
const BE_U32_DEC: &str = "2694947491";
const BE_U32_OCT: &str = "24050321243";
const BE_U32_BIN: &str = "10100000101000011010001010100011";
const LE_U32: u32 = 0xa3_a2_a1_a0;
const LE_U32_LHEX: &str = "a3a2a1a0";
const LE_U32_UHEX: &str = "A3A2A1A0";
const LE_U32_DEC: &str = "2745344416";
const LE_U32_OCT: &str = "24350520640";
const LE_U32_BIN: &str = "10100011101000101010000110100000";

const DATA_16: [u8; 2] = [0xa0, 0xa1];
const BE_U16: u16 = 0xa0_a1;
const BE_U16_LHEX: &str = "a0a1";
const BE_U16_UHEX: &str = "A0A1";
const BE_U16_DEC: &str = "41121";
const BE_U16_OCT: &str = "120241";
const BE_U16_BIN: &str = "1010000010100001";
const LE_U16: u16 = 0xa1_a0;
const LE_U16_LHEX: &str = "a1a0";
const LE_U16_UHEX: &str = "A1A0";
const LE_U16_DEC: &str = "41376";
const LE_U16_OCT: &str = "120640";
const LE_U16_BIN: &str = "1010000110100000";

const DATA_8: [u8; 1] = [0xa0];
const BE_U8: u8 = 0xa0;
const BE_U8_LHEX: &str = "a0";
const BE_U8_UHEX: &str = "A0";
const BE_U8_DEC: &str = "160";
const BE_U8_OCT: &str = "240";
const BE_U8_BIN: &str = "10100000";
const LE_U8: u8 = 0xa0;
const LE_U8_LHEX: &str = "a0";
const LE_U8_UHEX: &str = "A0";
const LE_U8_DEC: &str = "160";
const LE_U8_OCT: &str = "240";
const LE_U8_BIN: &str = "10100000";

#[test]
fn big_endian_format() {
    let value: BEu128 = unsafe { transmute(DATA_128) };
    assert_eq!(value, BE_U128);
    assert_eq!(format!("{:x}", value), BE_U128_LHEX);
    assert_eq!(format!("{:X}", value), BE_U128_UHEX);
    assert_eq!(format!("{}", value), BE_U128_DEC);
    assert_eq!(format!("{:?}", value), BE_U128_DEC);
    assert_eq!(format!("{:o}", value), BE_U128_OCT);
    assert_eq!(format!("{:b}", value), BE_U128_BIN);

    let value: BEu64 = unsafe { transmute(DATA_64) };
    assert_eq!(value, BE_U64);
    assert_eq!(format!("{:x}", value), BE_U64_LHEX);
    assert_eq!(format!("{:X}", value), BE_U64_UHEX);
    assert_eq!(format!("{}", value), BE_U64_DEC);
    assert_eq!(format!("{:?}", value), BE_U64_DEC);
    assert_eq!(format!("{:o}", value), BE_U64_OCT);
    assert_eq!(format!("{:b}", value), BE_U64_BIN);

    let value: BEu32 = unsafe { transmute(DATA_32) };
    assert_eq!(value, BE_U32);
    assert_eq!(format!("{:x}", value), BE_U32_LHEX);
    assert_eq!(format!("{:X}", value), BE_U32_UHEX);
    assert_eq!(format!("{}", value), BE_U32_DEC);
    assert_eq!(format!("{:?}", value), BE_U32_DEC);
    assert_eq!(format!("{:o}", value), BE_U32_OCT);
    assert_eq!(format!("{:b}", value), BE_U32_BIN);

    let value: BEu16 = unsafe { transmute(DATA_16) };
    assert_eq!(value, BE_U16);
    assert_eq!(format!("{:x}", value), BE_U16_LHEX);
    assert_eq!(format!("{:X}", value), BE_U16_UHEX);
    assert_eq!(format!("{}", value), BE_U16_DEC);
    assert_eq!(format!("{:?}", value), BE_U16_DEC);
    assert_eq!(format!("{:o}", value), BE_U16_OCT);
    assert_eq!(format!("{:b}", value), BE_U16_BIN);

    let value: BEu8 = unsafe { transmute(DATA_8) };
    assert_eq!(value, BE_U8);
    assert_eq!(format!("{:x}", value), BE_U8_LHEX);
    assert_eq!(format!("{:X}", value), BE_U8_UHEX);
    assert_eq!(format!("{}", value), BE_U8_DEC);
    assert_eq!(format!("{:?}", value), BE_U8_DEC);
    assert_eq!(format!("{:o}", value), BE_U8_OCT);
    assert_eq!(format!("{:b}", value), BE_U8_BIN);
}

#[test]
fn little_endian_format() {
    let value: LEu128 = unsafe { transmute(DATA_128) };
    assert_eq!(value, LE_U128);
    assert_eq!(format!("{:x}", value), LE_U128_LHEX);
    assert_eq!(format!("{:X}", value), LE_U128_UHEX);
    assert_eq!(format!("{}", value), LE_U128_DEC);
    assert_eq!(format!("{:?}", value), LE_U128_DEC);
    assert_eq!(format!("{:o}", value), LE_U128_OCT);
    assert_eq!(format!("{:b}", value), LE_U128_BIN);

    let value: LEu64 = unsafe { transmute(DATA_64) };
    assert_eq!(value, LE_U64);
    assert_eq!(format!("{:x}", value), LE_U64_LHEX);
    assert_eq!(format!("{:X}", value), LE_U64_UHEX);
    assert_eq!(format!("{}", value), LE_U64_DEC);
    assert_eq!(format!("{:?}", value), LE_U64_DEC);
    assert_eq!(format!("{:o}", value), LE_U64_OCT);
    assert_eq!(format!("{:b}", value), LE_U64_BIN);

    let value: LEu32 = unsafe { transmute(DATA_32) };
    assert_eq!(value, LE_U32);
    assert_eq!(format!("{:x}", value), LE_U32_LHEX);
    assert_eq!(format!("{:X}", value), LE_U32_UHEX);
    assert_eq!(format!("{}", value), LE_U32_DEC);
    assert_eq!(format!("{:?}", value), LE_U32_DEC);
    assert_eq!(format!("{:o}", value), LE_U32_OCT);
    assert_eq!(format!("{:b}", value), LE_U32_BIN);

    let value: LEu16 = unsafe { transmute(DATA_16) };
    assert_eq!(value, LE_U16);
    assert_eq!(format!("{:x}", value), LE_U16_LHEX);
    assert_eq!(format!("{:X}", value), LE_U16_UHEX);
    assert_eq!(format!("{}", value), LE_U16_DEC);
    assert_eq!(format!("{:?}", value), LE_U16_DEC);
    assert_eq!(format!("{:o}", value), LE_U16_OCT);
    assert_eq!(format!("{:b}", value), LE_U16_BIN);

    let value: LEu8 = unsafe { transmute(DATA_8) };
    assert_eq!(value, LE_U8);
    assert_eq!(format!("{:x}", value), LE_U8_LHEX);
    assert_eq!(format!("{:X}", value), LE_U8_UHEX);
    assert_eq!(format!("{}", value), LE_U8_DEC);
    assert_eq!(format!("{:?}", value), LE_U8_DEC);
    assert_eq!(format!("{:o}", value), LE_U8_OCT);
    assert_eq!(format!("{:b}", value), LE_U8_BIN);
}

#[test]
fn example() {
    #[repr(C)]
    struct Data {
        magic: BEu32,
        version: BEu32,
        size: BEu64,
        thing: BEu64,
    }
    impl Data {
        pub const SIZE: usize = core::mem::size_of::<Self>();
        pub const MAGIC: u32 = 0x1337_beef;

        fn handle_thing(thing: u64) {
            // ...
        }

        pub fn from_bytes(data: [u8; Self::SIZE]) -> Self {
            let mut data: Self = unsafe { core::mem::transmute(data) };

            assert_eq!(data.magic, Self::MAGIC);
            assert_eq!((data.version >> 16) & 0xff, 0x01);
            assert!(data.size >= Self::SIZE as u64);
            Self::handle_thing(data.thing.into());

            data
        }
    }

    #[rustfmt::skip]
    Data::from_bytes([
        /* magic: */ 0x13, 0x37, 0xbe, 0xef, 
        /* version: */ 0x00, 0x01, 0x00, 0x00,
        /* size: */ 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        /* thing: */ 0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd,
    ]);
}
