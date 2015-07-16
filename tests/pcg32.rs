#![feature(step_trait)] // Because we need Step for ranges.
#![feature(zero_one)] // Because we need One for ranges.

extern crate pcg;
mod util;
use util::{RngProperties, Round, test_pcg};

#[test]
fn test_pcg32() {
    let properties = &RngProperties {
        period_pow2: 64,
        streams_pow2: 63,
        size_of_rng: 16,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "6364136223846793005 109 1753877967969059832",
            numbers: &[0xa15c02b7,0x7b47f409,0xba1d3330,0x83d2f293,0xbfa4784b,0xcbed606e],
            coins: "HHTTTHTHHHTHTTTHHHHHTTTHHHTHTHTHTTHTTTHHHHHHTTTTHHTTTTTHTTTTTTTHT",
            rolls: &[3,4,1,1,2,2,3,2,4,3,2,4,3,3,5,2,3,1,3,1,5,1,4,1,5,6,4,6,6,2,6,3,3],
            rolls_used: 33,
            cards: "Qd Ks 6d 3s 3d 4c 3h Td Kc 5c Jh Kd Jd As 4s 4h Ad Th Ac Jc 7s Qs \
                    2s 7h Kh 2d 6c Ah 4d Qh 9h 6s 5s 2c 9c Ts 8d 9s 3c 8c Js 5d 2h 6h \
                    7d 8s 9d 5h 8h Qc 7c Tc",
        },
        Round {
            dump: "6364136223846793005 109 262717807517198251",
            numbers: &[0x74ab93ad,0x1c1da000,0x494ff896,0x34462f2f,0xd308a3e5,0x0fa83bab],
            coins: "HHHHHHHHHHTHHHTHTHTHTHTTTTHHTTTHHTHHTHTTHHTTTHHHHHHTHTTHTHTTTTTTT",
            rolls: &[5,1,1,3,3,2,4,5,3,2,2,6,4,3,2,4,2,4,3,2,3,6,3,2,3,4,2,4,1,1,5,4,4],
            rolls_used: 33,
            cards: "7d 2s 7h Td 8s 3c 3d Js 2d Tc 4h Qs 5c 9c Th 2c Jc Qd 9d Qc 7s 3s \
                    5s 6h 4d Jh 4c Ac 4s 5h 5d Kc 8h 8d Jd 9s Ad 6s 6c Kd 2h 3h Kh Ts \
                    Qh 9h 6d As 7c Ks Ah 8c",
        },
        Round {
            dump: "6364136223846793005 109 8025279220029899418",
            numbers: &[0x39af5f9f,0x04196b18,0xc3c3eb28,0xc076c60c,0xc693e135,0xf8f63932],
            coins: "HTTHHTTTTTHTTHHHTHTTHHTTHTHHTHTHTTTTHHTTTHHTHHTTHTTHHHTHHHTHTTTHT",
            rolls: &[5,1,5,3,2,2,4,5,3,3,1,3,4,6,3,2,3,4,2,2,3,1,5,2,4,6,6,4,2,4,3,3,6],
            rolls_used: 33,
            cards: "Kd Jh Kc Qh 4d Qc 4h 9d 3c Kh Qs 8h 5c Jd 7d 8d 3h 7c 8s 3s 2h Ks \
                    9c 9h 2c 8c Ad 7s 4s 2s 5h 6s 4c Ah 7h 5s Ac 3d 5d Qd As Tc 6h 9s \
                    2d 6c 6d Td Jc Ts Th Js",
        },
        Round {
            dump: "6364136223846793005 109 3719089583871696501",
            numbers: &[0x55ce6851,0x97a7726d,0x17e10815,0x58007d43,0x962fb148,0xb9bb55bd],
            coins: "HHTHHTTTTHTHHHHHTTHHHTTTHHTHTHTHTHHTTHTHHHHHHTHHTHHTHHTTTTHHTHHTT",
            rolls: &[6,6,3,2,3,4,2,6,4,2,6,3,2,3,5,5,3,4,4,6,6,2,6,5,4,4,6,1,6,1,3,6,5],
            rolls_used: 33,
            cards: "Qd 8h 5d 8s 8d Ts 7h Th Qs Js 7s Kc 6h 5s 4d Ac Jd 7d 7c Td 2c 6s \
                    5h 6d 3s Kd 9s Jh Kh As Ah 9h 3c Qh 9c 2d Tc 9d 2s 3d Ks 4h Qc Ad \
                    Jc 8c 2h 3h 4s 4c 5c 6c",
        },
        Round {
            dump: "6364136223846793005 109 12803124291375102636",
            numbers: &[0xfcef7cd6,0x1b488b5a,0xd0daf7ea,0x1d9a70f7,0x241a37cf,0x9a3857b7],
            coins: "HHHHTHHTTHTTHHHTTTHHTHTHTTTTHTTHTHTTTHHHTHTHTTHTTHTHHTHTHHHTHTHTT",
            rolls: &[5,4,1,2,6,1,3,1,5,6,3,6,2,1,4,4,5,2,1,5,6,5,6,4,4,4,5,2,6,4,3,5,6],
            rolls_used: 33,
            cards: "4d 9s Qc 9h As Qs 7s 4c Kd 6h 6s 2c 8c 5d 7h 5h Jc 3s 7c Jh Js Ks \
                    Tc Jd Kc Th 3h Ts Qh Ad Td 3c Ah 2d 3d 5c Ac 8s 5s 9c 2h 6c 6d Kh \
                    Qd 8d 7d 2s 8h 4h 9d 4s",
        },
    ];

    test_pcg(properties, rounds, |state, stream| pcg::Pcg32::with_stream(state as u64, stream as u64))
}

#[test]
fn test_pcg32_oneseq() {
    let properties = &RngProperties {
        period_pow2: 64,
        streams_pow2: 0,
        size_of_rng: 8,
        is_mcg: false,
    };
    let rounds = &[
        Round {
            dump: "6364136223846793005 1442695040888963407 10915315373440060052",
            numbers: &[0xc2f57bd6,0x6b07c4a9,0x72b7b29b,0x44215383,0xf5af5ead,0x68beb632],
            coins: "THTHHHTTHHTTHTTHTHHHTHTTTHTTHTTHTTTHHTTTTTHHTTTHTTHTHHTHHHTTHTTTH",
            rolls: &[4,1,3,3,6,6,5,1,3,4,4,3,2,2,5,4,1,3,3,3,1,4,6,4,6,6,1,6,1,2,3,6,6],
            rolls_used: 33,
            cards: "2d 5c 3h 6d Js 9c 4h Ts Qs 5d Ks 5h Ad Ac Qh Th Jd Kc Tc 7s Ah Kd \
                    7h 3c 4d 8s 2c 3d Kh 8h Jc 6h 4c 8d Qc 7c Td 2s 3s 4s 7d Qd Jh As \
                    6c 8c 5s 2h 6s 9d 9s 9h",
        },
        Round {
            dump: "6364136223846793005 1442695040888963407 5709138325044364821",
            numbers: &[0x0573afcc,0x2cab16db,0x6af6f55a,0xe916bec2,0x1ca9b4a4,0xbb2778eb],
            coins: "THHHTHTTTHHHTTTTTTHTTHTHTHHHTHHHTHTHTTHTTTTTHTHHTHHTTHHHHHTTTHTTH",
            rolls: &[1,5,3,3,5,1,5,6,5,6,6,3,5,5,6,6,2,6,4,1,5,6,3,6,5,5,1,3,2,4,5,1,1],
            rolls_used: 33,
            cards: "9c Ad 5d 7d Ah 8c Th Kd 5c Js 7c Kc Kh 6c Ks Tc Td 3d 7h 2d 5s 9s \
                    3h As 9d 8h 4s 6h Ts 2c Jh 3c 8s 4h 5h 6s Jd 8d 3s 6d 7s 4d Ac Qc \
                    4c 2h Qh 9h Qd 2s Qs Jc",
        },
        Round {
            dump: "6364136223846793005 1442695040888963407 13107181355803608426",
            numbers: &[0x114306f3,0xb9bf0d91,0x1aed8e5e,0x587de8b7,0x7477c8bd,0xd853ec9d],
            coins: "HTHHTHHHHTHTHTTHTHTHHTHTTHHHTTTTHHTTTTTTHTHTTTHTHTTTHTHHHHTTTTTTT",
            rolls: &[1,5,4,2,1,4,6,3,2,1,6,3,6,4,3,1,4,4,2,5,5,3,3,2,6,1,6,3,2,6,5,6,3],
            rolls_used: 33,
            cards: "Ah 8d Ad Jd 2d 3h Jh 7c Kc Ks 3d As 4s 3s 8h Qc 7d Td 6c 8c 4d 5c \
                    9d Qh Js Ac Kd 5s 6d Ts 9h 9s 9c 2c 5h 3c 5d Th 4c 6s 7s Qd 7h 2h \
                    Tc 6h 4h 8s Qs Jc Kh 2s",
        },
        Round {
            dump: "6364136223846793005 1442695040888963407 14730055255668324387",
            numbers: &[0xb982cd46,0x01cc6f94,0x0ad658ae,0xf6c6c97e,0xd1b772dd,0x0098599e],
            coins: "HTTHTTHHHHTHTHHHTTHTHTHTTTHTHTHHTHTHTTTTHHTTHHHTHTTHHTTTHHHTTHHHH",
            rolls: &[4,4,5,4,2,1,4,2,2,5,2,5,6,6,2,1,6,6,2,6,6,3,6,2,1,4,1,1,1,1,5,1,5],
            rolls_used: 33,
            cards: "6s Td 3h Js 7h Jh Ac Kh Th 4h 3c 6d Qs Ah 8h Kc Tc 2h 8c 2c Jd 2s \
                    Qh 4d 3d Ks 7s 9d 5d 2d 5s 5h Jc 3s 9s Qd Qc 7d 6h As 8s 4s 4c 8d \
                    9c 6c 5c Ad 7c 9h Kd Ts",
        },
        Round {
            dump: "6364136223846793005 1442695040888963407 13134172073006285072",
            numbers: &[0xef3c7322,0xa1ff2188,0x3f564b42,0x91c90425,0x17711b95,0xf43aa1f7],
            coins: "HTTHHHTTHTTTHTHHTHTHTHHTHHTTTHTTHTHHTHTTTTTHTHTTHHHHTHTHTHHTHHTHT",
            rolls: &[4,1,6,3,3,2,5,6,3,2,6,5,3,1,5,5,4,6,4,4,2,5,5,4,1,5,2,4,5,5,5,3,5],
            rolls_used: 33,
            cards: "6c 8d 4d Jc 9d As 9s 3c 9c Th Ks Qs 4c Js Ah Qc Ac Kd Td Qd Kh Kc \
                    Tc Jd 6s 5h 8c 8s Ad 5s 4s Ts 3h 3s 7h 7d 8h 2c 2d 5c 6h 2h 3d 7c \
                    9h 7s 4h 2s Jh 6d Qh 5d",
        },
    ];

    test_pcg(properties, rounds, |state, _| pcg::Pcg32OneSeq::with_state(state as u64))
}

#[test]
fn test_pcg32_fast() {
    let properties = &RngProperties {
        period_pow2: 62,
        streams_pow2: 0,
        size_of_rng: 8,
        is_mcg: true,
    };
    let rounds = &[
        Round {
            dump: "6364136223846793005 0 43",
            numbers: &[0x00000000,0x5c400ccc,0x03a8459e,0x9bdb59c5,0xf1c9dcf5,0xaac0af3b],
            coins: "HTHHTHHTTHTHHHTTTTTHHTTTTHTHTHTTHTTHHHHTHHTTTHHTTTTHTTTTHHHTHTHHT",
            rolls: &[1,3,1,4,3,1,4,3,5,1,5,1,6,3,4,6,2,3,3,5,5,2,5,6,5,3,2,4,2,3,1,1,3],
            rolls_used: 33,
            cards: "2s 8d 7s 9h Ad Qc Jh 5s 3d 3s 7c Qs Kh Ts 3h Ac 5c 9d 6s 4c 8h 2d \
                    Kc 6c 9c 8c 6d 5d As 2h 7h Th Td Js Jd 3c 5h 7d Ah Qd 4d 2c Ks 4s \
                    9s Jc 6h 8s 4h Kd Tc Qh",
        },
        Round {
            dump: "6364136223846793005 0 3588989041792787143",
            numbers: &[0x9d4c8720,0x888c050e,0x20a18d88,0x9af6f5ac,0xe9e08d16,0x30dc8422],
            coins: "HTHHHTTTHHHTTHHTTTTTHHHHHHHHHTTHHHHHHTTHHTTTHHTTTHHHTTTHHTHHTHTHH",
            rolls: &[1,6,5,2,2,4,3,6,3,6,4,2,4,3,4,6,6,3,4,4,5,6,4,4,5,4,6,2,1,2,6,2,3],
            rolls_used: 33,
            cards: "8c 3c 5h 2d 4s 7d 4h 5c Kh 5s 3s Kd 9h 6h Th 7h 8d 8h Qs Ks Tc Qh \
                    Qd Ac 9d Jd Td 6d Ts 8s Js Jh 6c 7c 6s 2c Kc 2h 4d 9c 3h Ah Qc 4c \
                    5d 7s Jc 9s As 2s Ad 3d",
        },
        Round {
            dump: "6364136223846793005 0 375581696086973075",
            numbers: &[0xd9561348,0xc5f085ff,0x55b15d21,0xb00d4c13,0x1ad51817,0xb1687c92],
            coins: "THTHHHTTTHTHHTHTHTTTTHHTTTTHHHTHTTTTTHHTHTHHHHHTTTHTTTTHTTHTTHTHT",
            rolls: &[5,3,4,1,4,1,5,5,3,1,2,5,4,1,1,6,2,2,1,2,2,2,4,6,3,6,2,4,6,5,2,5,1],
            rolls_used: 33,
            cards: "7d 5h As 8c Qh 6d Ts Qd Ah 6c 7c 9c 8d 9h 4s 4c 3d 4d Ad Ks Td 3s \
                    3c Kd 8h Kc Jd Kh Th Qs Jh 8s 2c 7s Jc 2h 6h 9d 2d 9s Ac 2s Qc 5s \
                    Js 5d Tc 5c 4h 3h 6s 7h",
        },
        Round {
            dump: "6364136223846793005 0 7737214110345585487",
            numbers: &[0xb00d4873,0x97e247d1,0x3aed3e74,0xa6f02f6a,0x007428ae,0x88fb2312],
            coins: "HTTHTHTTTHHTHHTTHHTTHTHHTTTHTHTHHHTHTHTTTHHHHTHHTHHHHTTHHHHTHHHHT",
            rolls: &[2,5,2,5,3,4,5,6,4,3,3,3,2,2,2,3,3,3,2,6,1,2,1,3,5,4,6,6,6,3,4,5,1],
            rolls_used: 33,
            cards: "4s Qc 2s 4h Kh 6d Ts 9s 2c Ks 5h Jc 2h Ac Qs Jd 8d 2d 7s 3c 8c 6c \
                    Kd Kc 7h 4c 9h 3h 5s Ah 8h 7c Td 3d 6h 5d Tc Qh 3s 4d 9c Ad Qd 9d \
                    8s Jh 7d 5c As Js Th 6s",
        },
        Round {
            dump: "6364136223846793005 0 9974383512159695803",
            numbers: &[0x9b08b727,0x4b6859af,0x06de6f08,0x628f4193,0x39397e2d,0x9e8304d1],
            coins: "THTHHTTTTHTTHHHTHTHTHTHHTTTHHHHTHHHTTTHHTHTHHTHTHTTTTTHHHHHHTTTHT",
            rolls: &[6,1,4,1,3,4,5,6,5,1,2,6,3,3,6,4,6,5,2,1,3,6,3,3,1,5,5,3,6,2,2,2,2],
            rolls_used: 33,
            cards: "4c 2h Qh 6d 6h 8h 5s Qd 6c 7d Ac 5h 8s Ad 4d 9s 2c Tc Js 3d Th Ah \
                    Jd 5d 3h 4s Ts Kc 6s 8d 8c 2d Kh 7h 9c Qc 3c Td As 7c Kd 2s 9d Ks \
                    Jh Jc 5c 4h 3s Qs 9h 7s",
        },
    ];

    test_pcg(properties, rounds, |state, _| pcg::Pcg32Fast::with_state(state as u64))
}
