extern crate pcg;
extern crate rand;

use rand::{Rng, SeedableRng};

#[test]
fn test_rand() {
    let mut rng = pcg::Pcg32::from_seed((42, 54));
    let exps: &[u32] = &[0xa15c02b7,0x7b47f409,0xba1d3330,0x83d2f293,0xbfa4784b,0xcbed606e];
    for (&exps, got) in exps.iter().zip(rng.gen_iter()) {
        assert_eq!(exps, got);
    }
}
