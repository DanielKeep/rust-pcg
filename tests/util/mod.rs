extern crate num;
extern crate pcg;

use self::num::{FromPrimitive, ToPrimitive};
use self::pcg::PcgGenerator;
use self::pcg::bounds::NextBoundedResult;

pub fn shuffle<Rng>(slice: &mut [Rng::Result], rng: &mut Rng)
where
    Rng: PcgGenerator,
    Rng::Result: NextBoundedResult + FromPrimitive + ToPrimitive,
{
    println!("shuffle(..):");
    let mut to = slice.len();
    while slice[..to].len() > 1 {
        let bound = FromPrimitive::from_usize(slice[..to].len()).unwrap();
        let chosen = rng.next_bounded(bound).to_usize().unwrap();
        print!(" {:?}", chosen);
        to -= 1;
        slice.swap(chosen, to);
    }
    println!("");
}