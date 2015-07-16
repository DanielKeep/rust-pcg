/*!
# Todo

- Split up PcgResult and PcgState into things that are *absolutely* needed, and things that can be in separate traits.
*/
extern crate num;

use std::marker::PhantomData;
use std::num::Wrapping;
use std::ops::{Add, BitAnd, BitOr, Mul, Shl, Shr};
use num::{One, Zero};

#[macro_use] mod macros;
mod engine;
pub mod engines;
mod output;
mod stream;

pub use engine::Engine;
pub use output::XshRs;
pub use stream::{NoStream, SingleStream, SpecificStream, UniqueStream};

#[doc(inline)] pub use engines::SetSeqXshRr_64_32 as Pcg32;

pub trait PcgEngineTypes {
    type Result;
    type State;
    type Output;
    type Phase;
    type Stream;
    type Multiplier;
}

pub trait PcgResult<State>
where State: PcgState {
    fn from_state(state: State) -> Self;
    fn rotate_right(self, n: u32) -> Self;
}

pub trait PcgState: Sized + Clone + Eq + Ord + BitOr<Output=Self> + One + Shr<usize, Output=Self> + Zero
where 
    Wrapping<Self>: Add<Output=Wrapping<Self>>
        + BitAnd<Output=Wrapping<Self>>
        + Mul<Output=Wrapping<Self>>
        + Shl<usize, Output=Wrapping<Self>>
        + Shr<usize, Output=Wrapping<Self>>,
{
    fn bit_0() -> Self;
    fn bit_3() -> Self;
    fn into_usize(self) -> usize;
    fn is_odd(&self) -> bool;
    fn is_mcg_wrapped(&self) -> bool;
    fn negate(self) -> Self;
    fn wrapped(self) -> Self;
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
    type State;

    fn from_stream_state(Self::State) -> Self;
}

pub trait PcgMultiplier<State> {
    fn multiplier() -> State;
}

pub trait PcgIncrement<State> {
    fn increment() -> State;
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

pcg_define_constants! { PcgDefault: u8  = 141, 77 }
pcg_define_constants! { PcgDefault: u16 = 12829, 47989 }
pcg_define_constants! { PcgDefault: u32 = 747796405, 2891336453 }
pcg_define_constants! { PcgDefault: u64 = 6364136223846793005, 1442695040888963407 }

#[derive(Clone, Debug)]
pub enum OutputPrevious {}
impl PcgPhase for OutputPrevious { fn output_previous() -> bool { true } }

#[derive(Clone, Debug)]
pub enum OutputNext {}
impl PcgPhase for OutputNext { fn output_previous() -> bool { false } }

#[derive(Clone, Debug)]
pub struct OutputPreviousIfSmall<T>(PhantomData<T>);
impl<T> PcgPhase for OutputPreviousIfSmall<T> {
    fn output_previous() -> bool { ::std::mem::size_of::<T>() <= 8 }
}
