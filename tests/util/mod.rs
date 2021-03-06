extern crate num_traits;
extern crate pcg;

use std::fmt::Debug;
use std::mem::size_of;
use std::ops::{Add, Range};
use self::num_traits::{FromPrimitive, One, ToPrimitive};
use self::pcg::{PcgGenerator, PcgStream};
use self::pcg::bounds::{DistanceToState, NextBoundedResult};

pub struct RngProperties {
    pub period_pow2: usize,
    pub streams_pow2: usize,
    pub size_of_rng: usize,
    pub is_mcg: bool,
}

#[derive(Debug)]
pub struct Round<'a, R: 'a> {
    pub dump: &'static str,
    pub numbers: &'a [R],
    pub again: Option<&'a [R]>,
    pub coins: &'static str,
    pub rolls: &'a [u32],
    pub rolls_used: usize,
    pub cards: &'static str,
}

const NUM_CARDS: usize = 52;
const CARD_NUMBERS: &'static [&'static str] = &[
    "A", "2", "3", "4", "5", "6", "7",
    "8", "9", "T", "J", "Q", "K"
];
const CARD_SUITS: &'static [&'static str] = &["h", "c", "d", "s"];

pub fn test_pcg<Rng, Fn>(properties: &RngProperties, rounds: &[Round<Rng::Result>], create_rng: Fn)
where
    Rng: PcgGenerator + Clone + Debug,
    Fn: FnOnce(usize, usize) -> Rng,
    Rng::Result: Copy + Debug + Eq + FromPrimitive + ToPrimitive
        + One + NextBoundedResult,
    Range<Rng::Result>: Iterator<Item=Rng::Result>,
    Rng::State: Copy + Debug + FromPrimitive + ToPrimitive + DistanceToState,
    for<'a> &'a Rng::Result: Add<Output=Rng::Result>,
{
    let mut rng = create_rng(42, 54);
    println!("rng: {:#?}", rng);
    println!(".. internals: {:?}", rng.dump_internals());
    println!(".. is_mcg: {}", Rng::Stream::is_mcg());

    assert_eq!(Rng::period_pow2(), properties.period_pow2);
    assert_eq!(Rng::streams_pow2(), properties.streams_pow2);
    assert_eq!(size_of::<Rng>(), properties.size_of_rng);
    assert_eq!(Rng::Stream::is_mcg(), properties.is_mcg);

    let result_0 = FromPrimitive::from_usize(0).unwrap();
    let result_2 = FromPrimitive::from_usize(2).unwrap();
    let result_6 = FromPrimitive::from_usize(6).unwrap();
    let result_cards = FromPrimitive::from_usize(NUM_CARDS).unwrap();
    let state_6 = FromPrimitive::from_usize(6).unwrap();

    for (round_i, round) in rounds.iter().enumerate() {
        println!("round {}", round_i);
        println!(".. {:?}", round);

        // Check state of rng.
        assert_eq!(round.dump, rng.dump_internals());

        // Make some N-bit numbers.
        for &ex in round.numbers {
            assert_eq!(ex, rng.next());
        }

        // Again.
        rng.backstep(state_6);
        for &ex in round.again.unwrap_or(round.numbers) {
            assert_eq!(ex, rng.next());
        }

        // Toss some coins.
        for ex in round.coins.chars() {
            let got = if rng.next_bounded(result_2) == result_0 { 'T' } else { 'H' };
            assert_eq!(ex, got);
        }

        // Roll some dice.
        let rng_copy = rng.clone();
        for &ex in round.rolls {
            assert_eq!(ex, rng.next_bounded(result_6).to_u32().unwrap() + 1);
        }
        assert_eq!(
            round.rolls_used,
            rng_copy.distance_to(&rng).expect("compare rngs for rolls used").to_usize().unwrap()
        );

        let mut cards = vec![result_0; NUM_CARDS];
        for (c, v) in cards.iter_mut().zip(result_0..result_cards) { *c = v }
        shuffle(&mut cards, &mut rng);

        for (ex, &got_val) in round.cards.split_whitespace().zip(cards.iter()) {
            let got = card_name(got_val.to_usize().unwrap());
            assert_eq!(ex, &*got);
        }
    }
}

fn card_name(v: usize) -> String {
    let suits = CARD_SUITS.len();
    format!("{}{}", CARD_NUMBERS[v / suits], CARD_SUITS[v % suits])
}

fn shuffle<Rng>(slice: &mut [Rng::Result], rng: &mut Rng)
where
    Rng: PcgGenerator,
    Rng::Result: NextBoundedResult + FromPrimitive + ToPrimitive,
{
    let mut to = slice.len();
    while slice[..to].len() > 1 {
        let bound = FromPrimitive::from_usize(slice[..to].len()).unwrap();
        let chosen = rng.next_bounded(bound).to_usize().unwrap();
        to -= 1;
        slice.swap(chosen, to);
    }
}
