#![cfg(feature = "serde")]
extern crate pcg;
extern crate serde;
extern crate serde_json;

use pcg::{Pcg32, PcgGenerator};

#[test]
fn test_serialize() {
    let mut rng = Pcg32::new();
    rng.advance(11 as u64);

    let rng_json = serde_json::to_string(&rng).unwrap();
    println!("rng_json: {}", rng_json);
    let mut rng_des: Pcg32 = serde_json::from_str(&rng_json).unwrap();

    for _ in 0..100 {
        let exp = rng.next();
        let got = rng_des.next();
        assert_eq!(exp, got);
    }
}
