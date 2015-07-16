extern crate num;
extern crate pcg;

use std::num::Wrapping;
use std::ops::{Add, BitAnd, Mul, Rem, Shl, Shr, Sub};
use self::num::{Bounded, FromPrimitive, One, ToPrimitive};
use self::pcg::{Engine, PcgResult, PcgState, PcgOutput, PcgPhase, PcgStream, PcgMultiplier};

pub fn shuffle<Result, State, Output, Phase, Stream, Multiplier>(
    slice: &mut [Result],
    rng: &mut Engine<Result, State, Output, Phase, Stream, Multiplier>
)
where
    Result: PcgResult<State> + Bounded + Clone + FromPrimitive + Ord + ToPrimitive
        + One + Add<Output=Result> + Sub<Output=Result> + Rem<Output=Result>,
    State: PcgState,
    Output: PcgOutput<Result, State>,
    Phase: PcgPhase,
    Stream: PcgStream<State>,
    Multiplier: PcgMultiplier<State>,
    Wrapping<Result>: Add<Output=Wrapping<Result>> + Sub<Output=Wrapping<Result>>,
    Wrapping<State>: Add<Output=Wrapping<State>>
        + BitAnd<Output=Wrapping<State>>
        + Mul<Output=Wrapping<State>>
        + Shl<usize, Output=Wrapping<State>>
        + Shr<usize, Output=Wrapping<State>>,
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