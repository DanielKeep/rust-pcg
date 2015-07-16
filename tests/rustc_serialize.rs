extern crate pcg;
extern crate rustc_serialize as rcs;

use pcg::{Pcg32, PcgGenerator};

#[test]
fn test_serialize() {
    let mut rng = Pcg32::new();
    rng.advance(11 as u64);

    let rng_json = rcs::json::encode(&rng).unwrap();
    println!("rng_json: {}", rng_json);
    let mut rng_des: Pcg32 = rcs::json::decode(&rng_json).unwrap();

    for _ in 0..100 {
        let exp = rng.next();
        let got = rng_des.next();
        assert_eq!(exp, got);
    }
}
