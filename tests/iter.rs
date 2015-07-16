extern crate pcg;

use pcg::{Pcg32, PcgGenerator};

#[test]
fn test_iter() {
    let rng = Pcg32::with_stream(42, 54);
    let exps = &[0xa15c02b7,0x7b47f409,0xba1d3330,0x83d2f293,0xbfa4784b,0xcbed606e];
    for (&exp, got) in exps.iter().zip(rng) {
        assert_eq!(exp, got);
    }
}

#[test]
fn test_iter_bounded() {
    let rng = Pcg32::with_stream(42, 54);
    let exps = &[50, 49, 52, 38, 1, 16];
    for (&exp, got) in exps.iter().zip(rng.iter_bounded(67)) {
        assert_eq!(exp, got);
    }
}

#[test]
fn test_iter_mut() {
    let mut rng = Pcg32::with_stream(42, 54);
    let exps = &[0xa15c02b7,0x7b47f409,0xba1d3330,0x83d2f293,0xbfa4784b,0xcbed606e];
    for (&exp, got) in exps.iter().zip(&mut rng) {
        assert_eq!(exp, got);
    }
}

#[test]
fn test_iter_mut_bounded() {
    let mut rng = Pcg32::with_stream(42, 54);
    let exps = &[50, 49, 52, 38, 1, 16];
    for (&exp, got) in exps.iter().zip(rng.iter_mut_bounded(67)) {
        assert_eq!(exp, got);
    }
}
