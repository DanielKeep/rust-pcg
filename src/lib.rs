/*!
# Todo

- Split up PcgResult and PcgState into things that are *absolutely* needed, and things that can be in separate traits.
*/
extern crate num;
extern crate rand;
extern crate rustc_serialize;

use std::fmt::Debug;
use std::marker::PhantomData;
use std::num::Wrapping;
use std::ops::{BitOr, Shr};
use num::{One, Zero};
use bounds::{DistanceToState, NextBoundedResult, WrappingState};

#[macro_use] mod macros;
pub mod bounds;
mod engine;
pub mod engines;
pub mod iter;
pub mod output;
pub mod stream;

pub use engine::Engine;

#[doc(inline)] pub use engines::SetSeqXshRr_64_32 as Pcg32;
#[doc(inline)] pub use engines::OneSeqXshRr_64_32 as Pcg32OneSeq;
#[doc(inline)] pub use engines::UniqueXshRr_64_32 as Pcg32Unique;
#[doc(inline)] pub use engines::McgXshRs_64_32 as Pcg32Fast;

#[doc(inline)] pub use engines::SetSeqRxsMXs_8_8 as Pcg8OnceInsecure;
#[doc(inline)] pub use engines::SetSeqRxsMXs_16_16 as Pcg16OnceInsecure;
#[doc(inline)] pub use engines::SetSeqRxsMXs_32_32 as Pcg32OnceInsecure;
#[doc(inline)] pub use engines::SetSeqRxsMXs_64_64 as Pcg64OnceInsecure;

pub trait PcgGenerator {
    type Result: PcgResult<Self::State>;
    type State: PcgState;
    type Output: PcgOutput<Self::Result, Self::State>;
    type Phase: PcgPhase;
    type Stream: PcgStream<Self::State>;
    type Multiplier: PcgMultiplier<Self::State>;

    fn advance(&mut self, delta: Self::State);

    fn backstep(&mut self, delta: Self::State);

    fn discard(&mut self, delta: Self::State);

    fn distance_to(&self, other: &Self) -> Option<Self::State>
    where Self::State: DistanceToState;

    fn iter(self) -> iter::IntoIter<Self>
    where Self: Sized {
        iter::IntoIter::new(self)
    }

    fn iter_bounded(self, upper_bound: Self::Result) -> iter::IntoIterBounded<Self>
    where
        Self: Sized,
        Self::Result: NextBoundedResult,
    {
        iter::IntoIterBounded::new(self, upper_bound)
    }

    fn iter_mut(&mut self) -> iter::IntoIterMut<Self>
    where Self: Sized {
        iter::IntoIterMut::new(self)
    }

    fn iter_mut_bounded(&mut self, upper_bound: Self::Result) -> iter::IntoIterMutBounded<Self>
    where
        Self: Sized,
        Self::Result: NextBoundedResult,
    {
        iter::IntoIterMutBounded::new(self, upper_bound)
    }

    fn next(&mut self) -> Self::Result;

    fn next_bounded(&mut self, upper_bound: Self::Result) -> Self::Result
    where Self::Result: NextBoundedResult;

    fn period_pow2() -> usize;

    fn streams_pow2() -> usize;

    #[doc(hidden)]
    fn dump_internals(&self) -> String
    where Self::State: Debug;
}

pub trait PcgResult<State>
where State: PcgState {
    fn from_state(state: State) -> Self;
    fn rotate_right(self, n: u32) -> Self;
}

pub trait PcgState: Sized + Clone + Eq + Ord + BitOr<Output=Self> + One + Shr<usize, Output=Self> + Zero {
    type Wrapping: WrappingState<State=Self>;

    fn bit_0() -> Self;
    fn bit_3() -> Self;
    fn into_usize(self) -> usize;
    fn is_odd(&self) -> bool;
    fn is_mcg_wrapped(&self) -> bool;
    fn negate(self) -> Self;
    fn wrapped(self) -> Self;
    fn wrapping(self) -> Self::Wrapping;
}

pub trait PcgOutput<Result, State>
where
    Result: PcgResult<State>,
    State: PcgState,
{
    fn output(State) -> Result;
}

pub trait PcgPhase {
    fn output_previous() -> bool;
}

pub trait PcgStream<State>
where State: PcgState {
    fn can_specify_stream() -> bool;
    fn is_mcg() -> bool;
    fn streams_pow2() -> usize;

    fn increment(&self) -> State;
}

pub trait PcgStatefulStream<State>: PcgStream<State>
where State: PcgState {
    fn from_stream_state(State) -> Self;
}

pub trait PcgMultiplier<State> {
    fn multiplier() -> State;
}

pub trait PcgIncrement<State> {
    fn increment() -> State;
}

pub trait McgMultiplier {
    fn multiplier() -> Self;
}

pcg_impl_result! { u8:  u8, u16, u32, u64 }
pcg_impl_result! { u16: u16, u32, u64 }
pcg_impl_result! { u32: u32, u64 }
pcg_impl_result! { u64: u64 }

pcg_impl_state! { u8 }
pcg_impl_state! { u16 }
pcg_impl_state! { u32 }
pcg_impl_state! { u64 }

#[derive(Clone, Debug)]
pub enum PcgDefault {}

pcg_define_pcg_constants! { PcgDefault: u8  = 141, 77 }
pcg_define_pcg_constants! { PcgDefault: u16 = 12829, 47989 }
pcg_define_pcg_constants! { PcgDefault: u32 = 747796405, 2891336453 }
pcg_define_pcg_constants! { PcgDefault: u64 = 6364136223846793005, 1442695040888963407 }

pcg_define_mcg_constant! { u8  = 217 }
pcg_define_mcg_constant! { u16 = 62169 }
pcg_define_mcg_constant! { u32 = 277803737 }
pcg_define_mcg_constant! { u64 = 12605985483714917081 }

#[derive(Clone, Debug)]
pub enum OutputPrevious {}
impl PcgPhase for OutputPrevious { #[inline] fn output_previous() -> bool { true } }

#[derive(Clone, Debug)]
pub enum OutputNext {}
impl PcgPhase for OutputNext { #[inline] fn output_previous() -> bool { false } }

#[derive(Clone, Debug)]
pub struct OutputPreviousIfSmall<T>(PhantomData<T>);
impl<T> PcgPhase for OutputPreviousIfSmall<T> {
    #[inline] fn output_previous() -> bool { ::std::mem::size_of::<T>() <= 8 }
}
