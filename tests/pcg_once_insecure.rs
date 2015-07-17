#![feature(step_trait)] // Because we need Step for ranges.
#![feature(zero_one)] // Because we need One for ranges.

extern crate pcg;
#[macro_use] mod util;
use util::{RngProperties, Round, test_pcg};

#[test]
fn test_pcg8_once_insecure() {
    let properties = &RngProperties {
        period_pow2: 8,
        streams_pow2: 7,
        size_of_rng: 2,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "141 109 152",
            numbers: &[0xea,0x4d,0x8a,0x45,0x6b,0x23,0xcb,0xaa,0xf7,0x63,0xec,0x30,0x39,0xbc],
            again: sca![u8: 0xf7,0x63,0xec,0x30,0x39,0xbc,0x24,0xf4,0x82,0x85,0x8b,0x88,0x28,0x21],
            coins: "HTTHTHHTTHTHTHTTTHTHHHTHHHHTHHHHHTTHHTHTTTTTTTTTHTHHHHHHHTHTHTHTT",
            rolls: &[5,4,1,5,3,2,2,4,6,3,5,6,3,4,1,3,4,2,4,1,2,5,3,4,6,1,1,6,2,1,5,1,3],
            rolls_used: 34,
            cards: "2c 6d 3c 4d Th Js 4s 8d 2d 9c 9h 3h 3d Td Tc 7c 5h 8c 5s 4h Qh 2s \
                    Ks 6h 7d Jh 5d 8s Qs As Ah 7h Ac 5c 8h Ts 9d Kd Kc Qd Ad 6s 2h Jd \
                    Kh 6c Jc 4c Qc 3s 7s 9s",
        },
        Round {
            dump: "141 109 34",
            numbers: &[0x98,0xfd,0x95,0x79,0xfc,0xf6,0x73,0x64,0x7d,0xf3,0x8e,0xa6,0x51,0xf1],
            again: sca![u8: 0x7d,0xf3,0x8e,0xa6,0x51,0xf1,0x1f,0x4f,0x62,0x29,0x2e,0x01,0x05,0x72],
            coins: "THTTHTTTTHTHTTTHHHHTTHHTTTTTHTHTHTTTHHTHHTHHHTHTTTHTHHTHTHTHTHTHH",
            rolls: &[6,6,3,2,4,3,1,4,3,1,5,5,2,2,5,5,4,4,3,2,1,6,4,5,3,6,5,4,5,2,5,1,3],
            rolls_used: 34,
            cards: "Th 3d 5h 9h Kd 7s Ad 7c 2h Qs 6c Ac 7d Jd 4c Tc Ah Ts 2c As 4h 9c \
                    6d Kh 9d 5d 3c Td 3s Jc 8h 2d 7h Qc 8c 6h 6s Kc 8s Qh 3h 4d Qd Ks \
                    4s 5s 8d 2s Jh 5c Js 9s",
        },
        Round {
            dump: "141 109 43",
            numbers: &[0xfa,0xe0,0x07,0xaf,0x5d,0xc5,0x44,0xe8,0x89,0x68,0xa5,0xa2,0x2c,0x81],
            again: sca![u8: 0x89,0x68,0xa5,0xa2,0x2c,0x81,0xe5,0xcf,0x54,0xb5,0x52,0x3e,0xb1,0xd1],
            coins: "TTHHTTTTHHHTHTHHTHHTHTTHHHTHHTHHTTTTTTHHHTTHTTHHHTTTHTTHTHHTTTHHH",
            rolls: &[1,1,2,5,6,4,5,5,4,2,2,2,3,6,5,6,1,1,4,3,3,4,5,1,3,1,2,3,2,1,1,5,6],
            rolls_used: 34,
            cards: "6d 3d 4d 8d Ts 9d 4h 5h Js Kc Tc 2c Ah 9h Kd Jc As 5s Kh Ac 2h Ks \
                    4s Ad 7c Qs Qh 7s 7h 8c 7d 9s Td 3h 6c 2s Th 5d 4c 3s 9c 5c Jd Jh \
                    8s 8h 2d Qc Qd 3c 6h 6s",
        },
        Round {
            dump: "141 109 108",
            numbers: &[0x39,0xbc,0x24,0xf4,0x82,0x85,0x8b,0x88,0x28,0x21,0x2d,0x26,0x00,0x61],
            again: sca![u8: 0x28,0x21,0x2d,0x26,0x00,0x61,0xba,0x77,0xbd,0x46,0x8c,0x71,0xdc,0x87],
            coins: "THTTTHTHHHTHHHHTHHHHHTTHHTHTTTTTTTTTHTHHHHHHHTHTHTHTTTHTHTTHHHHTT",
            rolls: &[6,3,4,1,3,4,2,4,1,2,5,3,4,6,1,1,6,2,1,5,1,3,6,4,6,1,2,3,2,4,3,6,6],
            rolls_used: 33,
            cards: "Ts Qd 7c Tc 8h 2d 7d 3s 2h 6h 6d Qs 5s Ad Qh 8s 3d Js Ac Kh Th 8d \
                    Kc 2c 9c 5c Td Jc Jd Kd 8c 3c 6s 4d 3h 9d 4s Ks 6c Qc 9h 5h 7s 2s \
                    4c As 7h 9s 5d Ah 4h Jh",
        },
        Round {
            dump: "141 109 77",
            numbers: &[0xa6,0x51,0xf1,0x1f,0x4f,0x62,0x29,0x2e,0x01,0x05,0x72,0xe4,0xdb,0xc8],
            again: sca![u8: 0x01,0x05,0x72,0xe4,0xdb,0xc8,0x86,0x4b,0xa0,0x1e,0x9e,0xb4,0xa9,0xb0],
            coins: "HTTTHHHHTTHHTTTTTHTHTHTTTHHTHHTHHHTHTTTHTHHTHTHTHTHTHHHHTHHTTHTTT",
            rolls: &[5,2,2,5,5,4,4,3,2,1,6,4,5,3,6,5,4,5,2,5,1,3,4,5,4,2,2,5,6,4,6,4,3],
            rolls_used: 34,
            cards: "Qs Jc Kd Th 8c 7d Qc Qd 4c 2c 2s 9h 9s 5s Jd 2h As 8s 5d Td 7c Ts \
                    9c Js 7h 5c 4s Ac 6c 6s 9d Tc 3h Ad 3d 8h Jh Qh 4h 4d Kh 2d 5h 6d \
                    8d 6h Ks 7s Ah 3c 3s Kc",
        },
    ];

    test_pcg(properties, rounds, |state, stream| pcg::Pcg8OnceInsecure::with_stream(state as u8, stream as u8))
}
