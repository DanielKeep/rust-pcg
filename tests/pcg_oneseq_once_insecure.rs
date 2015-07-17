#![feature(step_trait)] // Because we need Step for ranges.
#![feature(zero_one)] // Because we need One for ranges.

extern crate pcg;
#[macro_use] mod util;
use util::{RngProperties, Round, test_pcg};

#[test]
fn test_pcg8_oneseq_once_insecure() {
    let properties = &RngProperties {
        period_pow2: 8,
        streams_pow2: 0,
        size_of_rng: 1,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "141 77 216",
            numbers: &[0x2e,0x44,0x2f,0x91,0x50,0x84,0xcb,0x60,0x4b,0xe5,0x5f,0x97,0x0f,0x58],
            again: sca![u8: 0x4b,0xe5,0x5f,0x97,0x0f,0x58,0x24,0xfd,0x9c,0x42,0x6c,0xd7,0x25,0xc2],
            coins: "HHTHTTTHTHHTTTTHTTHHHHHTHHHTTTHHHTHHHTTTTTTHHHTTHTHTTTHHHHTHTHTHT",
            rolls: &[5,5,6,6,4,1,2,4,5,6,6,3,1,3,5,5,3,5,5,2,3,5,4,1,2,1,2,5,5,3,1,4,5],
            rolls_used: 36,
            cards: "6s Jc 9h Kc 4h 4c 4s As Ah 4d Ks Js 6d Qc 5h 9d 3c 5c 9c 6h 7h 3d \
                    8s 5d 6c 2c Ad 9s Ts 2s 8c Kd Qs Kh 7s 2h 3h 8h Tc Th Td Qd Ac 2d \
                    8d Qh 5s Jd 7c Jh 3s 7d",
        },
        Round {
            dump: "141 77 85",
            numbers: &[0x85,0x31,0x8f,0xae,0xda,0x7d,0x34,0x1a,0x61,0xff,0x78,0x7a,0x10,0x62],
            again: sca![u8: 0x61,0xff,0x78,0x7a,0x10,0x62,0x5d,0x6f,0x87,0x94,0xd0,0x6b,0xb7,0xc8],
            coins: "THHHHHTTHHTHTTTHTHHHHHHTHHTTHTHTTHHHTTHHHHTTTHHHHTTHHHTTHTTHHTTHT",
            rolls: &[4,2,6,2,4,5,1,2,1,1,1,6,2,3,4,4,3,6,1,1,3,4,3,6,6,3,3,5,3,6,3,3,2],
            rolls_used: 33,
            cards: "Ac 6d Ah 6c Kh 2s As 8c 5h 8s 9h Kc Th 2h 7d 8d 7c Qs 3h Js 3d 2c \
                    7h 6s 5d Jd Jc 5c Ad Ts Qc 2d 7s 4h Qd Ks Kd 4d 4s 9c 6h 8h Tc Jh \
                    Td 3c 3s Qh 9d 9s 4c 5s",
        },
        Round {
            dump: "141 77 242",
            numbers: &[0xaf,0xed,0xdc,0x01,0x83,0x17,0x02,0xce,0xa2,0xaa,0x5e,0xa0,0xd4,0xee],
            again: sca![u8: 0xa2,0xaa,0x5e,0xa0,0xd4,0xee,0x3a,0x2b,0x3e,0xf4,0x9f,0x3c,0x0d,0x7e],
            coins: "HTTTTHTTHHTHTTTTTHTHHHTHTTTHHTHTHTTTHHTHTHTTHHTTHHTTHHHTHHTTHHHHT",
            rolls: &[3,6,5,3,2,4,1,3,5,3,4,4,4,5,5,6,4,3,3,6,4,4,2,4,5,3,4,4,3,6,1,5,5],
            rolls_used: 33,
            cards: "Js 3d Ad 6c 7d 9d Jc Kc 9s 6s As Th 4c 2s Jh Ah 5d 3s 3h 9c Kh Ac \
                    Jd 8d 8s 7s Td 2d 2h 2c 8c 3c 7c 4d Tc 5c 6h Ks 7h 4h Qs Qh 4s Ts \
                    Kd 8h 9h Qc 5h 5s 6d Qd",
        },
        Round {
            dump: "141 77 44",
            numbers: &[0x0f,0x58,0x24,0xfd,0x9c,0x42,0x6c,0xd7,0x25,0xc2,0x2d,0xf3,0x08,0xbf],
            again: sca![u8: 0x25,0xc2,0x2d,0xf3,0x08,0xbf,0x48,0xc6,0xe0,0xe1,0x8c,0x29,0xb9,0x56],
            coins: "TTTHTTHHHHHTHHHTTTHHHTHHHTTTTTTHHHTTHTHTTTHHHHTHTHTHTTTTHHHTHHTHH",
            rolls: &[6,3,1,3,5,5,3,5,5,2,3,5,4,1,2,1,2,5,5,3,1,4,5,1,2,2,3,6,1,3,1,3,1],
            rolls_used: 34,
            cards: "8s 7s 9h 3h 4c Ts 6h 6s Jd 8h Qc 4d Th 7d 2s 5s 3d Jh Ad As 8d 9s \
                    2h Kh 9c 2d 6c Qs Kc Qd Qh 8c 4s Js 3s 7h 5h 5d 6d 9d 7c Jc 4h Td \
                    Ah Ks Ac 5c 2c Tc Kd 3c",
        },
        Round {
            dump: "141 77 160",
            numbers: &[0x1a,0x61,0xff,0x78,0x7a,0x10,0x62,0x5d,0x6f,0x87,0x94,0xd0,0x6b,0xb7],
            again: sca![u8: 0x6f,0x87,0x94,0xd0,0x6b,0xb7,0xc8,0x2c,0xb3,0x75,0xc3,0xd3,0x39,0x76],
            coins: "THHTHTTTHTHHHHHHTHHTTHTHTTHHHTTHHHHTTTHHHHTTHHHTTHTTHHTTHTHHHHHTT",
            rolls: &[2,1,1,1,6,2,3,4,4,3,6,1,1,3,4,3,6,6,3,3,5,3,6,3,3,2,6,2,2,2,3,4,2],
            rolls_used: 33,
            cards: "3s 9s Kc 7d 3c 5h 9d 6c 7c 8d Ah Kh Qs 7s 8h Th 8c 6s Tc 7h Jh 2s \
                    5s Jc Qc 2d As 3d Js 6h 6d 3h Qh Ac Td 5d Jd Kd Ks 9h 2h Ts 2c 4s \
                    5c Qd 9c 4c 4d Ad 4h 8s",
        },
    ];

    test_pcg(properties, rounds, |state, _| pcg::Pcg8OneSeqOnceInsecure::with_state(state as u8))
}

#[test]
fn test_pcg16_oneseq_once_insecure() {
    let properties = &RngProperties {
        period_pow2: 16,
        streams_pow2: 0,
        size_of_rng: 2,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "12829 47989 2680",
            numbers: &[0x7f90,0x7f82,0x54f7,0xe8c8,0x9444,0xba1a,0xb7fb,0x2167,0x39dd,0xb0f2],
            again: sca![u16: 0x9444,0xba1a,0xb7fb,0x2167,0x39dd,0xb0f2,0x71e2,0xa94c,0x05c4,0x8119],
            coins: "THHHTTTHHHHHHTHHHTTTTTHHHHTHHTHHTTTTHTHTTHHHHTHTHTHTTHTHTHHTTHTHH",
            rolls: &[1,1,1,6,4,2,5,2,3,5,6,5,1,2,5,2,2,5,3,2,5,4,4,3,5,4,3,5,3,2,3,4,4],
            rolls_used: 33,
            cards: "3s 5s 2h 9c 4d Qc Ks Ad 3h 8s 8d Td Ts 3d 4s As Js 8c 7c Qd Tc 5c
                    Jd 6h Th Qh 4c 2c Ac 6s 9d 7s 3c Kc 9h Qs 7h 9s Jh 8h 2d 6c 7d Jc
                    5d Ah 6d Kh 4h Kd 2s 5h",
        },
        Round {
            dump: "12829 47989 9803",
            numbers: &[0x19d4,0xadde,0xdce3,0xe222,0x7199,0xe4aa,0x9e56,0xe264,0x238e,0xe5b9],
            again: sca![u16: 0x7199,0xe4aa,0x9e56,0xe264,0x238e,0xe5b9,0x6d73,0x83e3,0x281e,0x91cc],
            coins: "HTTHHTHTHTTHHHTHHHHHTHTHHTHTTTTTHHHHHHHHHHTTTTTTHTTHTHHHHTHTTTTHH",
            rolls: &[4,3,1,1,5,3,5,5,4,1,3,1,1,1,4,3,2,2,4,1,5,3,1,6,4,6,4,6,6,2,5,2,6],
            rolls_used: 33,
            cards: "4s 8d Tc Kd 9h 7h Qc 4h 7c Kh 5d 4c 5h Kc 7s Ac Js Ts Ks Jd Th 8s
                    4d 3d 5s 8c 3c Jc Td 7d Ad 3h Qh 9d 2s 6h 6s 2h 2d 8h As 9c 6d Qd
                    Ah 2c 3s 6c 5c 9s Qs Jh",
        },
        Round {
            dump: "12829 47989 47274",
            numbers: &[0x0b43,0x96d8,0x6d84,0xfb69,0xfdbc,0x053b,0x64bc,0x145d,0xd617,0xaaed],
            again: sca![u16: 0xfdbc,0x053b,0x64bc,0x145d,0xd617,0xaaed,0x4438,0x09f3,0x90da,0x0ecc],
            coins: "HTTTHTHHHTTTHTHHTHHHHTHTHHTHHTTHTTTHTTTHHHTTHHHHTTTTHHHTHTTHTHTTH",
            rolls: &[2,3,3,5,3,4,6,2,2,3,4,6,5,2,1,2,3,1,4,1,3,4,1,6,4,2,2,6,3,3,5,6,2],
            rolls_used: 33,
            cards: "4c Jd 2s 2c 8s Td 3d Ac 9c 9d Th 6s Qd 4d 6d 7s 9s Jh 8h Kh 8d 7h
                    5h 3h 4h 7d Tc 2h Qc Qh Ts 7c 2d 5c 3c Jc Ks 5d As 9h Ad 8c Qs Ah
                    6h 3s 5s 6c Kc 4s Kd Js",
        },
        Round {
            dump: "12829 47989 59589",
            numbers: &[0x8e36,0x32bf,0x52af,0x7420,0x6630,0x6d3b,0x9052,0xf56d,0xdc76,0x018d],
            again: sca![u16: 0x6630,0x6d3b,0x9052,0xf56d,0xdc76,0x018d,0xfdee,0xbb1e,0x9892,0xa779],
            coins: "THHTTTTTHTTHHHTTTHHTHHTHTHTTHHHTHHTHTTTTHTTHTTHHHHHTHHTTHHTHHTHTT",
            rolls: &[2,4,6,4,3,6,4,1,5,5,1,5,2,3,3,1,2,1,1,4,6,3,1,3,6,3,1,4,1,5,1,2,1],
            rolls_used: 33,
            cards: "Td 6s Qh 3c Ts 7d Js 5h 3d 6h 7h Ac 9s Th 2d Ah 4d As 8h 8s 8c 4h
                    8d Jd 5s Kd 7s Tc Ks 6d Jh Jc 7c Qs 5d 9c 9h 5c 2h Kc 2s 3s 3h 6c
                    Qc Qd 4c 4s Ad 2c 9d Kh",
        },
        Round {
            dump: "12829 47989 28300",
            numbers: &[0x50cc,0xc1f3,0x63d4,0x436e,0xabb8,0x1983,0xe595,0x6424,0xf0f6,0x7a55],
            again: sca![u16: 0xabb8,0x1983,0xe595,0x6424,0xf0f6,0x7a55,0xf5d1,0xa53f,0x3e76,0x1bd5],
            coins: "TTTHTHHTHTHTTTTHTTHTTHTTTHTTTTTTHHHHTTHHTTHTTHHHHHTTHHHTTTHTHTHHT",
            rolls: &[1,1,6,1,1,1,4,4,2,4,6,1,6,2,4,1,2,6,2,2,5,6,2,4,2,6,1,1,1,6,5,6,1],
            rolls_used: 33,
            cards: "Th 6c 2d 7s Ad 9d 8s 8c Jc Kc Jh 6s Qc Ac Tc 9c 4d 2s 3d 2h 4h 3c
                    Qh 2c As 5d Qs 7h Kh Jd 4c 5c 8h 9s Qd 5h 9h Td 3h 5s Ks 3s Ts Ah
                    6h Js Kd 8d 7c 6d 4s 7d",
        },
    ];

    test_pcg(properties, rounds, |state, _| pcg::Pcg16OneSeqOnceInsecure::with_state(state as u16))
}

#[test]
fn test_pcg32_oneseq_once_insecure() {
    let properties = &RngProperties {
        period_pow2: 32,
        streams_pow2: 0,
        size_of_rng: 4,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "747796405 2891336453 2535083072",
            numbers: &[0x256b5357,0xa5efad32,0x170b7830,0x334a5b22,0x3de5c680,0x9b47b7b3],
            again: None,
            coins: "HTHTHTHHHTTHTTTTTTHHTTTHHTTTHHTTHHTHTTHHTHHTTTTTHTTTHHHHHHHHTTTTT",
            rolls: &[5,5,5,1,5,6,5,1,3,4,5,3,4,5,4,5,2,5,6,4,5,4,4,5,5,6,4,3,6,3,5,4,5],
            rolls_used: 33,
            cards: "3c 5c Kc 6s Qh 7s Jh 4d 3s 5d 9h Th Qs 7h 4c 7c Qd 2d 3h 5h 2h 6c
                    6d Js Jd 9d 8s 9s 9c Qc Kh 8d 8c 2s Tc 4s Ac 2c Jc Ks As Ah 6h Ad
                    Ts 7d 3d 8h 5s Kd 4h Td",
        },
        Round {
            dump: "747796405 2891336453 2049603923",
            numbers: &[0xd3ea68f3,0x004a141a,0x08de95da,0xe6f4f6ad,0x1023b258,0x0fdabaa1],
            again: None,
            coins: "HTHHTTHHHTTHTTTHTTTHHTHHHHHHHHTHTTHHHHTTTHTTHHHHTTTHHTTHHTTTHHTTH",
            rolls: &[2,3,6,1,6,4,2,2,3,1,6,4,3,6,1,2,4,6,4,5,2,2,2,5,1,3,6,2,3,2,2,5,3],
            rolls_used: 33,
            cards: "6c Kc 5d Ac Tc 3c 7h Qh 7c 2c Kd 8c 2h Qs Qc 2s 6s Ts Jc 4h Ah 5c
                    Qd 8d 4d Th 3d 7s 5s Jd 4c 9h 8h 6d 9c 9s 3s Td Js Kh 9d As 6h 3h
                    2d Ks 4s 7d Jh Ad 5h 8s",
        },
        Round {
            dump: "747796405 2891336453 2710303786",
            numbers: &[0x6a106195,0xe06d41b2,0xbfd78624,0xe0ef944f,0x57571028,0x10aae72d],
            again: None,
            coins: "HHTTHHTHTTTHHTHTHTHHHTTHHHTTHTTHTTHTTHTTHHTTTHHHTHTHTHHHHTHHTHTHH",
            rolls: &[4,6,1,3,1,6,6,1,4,5,1,5,5,6,2,4,6,5,2,5,4,6,4,3,5,2,3,6,6,3,1,2,5],
            rolls_used: 33,
            cards: "4d Jc 6d 2s 8c 7d Th 6h 5s 3c 3d Qd Ad 4h 2c 7s Tc 4s 3s Td 6s 9c
                    2d 7c 8d 8h Jh Ts 4c 2h 5c 5h Ac 8s Qs Kh Kc 6c Qc 9h 9s 5d Kd Js
                    Qh 3h 7h Ah As Jd 9d Ks",
        },
        Round {
            dump: "747796405 2891336453 1187600501",
            numbers: &[0xdde49a52,0x79306ca7,0x2bb1673c,0xfde1d6ff,0x0b261fe8,0xe866fced],
            again: None,
            coins: "HHHHTTTTTTTTHTHTHHTTTHHTHTHHHHHTTTTTTTHTTHTTHTTHHHHHHTHTHTHTHHTTT",
            rolls: &[4,1,4,1,2,6,5,5,5,5,1,3,6,4,5,4,6,1,1,5,5,3,6,1,4,1,6,5,1,4,6,3,2],
            rolls_used: 33,
            cards: "Td 7d 3h 2c 5s 6d Ac 8s Kc 5c 4s Qd 2s Kd As 6c 2d Kh 9c 3d 5d 3s
                    Jd 8c 7s 4d 4h Qc 5h Js 7c 9s Ts Qh Ks 6s Th 8d 3c Tc 8h 9h Ad Jh
                    Jc 9d 7h 2h Ah 6h 4c Qs",
        },
        Round {
            dump: "747796405 2891336453 4035423268",
            numbers: &[0x4253371d,0xcc6b3679,0xb8d7cd7d,0x9e7e0310,0xb1ee5e37,0x6cbff1d2],
            again: None,
            coins: "HHHTHHHTTHTTHTHHTHHTTHTTTHHTHHTTHHHTHHTTHTHHTTTTHTTHHHTTHHTHTTHTH",
            rolls: &[2,2,3,2,1,4,4,1,2,4,6,3,2,5,5,4,1,2,2,2,3,3,2,2,6,4,6,4,5,4,2,4,5],
            rolls_used: 33,
            cards: "Kh 4d 8d 5h 4c 8s 3s Qc Js Td Jc 6c 5d 8h 9s 3h Kc Ac Tc 8c 6s 7h
                    Jd 7c Ad Qd Jh 9d As 2c 6d 4h Kd 4s Qs 7s Qh 9h 3d 6h Ts Ks 7d 5c
                    5s 9c 3c 2s 2h 2d Th Ah",
        },
    ];

    test_pcg(properties, rounds, |state, _| pcg::Pcg32OneSeqOnceInsecure::with_state(state as u32))
}

#[test]
fn test_pcg64_oneseq_once_insecure() {
    let properties = &RngProperties {
        period_pow2: 64,
        streams_pow2: 0,
        size_of_rng: 8,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "6364136223846793005 1442695040888963407 10915315373440060052",
            numbers: &[0x27a53829edf003a9,0xdf28458e5c04c31c,0x2756dc550bc36037,0xa10325553eb09ee9,0x40a0fccb8d9df09f,0x5c2047cfefb5e9ca],
            again: None,
            coins: "TTHHHTHHTTHHHTHTTHHTHHTTHHHTTHTTTTHHHTTHTHHHHTHTTTTTTHHHTTTHTTTTT",
            rolls: &[6,4,3,6,3,4,6,6,2,3,5,6,1,4,3,6,4,1,5,2,3,4,2,5,4,5,3,6,6,4,4,2,2],
            rolls_used: 33,
            cards: "3d Ah 7d Kh Td Th 8h 5d Ac 7c Ts As 7s 2s 2c 4h 8s Kc 3c 5h 6s 9c
                    4s Js Kd Qc 3h 2d Jh 6d 7h 3s 9d Tc 5s 9s 9h 4d Ks Jd Qd Ad Qs Jc
                    8d 4c 6c Qh 8c 5c 6h 2h",
        },
        Round {
            dump: "6364136223846793005 1442695040888963407 5709138325044364821",
            numbers: &[0xbf1a2718f46a9ab3,0x400c010fa404b25f,0x3e6dcc00e4e9238d,0x2ef92088e248ee1f,0xc6ba2d56b70972a8,0x7e11c48b5ee4b511],
            again: None,
            coins: "HHTHTHTTHTTHTTTTHTTHHHTHHTHTHHHHTHHTTTTTHTHTTHHTTTHTTHTHTTHTTHTHT",
            rolls: &[5,1,2,3,6,1,5,2,3,2,2,6,4,6,5,6,1,5,4,3,1,1,5,3,4,5,5,2,3,4,2,3,5],
            rolls_used: 33,
            cards: "9d Qs 4c 3s 7h 8d 3h Qd Jd 6s Ad Jh 5d Js 5s 8c 4h 5c 7s 8h Kc 9h
                    Ac 9c 3c 2s Tc 7d Ah 6d Jc Ks 2d 2c 9s Th 8s Kh Ts 4d 6h Td Qh Kd
                    3d 6c 2h 5h 7c As Qc 4s",
        },
        Round {
            dump: "6364136223846793005 1442695040888963407 13107181355803608426",
            numbers: &[0xa23134099c2db545,0xfd4094498a894a69,0x1199f424f91baa00,0x6c1c7a2896530d0b,0x1b9ca95e37253136,0xc05c20630e73d51f],
            again: None,
            coins: "HHTTHTTTTTTHHHTHHHTHHTTTHTTHHHHHTHTTHHHTHTTTHHHHTTTTHHTTHTTHHHHTT",
            rolls: &[5,3,5,1,1,5,6,1,6,4,5,4,5,3,6,1,6,4,6,3,2,4,3,3,2,5,4,4,6,3,6,4,1],
            rolls_used: 33,
            cards: "Ac 7h Ts Kc 9d 7s Js 2h Th 9s 2s 6h 3h Jc 3s 2c 5h 5d Qd 8h 3d 5s
                    6c Qh 7c 8d 7d Td As Ah Ad 6s Ks 4c Jd 9c 3c 4d Jh 9h Qs Qc 2d Kh
                    5c 8c 8s 6d 4h 4s Tc Kd",
        },
        Round {
            dump: "6364136223846793005 1442695040888963407 14730055255668324387",
            numbers: &[0x61748befe0c9e052,0x9261b659354e4790,0xf5760c6a80d81414,0xe9f8cd09b4a145b6,0x63cea77be983a1c2,0xff19b1c85857ee53],
            again: None,
            coins: "THTHHHTHHHTTHTHTTTHHTHTTHTTHHHTHTHTHHHTTTHHHHHHHHTHTHHHHHTTTTTTTT",
            rolls: &[6,6,1,3,3,2,6,5,5,2,3,4,5,3,3,6,6,5,6,1,4,3,5,3,1,2,4,2,2,2,6,6,1],
            rolls_used: 33,
            cards: "Jd 6h 5d 8s 3c Qd 2c 9s Ks 4h Kd 3h 6s 5c Jh Tc 4c Kh 2s 7d 8d Qh
                    As 9c Th 6c 9h 2h Qs 8c 7h 4s Js Ad 3d Jc Ac 7c 5h 2d Qc 6d Ah 7s
                    Ts 3s 5s 8h Kc Td 9d 4d",
        },
        Round {
            dump: "6364136223846793005 1442695040888963407 13134172073006285072",
            numbers: &[0x36288c2f6a1fcaba,0x9761b19d77017828,0x4192af9e0d374ee5,0x3d3134763fbd6bbf,0x1fb46e26bd261bd6,0x2c22c75b821893ac],
            again: None,
            coins: "HHHHHHTTHTHHTTHHHTTTHHHTHHTHHHHHHTTTHTTTHTTTTTTHTTTHTTHTTHTTHHHHH",
            rolls: &[6,4,3,4,5,5,2,3,1,6,6,6,6,4,4,4,1,2,2,5,4,1,5,2,3,4,1,4,1,5,1,3,5],
            rolls_used: 33,
            cards: "9s Th 9d 8c Jc Td 7c 3c 7h 3h 7d 3s 8h Qs 6c Qd Ah Ad 4d 5h Jd Kh
                    8d Qc Ts 6s 9c 5s As Ac 4s 6h 3d Ks 4h 5d 2h 5c 2s 2c Tc Jh 9h Js
                    4c 6d 7s 2d Kc 8s Kd Qh",
        },
    ];

    test_pcg(properties, rounds, |state, _| pcg::Pcg64OneSeqOnceInsecure::with_state(state as u64))
}
