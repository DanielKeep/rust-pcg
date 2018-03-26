// #![feature(step_trait)] // Because we need Step for ranges.
// #![feature(zero_one)] // Because we need One for ranges.

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

#[test]
fn test_pcg16_once_insecure() {
    let properties = &RngProperties {
        period_pow2: 16,
        streams_pow2: 15,
        size_of_rng: 4,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "12829 109 36744",
            numbers: &[0x9bec,0x5957,0x960e,0xd08d,0x4e05,0xde00,0x03f7,0x1fa6,0xee22,0xf6fa],
            again: sca![u16: 0x4e05,0xde00,0x03f7,0x1fa6,0xee22,0xf6fa,0x4aad,0xed64,0x9b8a,0x8a77],
            coins: "TTTHTTHTTHHHTTHHTTTTHTHHTHHTHTTHHHTTHTTHTHHTTTHTHTTHTHTHHTHTTTHTH",
            rolls: &[1,4,1,3,6,4,4,4,3,1,6,4,3,6,3,5,2,6,5,2,4,4,2,4,1,2,1,4,4,6,6,2,6],
            rolls_used: 33,
            cards: "Js 4s Jd Ts 2h Qc 4c 9d Jh 3d 2d 8h 3c Jc Tc 9h Kh 4h 6h 4d As Qd \
                    5d 8s 7s 6s 2s Kd Kc 7d 6d 7c Ks 5s Ad Qh 3s Ah 9s 6c 7h Ac Td 9c \
                    3h Th 8d 2c Qs 5h 8c 5c",
        },
        Round {
            dump: "12829 109 39011",
            numbers: &[0x6d56,0x15fd,0xde82,0x6cef,0x283e,0x8b82,0x94b3,0xdb7a,0x6c9d,0xaeac],
            again: sca![u16: 0x283e,0x8b82,0x94b3,0xdb7a,0x6c9d,0xaeac,0x0522,0x1190,0xad62,0x9490],
            coins: "THHHHHTTTTTTTHTHTHHHTHTHHTTHTHHHHTHHTHTHHTTTHTTTTTHHTHTTTTTHHHTTT",
            rolls: &[3,4,5,5,1,1,4,6,3,4,3,4,3,4,1,6,3,5,1,6,3,4,5,5,5,4,5,6,2,6,5,4,6],
            rolls_used: 33,
            cards: "4c 8d 9d 5d 2c Kh 5h 6c Ks 7h 8c 8s 7d 5s Jh Ah Qs 8h Th 2s 5c Jc \
                    4d 6h 3c 9h Qh As 6s 3s Ac Kd Qc 7s Jd 9s 3d Ad Td Js Kc 6d Tc 3h \
                    4h 2h 4s 2d 9c Ts 7c Qd",
        },
        Round {
            dump: "12829 109 10730",
            numbers: &[0x7d5b,0xfd3f,0x64a4,0x1752,0x70b2,0x093f,0x9888,0xed16,0xa2d3,0x485b],
            again: sca![u16: 0x70b2,0x093f,0x9888,0xed16,0xa2d3,0x485b,0x89ac,0x0b44,0x1028,0xd78f],
            coins: "HHHTHTTTTHTTHTTTHHHHTTTTTTTTHTTTTTTHHHTHTTTHHTHTHHHHHTHHHHHTTTHHT",
            rolls: &[4,6,5,1,2,4,3,5,1,3,6,6,5,3,2,3,5,1,2,5,6,4,5,6,1,3,2,2,6,6,5,4,2],
            rolls_used: 33,
            cards: "4c 6c 5s 5h 3s Kd 6d Ac Jc As 5c 7d Qh 4d Ah 5d Qd 2d 9d Th 3c 8s \
                    9c Td 6h Ts Kc 7c Qc Tc 2c 8d 7s 7h Jh 2h Qs Js 8c 9s 9h 8h 2s Ks \
                    3d 3h 4h Kh Jd Ad 6s 4s",
        },
        Round {
            dump: "12829 109 43981",
            numbers: &[0xb560,0xc497,0xf27a,0x7b73,0x1c23,0x38f4,0xe221,0x10aa,0x030c,0x8ea3],
            again: sca![u16: 0x1c23,0x38f4,0xe221,0x10aa,0x030c,0x8ea3,0xf40f,0xc014,0x1fe2,0xa1c2],
            coins: "TTHHTTTTTTHTTHTHHTTTHTTHHHHHHTTHHTHHTHTHTTHTTHTTHHHHTTHHHTTTTTHHH",
            rolls: &[4,1,1,2,4,1,3,3,2,2,3,4,2,1,3,4,5,1,6,6,2,3,6,5,6,6,2,6,4,3,1,5,2],
            rolls_used: 33,
            cards: "2s Jd Kc Tc 9d Qs Js 8c 5h 3d 6c 8h 8d Td 4s 2c 2d Th Kd Ah 9s 5d \
                    7c 7h 4d Qh 7d 6d Ac Ad 7s Jc Jh 3s 4c Ts 2h 9c 8s 3c 3h 6h 4h Kh \
                    As 5s Qc 9h Qd 5c 6s Ks",
        },
        Round {
            dump: "12829 109 63612",
            numbers: &[0xd3e8,0xe4e6,0xa68f,0xe732,0x7e4e,0xbd98,0x537b,0x0be1,0x720b,0x2087],
            again: sca![u16: 0x7e4e,0xbd98,0x537b,0x0be1,0x720b,0x2087,0x5534,0x84ba,0x94b4,0x30f1],
            coins: "HTTHTTTTTHHTHHTTTTHTHTTTTHTTHHHTTTTHTHTTTHTHHTHHHTTTHHTTTTHHHTHTH",
            rolls: &[5,5,2,6,4,2,3,4,6,3,5,3,2,3,2,6,2,3,4,4,3,4,6,1,1,1,1,3,6,6,1,3,3],
            rolls_used: 33,
            cards: "3d Ah 6c Ts 3h Kh 4s 9h 6s As 5c 7c 8h 9s Kd 6d Th Qs 7h Ad Qh Js \
                    5h 9c Qd Jh 8s 7d 3c 9d 3s Ac 4h 2c 8c Td Tc 5s Qc Kc 4d 2h Jd 7s \
                    2s Ks 5d Jc 8d 4c 6h 2d",
        },
    ];

    test_pcg(properties, rounds, |state, stream| pcg::Pcg16OnceInsecure::with_stream(state as u16, stream as u16))
}

#[test]
fn test_pcg32_once_insecure() {
    let properties = &RngProperties {
        period_pow2: 32,
        streams_pow2: 31,
        size_of_rng: 8,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "747796405 109 1248107568",
            numbers: &[0xf84b622d,0xdc1e5bb4,0x74fb8ac1,0xb3bbf8de,0x9cf62074,0x2d2f5e33],
            again: None,
            coins: "HTHHHHTTTTTHHHHHTTTTTHHTTTHTTHTHTTHTTTTTHTTTHHTHTTHTTHTTTTHTTHHHT",
            rolls: &[6,1,5,3,2,4,4,6,5,1,1,5,4,2,6,4,6,5,1,6,5,2,5,4,6,2,6,3,5,1,6,4,3],
            rolls_used: 33,
            cards: "Kd 8h Td 9d As 4h 7s 5s 2s Js Qd 5h Ac 3d 8d 2d 3h 8s 7h Th 4d 9s \
                    Qc 3s Kc 6d 7c 9h 4c 8c Kh Jh Jc 4s 3c Ah Tc 6s 9c 7d 5c Ks 6c Qs \
                    Ad 6h Ts Jd 2h 2c 5d Qh",
        },
        Round {
            dump: "747796405 109 1883088859",
            numbers: &[0xd6fdef4c,0xb793e894,0x62d8db75,0x51c7462c,0x9bbee1c9,0x9c609fb5],
            again: None,
            coins: "TTTHTHTTTHTTHTTHHTTTHHTHTHTHTTTTHTTHHTHTHHHHHTHHHHTHHTTTHHHTHHHTT",
            rolls: &[1,6,3,5,2,6,4,1,3,6,2,3,1,1,2,3,2,5,6,2,2,1,6,6,3,3,1,1,1,6,6,2,4],
            rolls_used: 33,
            cards: "6c Qd 9s 3d 7c 7h Ts 2c 3h Kd 2s Td 6h 8d Jh 2d 2h Ah 5h Th 5c 9c \
                    8s Kh As Kc 5s 6d Js Jd 4h 7d 5d Tc 8c Jc 6s 4s 9d 3c 4c Ac Qh Qs \
                    7s 4d 9h Ad Ks 8h 3s Qc",
        },
        Round {
            dump: "747796405 109 2419046250",
            numbers: &[0x7e849685,0x0a1a7a41,0xcf53a482,0xcbc007c5,0x60e65898,0x9179fbd7],
            again: None,
            coins: "THHHHHHTTHHTHHTHHTHHTTHTHTTHHHTTHHHHHTHTHTHTTHTHTTHTHTHHTHTTHTTTH",
            rolls: &[6,1,4,4,3,4,6,3,2,5,3,5,2,2,6,6,3,4,6,4,6,1,3,2,2,3,2,2,3,6,2,1,4],
            rolls_used: 33,
            cards: "Jd 8d 6c Jc 2s 9h 4d Kd 5d Qc Ts 8c 5s 3h 5h Ks 6d 4s 7c 5c Kc Js \
                    6s Kh Qs 7d 2c Ah 9d 3c 4c 4h 7h 9c 3s 3d As Tc Ac Ad 8s 8h Th Jh \
                    2d 6h Qd Qh 9s 7s 2h Td",
        },
        Round {
            dump: "747796405 109 1044292621",
            numbers: &[0xcc53664c,0xe23c4863,0xa79bb6df,0x96f9b755,0x13a38786,0x34a8f727],
            again: None,
            coins: "TTTTHTHHTHHTTTHTHTHHHHTHHTTHHHHHHHTHTHHHTHTTTHTTHHHHHTHTTTHHHTHTT",
            rolls: &[4,1,5,6,2,3,1,5,4,2,4,4,5,2,1,5,2,6,6,5,2,6,2,1,2,5,3,1,4,6,5,3,3],
            rolls_used: 33,
            cards: "Ad 9h 9d 5c 5d 5s 2s Ac Qs 4c 8s 6d Qh Kd 9c Ts 3d 4d Td 6h 4s 2h \
                    Jd 8h 7h Qd Ks Tc 7c 6s 8d As Kh Th Jh 3c 9s Kc Jc 3s 2c 8c 2d Js \
                    4h Ah 3h 5h 7d Qc 7s 6c",
        },
        Round {
            dump: "747796405 109 862749492",
            numbers: &[0x34c5b8b1,0x818c3828,0x23842fe4,0xd64649b8,0x5d1b76c9,0x18819107],
            again: None,
            coins: "TTHHTHTTTHTHTTTTTHHTHTHTTTHHTTHTHTHHHHTTTHTTTTTTHHHTHTTTTHHTHTTTT",
            rolls: &[5,6,1,1,5,3,1,6,4,5,3,1,2,4,1,3,5,1,1,5,2,3,2,4,1,1,3,2,3,1,2,4,2],
            rolls_used: 33,
            cards: "3h 5h 7s 4h 3c 8h 2h Qc 8c 4d 6s 5d Jh Ad 6c 4c 7h Js 7d 6d 8s 9d \
                    2d Qs 3s Ts 2c 2s Ac 8d Th Kd 5s Kc 9c 7c 3d Td Jc As Tc Ks Qh Qd \
                    6h 9h 4s Jd 5c Ah 9s Kh",
        },
    ];

    test_pcg(properties, rounds, |state, stream| pcg::Pcg32OnceInsecure::with_stream(state as u32, stream as u32))
}

#[test]
fn test_pcg64_once_insecure() {
    let properties = &RngProperties {
        period_pow2: 64,
        streams_pow2: 63,
        size_of_rng: 16,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "6364136223846793005 109 1753877967969059832",
            numbers: &[0xe1cbc180b69606bb,0x6573bce7abaee684,0xc744f07442006076,
                       0x9e9f98ccbd60b8fc,0xde693821ee9629ae,0x263cc2cdc66ebc25],
            again: None,
            coins: "THHHHTHHHHHTHTTHHHHTHHHTTTHTTHHTHTHTHHHHHHTHHTTHHTHHTHHTHHTTTHTTT",
            rolls: &[3,1,1,6,1,3,1,1,3,1,1,4,1,5,3,1,2,6,1,1,6,1,4,6,5,2,6,2,2,3,6,3,5],
            rolls_used: 33,
            cards: "Qd Qc Th 5s 6h 5c 2c 3s Ts Jh 9s Kh 4h Td Ad Ks 6c Ah 7h 8s 6s 8d \
                    2d Jc 2h 4s 9d Kc Qs 4d 3c 2s Kd 4c Tc 9c 7d 9h 8c 5h As Qh 8h 6d \
                    7c Js 3h 5d Ac Jd 7s 3d",
        },
        Round {
            dump: "6364136223846793005 109 262717807517198251",
            numbers: &[0x6869deb01736aa1d,0x3976d7772b7283b6,0xf192d7e3c132c7d9,
                       0x41b1b929e310e3c5,0x6a48bd0efb5c699d,0x856a779cda69bfd0],
            again: None,
            coins: "HTTHTHHTTTTHHTTTTTTHHHHHHHHHTTTTTTHHTHTHTHHTHHHTHTHHTTTHHTHHTTHTT",
            rolls: &[1,4,5,2,2,1,2,1,3,5,3,1,2,4,3,3,2,1,4,3,1,5,3,3,2,5,6,4,6,1,2,1,1],
            rolls_used: 33,
            cards: "4c Kh 5h Ts 8h 7c Tc 8d 5d Jc Js Jd Qh 9s 3h 9d 8s 2s Ks 2d 6d 6c \
                    2c 4s 7h Ad 3c 7d Kd Qc Td 4d 7s 9c 8c 2h Kc 3d Jh Ac 4h 9h Qd Ah \
                    5c Qs Th 5s 6s As 6h 3s",
        },
        Round {
            dump: "6364136223846793005 109 8025279220029899418",
            numbers: &[0x3f3dc8d4299c7a86,0x3e076e65a0310764,0x714322cce3e68f09,
                       0xe88471f1e7176e5e,0x262c50673106006a,0xbcc19d6d0d1ca1c4],
            again: None,
            coins: "HHHHHHHHHHTHTHTTTTTHTHHTTTTHHHHHHTTHHHHHHHTHHHHHHTHTHTHHHTTTTTTTT",
            rolls: &[4,2,1,5,6,1,2,3,6,1,5,3,5,4,6,1,5,5,2,5,6,6,4,6,3,4,4,5,5,1,2,4,2],
            rolls_used: 33,
            cards: "9h Kd Ks 6d 7h 4d 4s Qs 6c 6h 9d 9c Jc 5s Th 3s 2c 7c Qd Qh Qc 2h \
                    4c 8s 9s 8d 8h 2d Kh 2s Ah Ac Ad 3d 7d 8c Jd Js 7s Jh 5c Ts 6s 3h \
                    4h 3c As Td 5h 5d Kc Tc",
        },
        Round {
            dump: "6364136223846793005 109 3719089583871696501",
            numbers: &[0x24fccd9eb54476bf,0x2c32833537b2ca5d,0xfc4de62db8baf980,
                       0x66208e6be0123658,0x9f4674d5c6e9485a,0x4e093c882196a5f3],
            again: None,
            coins: "HTTHTHHTHTHTHHHTHHHHHHHTTHHHHHTHHTTHTHTHTTHTTHHHHTTHHTTHHTTHHHHTT",
            rolls: &[3,4,1,4,4,5,6,4,1,5,5,1,2,4,4,4,3,6,2,2,1,2,6,5,2,6,2,6,4,4,5,2,1],
            rolls_used: 33,
            cards: "4c Kh 2c Qd 2s Jc 8s 7c 6s 5d 2h 6d 9s Ks 8d 3s Qc 4d 8h Td Ad 6h \
                    3d 7s As 5h 3c 7h Th 4h Js Ac 7d 2d 6c Qs 9h 3h 4s Qh 9d Jh Ts 5s \
                    Ah Jd 9c Kd Kc Tc 5c 8c",
        },
        Round {
            dump: "6364136223846793005 109 12803124291375102636",
            numbers: &[0x8205935a6e122e27,0x70e48fea82d8d2f6,0xa6114ae38986de05,
                       0x05cd66f6baa6aae4,0x8aa8873d87856c4f,0xf9970ffeda4657e2],
            again: None,
            coins: "HHHTTHTHHTHHHHTHHTHTHTTTHHHHHHTHTHHTTHHTHTTHTTHHHHTHTHHTHHHTTHHHT",
            rolls: &[3,5,5,6,3,4,6,4,5,5,2,1,2,5,2,1,5,5,5,5,6,1,5,5,3,5,2,3,1,6,5,1,4],
            rolls_used: 33,
            cards: "3c 6s 7d 4h 5h Qs 5c Js 8s 2c 6c Qd 4d 6d 6h Ks 3h Th Ts Ad 8c 2d \
                    9s 3s Tc 5d Kh Jh 7h 9h Qc Kd Jd 9c 2s 7s Jc 5s Td 8h 8d As 9d Qh \
                    3d 2h Ah Ac 4c Kc 7c 4s",
        },
    ];

    test_pcg(properties, rounds, |state, stream| pcg::Pcg64OnceInsecure::with_stream(state as u64, stream as u64))
}
